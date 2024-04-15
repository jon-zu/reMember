use std::sync::{atomic::AtomicI64, Arc};

use crate::{
    entities::{equip_item, inventory_slot, item_stack, pet_item},
    model::inv::{
        CashInv, CashItemSlot, EquipInventory, EquippedInventory, InvSetHandler, InventorySet,
        NoopInvSetHandler, StackInv, StackItemSlot,
    },
};
use anyhow::anyhow;
use itertools::Itertools;
use num_enum::TryFromPrimitive;
use sea_orm::{
    sea_query::Expr, ActiveValue::NotSet, ColumnTrait, DeriveColumn, EntityTrait, EnumIter,
    QueryFilter, QuerySelect, Set,
};
use shroom_meta::{
    id::{
        item_id::{InventoryType, ItemGrade},
        CharacterId, ItemId,
    },
    item::{
        it::{Durability, EquipItem, ItemFlags, ItemInfo, PetItem, StackItem},
        EquipStat,
    },
    MetaService,
};
use shroom_proto95::shared::inventory::CharEquipSlot;
use shroom_srv::game::{
    inventory::{InvEventHandler, InvSlotIndex},
    stack_inv::StackInvEventHandler,
};

use super::{character::ItemStarterSet, DbConn};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CharacterEquippedItemIds {
    pub equipped: Vec<(CharEquipSlot, ItemId)>,
    pub masked: Vec<(CharEquipSlot, ItemId)>,
}

fn map_equip_to_active_model(item: &EquipItem) -> equip_item::ActiveModel {
    let stats = &item.stats;
    let lvl = item.level_info.as_ref();
    let id = item.db_id.map(Set).unwrap_or(NotSet);

    equip_item::ActiveModel {
        id,
        expires_at: Set(item.expiration),
        game_id: Set(item.game_id),
        item_id: Set(item.item_id.0 as i32),
        flags: Set(item.flags.bits() as i32),
        //TODO ioptional
        item_level: Set(lvl.map(|lvl| lvl.level as i32).unwrap_or_default()),
        //TODO Optional
        item_exp: Set(lvl.map(|lvl| lvl.exp as i32).unwrap_or_default()),
        hammer_slots: Set(item.hammers_used as i32),
        //TODO optional
        owner_tag: Set(item.owner.clone().unwrap_or_default()),
        level: Set(0),
        upgrades: Set(item.upgrades as i32),
        upgrade_slots: Set(item.upgrade_slots as i32),
        str: Set(stats[EquipStat::Str].0 as i32),
        dex: Set(stats[EquipStat::Dex].0 as i32),
        luk: Set(stats[EquipStat::Luk].0 as i32),
        int: Set(stats[EquipStat::Int].0 as i32),
        hp: Set(stats[EquipStat::Hp].0 as i32),
        mp: Set(stats[EquipStat::Mp].0 as i32),
        weapon_atk: Set(stats[EquipStat::Pad].0 as i32),
        weapon_def: Set(stats[EquipStat::Pdd].0 as i32),
        magic_atk: Set(stats[EquipStat::Mad].0 as i32),
        magic_def: Set(stats[EquipStat::Mdd].0 as i32),
        accuracy: Set(stats[EquipStat::Acc].0 as i32),
        avoid: Set(stats[EquipStat::Eva].0 as i32),
        speed: Set(stats[EquipStat::Speed].0 as i32),
        jump: Set(stats[EquipStat::Jump].0 as i32),
        craft: Set(stats[EquipStat::Craft].0 as i32),
        is_cash: Set(item.is_cash),
        level_ty: Set(lvl.map(|lvl| lvl.item_level_ty as i32).unwrap_or_default()),
        grade: Set(item.grade as i32),
        option1: Set(item.options.0[0].0 as i32),
        option2: Set(item.options.0[1].0 as i32),
        option3: Set(item.options.0[2].0 as i32),
        socket1: Set(item.sockets.0[0] as i32),
        socket2: Set(item.sockets.0[1] as i32),
        durability: Set(item.durability.into()),
        equipped_at: Set(item.equipped_at),
        stars: Set(item.stars as i32),
    }
}

fn map_stack_to_active_model(item: &StackItem) -> item_stack::ActiveModel {
    let id = item.db_id.map(Set).unwrap_or(NotSet);

    item_stack::ActiveModel {
        id,
        expires_at: Set(item.expiration),
        game_id: Set(item.game_id),
        item_id: Set(item.item_id.0 as i32),
        flags: Set(item.flags.bits() as i32),
        quantity: Set(item.quantity as i32),
        is_cash: Set(item.is_cash),
    }
}

fn map_pet_to_active_model(item: &PetItem) -> pet_item::ActiveModel {
    let id = item.db_id.map(Set).unwrap_or(NotSet);

    pet_item::ActiveModel {
        id,
        expires_at: Set(item.expiration),
        game_id: Set(item.game_id),
        item_id: Set(item.item_id.0 as i32),
        flags: Set(item.flags.bits() as i32),
        name: Set(item.name.clone()),
        level: Set(item.level as i32),
        tameness: Set(item.tameness as i32),
        fullness: Set(item.fullness as i32),
        skill: Set(item.skill as i32),
        remaining_life: Set(item.remaining_life as i32),
        active: Set(true),
        attr1: Set(item.attr1 as i32),
        attr2: Set(item.attr2 as i32),
        dead_at: Set(item.dead_at),
    }
}

#[derive(Debug)]
pub struct SharedItemSvc {
    meta: &'static MetaService,
    last_equip_game_id: AtomicI64,
    last_stack_game_id: AtomicI64,
    last_pet_game_id: AtomicI64,
}

impl SharedItemSvc {
    pub fn next_stack_game_id(&self) -> i64 {
        self.last_stack_game_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn new_stack_item(&self, id: ItemId, quantity: usize) -> Box<StackItem> {
        Box::new(StackItem {
            info: ItemInfo {
                db_id: None,
                item_id: id,
                is_cash: false,
                game_id: self.next_stack_game_id(),
                expiration: None,
                owner: None,
                flags: ItemFlags::empty(),
                last_update: 0,
            },
            quantity: quantity as u16,
        })
    }

    pub fn create_equip(&self, id: ItemId) -> anyhow::Result<EquipItem> {
        let eq_meta = self
            .meta
            .items()
            .get_equip(id)
            .ok_or_else(|| anyhow!("Invalid item: {id:?}"))?;
        // TODO: Assert item is an equip

        let mut rng = rand::thread_rng();
        //let mut stats = EquipBaseStats::from_equip_meta(eq_meta);

        let mut stats = eq_meta.stats.base.clone();
        stats.apply_chaos_scroll(&mut rng, -2..=2);

        Ok(EquipItem {
            info: ItemInfo::from_id(
                id,
                self
                    .last_equip_game_id
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                false,
            ),
            stats,
            upgrade_slots: eq_meta.upgrade_slots, //TODO fix that
            hammers_used: 0,
            level_info: None,
            upgrades: 0,
            sockets: Default::default(),
            options: Default::default(),
            equipped_at: None,
            grade: ItemGrade::Normal,
            durability: Durability::Infinite,
            stars: 0,
        })
    }
}

pub type DbItemId = i32;
pub type DbSlotId = i32;

#[derive(Debug)]
pub struct ItemService {
    db: DbConn,
    shared: Arc<SharedItemSvc>,
}

impl ItemService {
    pub async fn new(db: DbConn, meta: &'static MetaService) -> anyhow::Result<Self> {
        let max_equip_id = equip_item::Entity::find()
            .select_only()
            .column_as(Expr::col(equip_item::Column::GameId).max(), "max_id")
            .into_tuple::<Option<i64>>()
            .one(&db.0)
            .await?
            .flatten()
            .unwrap_or_default()
            + 1;

        let max_stack_id = item_stack::Entity::find()
            .select_only()
            .column_as(Expr::col(item_stack::Column::GameId).max(), "max_id")
            .into_tuple::<Option<i64>>()
            .one(&db.0)
            .await?
            .flatten()
            .unwrap_or_default()
            + 1;

        let max_pet_id = pet_item::Entity::find()
            .select_only()
            .column_as(Expr::col(pet_item::Column::GameId).max(), "max_id")
            .into_tuple::<Option<i64>>()
            .one(&db.0)
            .await?
            .flatten()
            .unwrap_or_default()
            + 1;

        Ok(Self {
            db,
            shared: Arc::new(SharedItemSvc {
                meta,
                last_equip_game_id: AtomicI64::new(max_equip_id),
                last_stack_game_id: AtomicI64::new(max_stack_id),
                last_pet_game_id: AtomicI64::new(max_pet_id),
            }),
        })
    }

    pub fn create_equip(&self, id: ItemId) -> anyhow::Result<EquipItem> {
        self.shared.create_equip(id)
    }

    pub fn create_stack(&self, id: ItemId, quantity: u16) -> anyhow::Result<StackItem> {
        /*let stack_meta = self
        .meta
        .get_item_data(id)
        .ok_or_else(|| anyhow!("Invalid stack item: {id:?}"))?;*/
        // TODO verify stack item

        Ok(StackItem {
            info: ItemInfo::from_id(
                id,
                self.shared
                    .last_stack_game_id
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                false,
            ),
            quantity,
        })
    }

    pub fn create_pet(&self, id: ItemId) -> anyhow::Result<PetItem> {
        /*let pet_meta = self
        .meta
        .get_item_data(id)
        .ok_or_else(|| anyhow!("Invalid pet item: {id:?}"))?;*/
        // TODO verify pet item

        Ok(PetItem {
            info: ItemInfo::from_id(
                id,
                self.shared
                    .last_pet_game_id
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                true,
            ),
            dead_at: None,
            name: "Pet Petson".to_string(),
            level: 0,
            tameness: 0,
            fullness: 100,
            attr1: 0,
            attr2: 0,
            remaining_life: 100,
            skill: 0,
        })
    }

    pub async fn save_equip(&self, item: &mut EquipItem) -> anyhow::Result<()> {
        if let Some(db_id) = item.db_id {
            if item.last_update > 0 {
                equip_item::Entity::update(map_equip_to_active_model(item))
                    .filter(equip_item::Column::Id.eq(db_id))
                    .exec(&self.db.0)
                    .await?;
            }
        } else {
            let id = equip_item::Entity::insert(map_equip_to_active_model(item))
                .exec(&self.db.0)
                .await?
                .last_insert_id;
            item.db_id = Some(id);
        }
        item.last_update = 0;

        Ok(())
    }

    pub async fn save_stack(&self, item: &mut StackItem) -> anyhow::Result<()> {
        if let Some(db_id) = item.db_id {
            if item.last_update > 0 {
                item_stack::Entity::update(map_stack_to_active_model(item))
                    .filter(item_stack::Column::Id.eq(db_id))
                    .exec(&self.db.0)
                    .await?;
            }
        } else {
            let id = item_stack::Entity::insert(map_stack_to_active_model(item))
                .exec(&self.db.0)
                .await?
                .last_insert_id;
            item.db_id = Some(id);
        }
        item.last_update = 0;

        Ok(())
    }

    pub async fn save_pet(&self, item: &mut PetItem) -> anyhow::Result<()> {
        if let Some(db_id) = item.db_id {
            if item.last_update > 0 {
                pet_item::Entity::update(map_pet_to_active_model(item))
                    .filter(pet_item::Column::Id.eq(db_id))
                    .exec(&self.db.0)
                    .await?;
            }
        } else {
            let id = pet_item::Entity::insert(map_pet_to_active_model(item))
                .exec(&self.db.0)
                .await?
                .last_insert_id;
            item.db_id = Some(id);
        }
        item.last_update = 0;

        Ok(())
    }

    pub async fn create_starter_set(
        &self,
        char_id: CharacterId,
        starter_set: ItemStarterSet,
    ) -> anyhow::Result<()> {
        let slots = [
            CharEquipSlot::Bottom,
            CharEquipSlot::Shoes,
            CharEquipSlot::Top,
            CharEquipSlot::Weapon,
        ];
        let items = [
            starter_set.bottom,
            starter_set.shoes,
            starter_set.top,
            starter_set.weapon,
        ]
        .iter()
        .map(|id| self.create_equip(*id))
        .collect::<anyhow::Result<Vec<_>>>()?;

        let mut inv = InventorySet::<NoopInvSetHandler>::with_default_slots(self.shared.clone());
        for (item, slot) in items.into_iter().zip(slots) {
            inv.equipped.set(slot.into(), item.into())?;
        }
        inv.etc
            .set(0, Box::new(self.create_stack(starter_set.guide, 1)?).into())?;

        self.save_inventory(&mut inv, char_id).await?;

        Ok(())
    }

    pub async fn clear_inventory(&self, char_id: i32) -> anyhow::Result<()> {
        inventory_slot::Entity::delete_many()
            .filter(inventory_slot::Column::CharId.eq(char_id))
            .exec(&self.db.0)
            .await?;

        Ok(())
    }

    async fn save_eq_inventory_type(
        &self,
        inv_type: InventoryType,
        char_id: CharacterId,
        inv: &mut EquipInventory,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }

        // Update items
        for item_slot in inv.items_mut() {
            let item = &mut item_slot.item;
            self.save_equip(item).await?;
        }

        let slots = inv
            .item_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(Some(item.item.db_id.unwrap())),
                char_id: Set(char_id.0 as i32),
                slot: Set(slot as u8 as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(None),
                pet_item_id: Set(None),
            });

        let slots = slots.collect_vec();
        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db.0)
            .await?;

        Ok(())
    }

    async fn save_eqd_inventory_type(
        &self,
        inv_type: InventoryType,
        char_id: CharacterId,
        inv: &mut EquippedInventory,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }

        // Update items
        for item_slot in inv.items_mut() {
            let item = &mut item_slot.0.item;
            self.save_equip(item).await?;
        }

        let slots = inv
            .item_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(Some(item.0.item.db_id.unwrap())),
                char_id: Set(char_id.0 as i32),
                slot: Set(slot.to_ix() as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(None),
                pet_item_id: Set(None),
            });

        let slots = slots.collect_vec();
        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db.0)
            .await?;

        Ok(())
    }

    async fn save_stack_inventory_type<
        T: InvEventHandler<Item = StackItemSlot> + StackInvEventHandler,
    >(
        &self,
        inv_type: InventoryType,
        char_id: CharacterId,
        inv: &mut StackInv<T>,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }
        // Update items
        // TODO optimize this + use transaction
        for item in inv.items_mut() {
            self.save_stack(item).await?;
        }

        let slots = inv
            .item_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(None),
                char_id: Set(char_id.0 as i32),
                slot: Set(slot as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(Some(item.db_id.unwrap())),
                pet_item_id: Set(None),
            });

        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db.0)
            .await?;

        Ok(())
    }

    async fn save_cash_inventory_type<
        T: InvEventHandler<Item = CashItemSlot> + StackInvEventHandler,
    >(
        &self,
        inv_type: InventoryType,
        char_id: CharacterId,
        inv: &mut CashInv<T>,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }
        // Update items
        // TODO optimize this + use transaction
        for item in inv.items_mut() {
            match item {
                CashItemSlot::Stack(stack) => self.save_stack(stack).await?,
                CashItemSlot::Pet(pet) => self.save_pet(pet).await?,
            }
        }

        let slots = inv
            .item_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(None),
                char_id: Set(char_id.0 as i32),
                slot: Set(slot as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(item.as_stack().map(|s| s.db_id.unwrap())),
                pet_item_id: Set(item.as_pet().map(|s| s.db_id.unwrap())),
            });

        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db.0)
            .await?;

        Ok(())
    }

    pub async fn save_inventory<T: InvSetHandler>(
        &self,
        invs: &mut InventorySet<T>,
        char_id: CharacterId,
    ) -> anyhow::Result<()> {
        inventory_slot::Entity::delete_many()
            .filter(inventory_slot::Column::CharId.eq(char_id.0 as i32))
            .exec(&self.db.0)
            .await?;

        self.save_eqd_inventory_type(InventoryType::Equipped, char_id, &mut invs.equipped)
            .await?;

        self.save_eqd_inventory_type(InventoryType::Special, char_id, &mut invs.masked_equipped)
            .await?;

        self.save_eq_inventory_type(InventoryType::Equip, char_id, &mut invs.equip)
            .await?;

        self.save_stack_inventory_type(InventoryType::Consume, char_id, &mut invs.consume)
            .await?;
        self.save_stack_inventory_type(InventoryType::Install, char_id, &mut invs.misc)
            .await?;
        self.save_stack_inventory_type(InventoryType::Etc, char_id, &mut invs.etc)
            .await?;
        self.save_cash_inventory_type(InventoryType::Cash, char_id, &mut invs.cash)
            .await?;
        Ok(())
    }

    pub async fn load_inventory_for_character(
        &self,
        char_id: CharacterId,
    ) -> anyhow::Result<InventorySet<NoopInvSetHandler>> {
        let equip_item_slots = inventory_slot::Entity::find()
            .filter(inventory_slot::Column::CharId.eq(char_id.0 as i32))
            .inner_join(equip_item::Entity)
            .select_also(equip_item::Entity)
            .all(&self.db.0)
            .await?;

        let item_stack_slots = inventory_slot::Entity::find()
            .filter(inventory_slot::Column::CharId.eq(char_id.0 as i32))
            .inner_join(item_stack::Entity)
            .select_also(item_stack::Entity)
            .all(&self.db.0)
            .await?;

        let pet_slots = inventory_slot::Entity::find()
            .filter(inventory_slot::Column::CharId.eq(char_id.0 as i32))
            .inner_join(pet_item::Entity)
            .select_also(pet_item::Entity)
            .all(&self.db.0)
            .await?;

        let mut inv = InventorySet::with_default_slots(self.shared.clone());

        // Load equips
        for (slot_info, equip_item) in equip_item_slots {
            let Some(equip_item) = equip_item else {
                anyhow::bail!("Invalid no equip item");
            };
            let inv_type = InventoryType::try_from_primitive(slot_info.inv_type as u8)?;
            match inv_type {
                InventoryType::Equipped => {
                    let slot = CharEquipSlot::try_from_primitive(slot_info.slot as u8)?;
                    let equip_item: EquipItem = equip_item.into();
                    inv.equipped.set(slot.into(), equip_item.into())?;
                }
                InventoryType::Special => {
                    let slot = CharEquipSlot::try_from_primitive(slot_info.slot as u8)?;
                    let equip_item: EquipItem = equip_item.into();
                    inv.masked_equipped.set(slot.into(), equip_item.into())?;
                }
                InventoryType::Equip => {
                    let slot = slot_info.slot as usize;
                    let equip_item: EquipItem = equip_item.into();
                    inv.equip.set(slot, equip_item.into())?
                }
                _ => anyhow::bail!(
                    "Inventory Item({} - {}) with invalid inventory type found: {inv_type:?}",
                    equip_item.id,
                    equip_item.item_id
                ),
            }
        }

        // Load slots
        for (slot_info, stack_item) in item_stack_slots {
            let Some(stack_item) = stack_item else {
                anyhow::bail!("Invalid no stack item");
            };
            let inv_type = InventoryType::try_from_primitive(slot_info.inv_type as u8)?;
            let slot = slot_info.slot as usize;

            inv.get_stack_inventory_mut(inv_type)?
                .set(slot, stack_item.into())?;
        }

        // Load pets
        for (slot_info, pet_item) in pet_slots {
            let Some(pet_item) = pet_item else {
                anyhow::bail!("Invalid no pet item");
            };
            let slot = slot_info.slot as usize;

            inv.get_cash_inventory_mut()
                .set(slot, CashItemSlot::Pet(pet_item.into()))?;
        }

        Ok(inv)
    }

    pub async fn load_equipped_items(
        &self,
        char_id: CharacterId,
    ) -> anyhow::Result<CharacterEquippedItemIds> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            InvType,
            ItemId,
            Slot,
        }

        let equip_items: Vec<(i32, i32, i32)> = equip_item::Entity::find()
            .select_only()
            .column_as(inventory_slot::Column::InvType, QueryAs::InvType)
            .column_as(equip_item::Column::ItemId, QueryAs::ItemId)
            .column_as(inventory_slot::Column::Slot, QueryAs::Slot)
            .inner_join(inventory_slot::Entity)
            .filter(inventory_slot::Column::InvType.is_in([
                InventoryType::Equipped as i32,
                InventoryType::Special as i32,
            ]))
            .filter(inventory_slot::Column::CharId.eq(char_id.0 as i32))
            .into_values::<_, QueryAs>()
            .all(&self.db.0)
            .await?;

        equip_items.iter().try_fold(
            CharacterEquippedItemIds::default(),
            |mut acc, &(inv_ty, item_id, slot)| {
                let item = (CharEquipSlot::try_from(slot as u8)?, ItemId(item_id as u32));
                // Inv type has to be either equipped or maskedequipped
                match InventoryType::try_from_primitive(inv_ty as u8).unwrap() {
                    InventoryType::Equipped => acc.equipped.push(item),
                    InventoryType::Special => acc.masked.push(item),
                    _ => unreachable!(),
                };
                Ok(acc)
            },
        )
    }
}

#[cfg(test)]
mod tests {

    use either::Either;
    use shroom_proto95::shared::{inventory::CharEquipSlot, Gender};
    use shroom_srv::game::stack_inv::InvStackItem;

    use crate::{
        gen_sqlite,
        services::{
            account::{AccountId, Region},
            character::{CharacterCreateDTO, ItemStarterSet},
            DataProvider,
        },
    };
    use shroom_meta::{
        id::{job_id::JobGroup, CharacterId, FaceId, HairId, Skin},
        MetaService,
    };

    fn get_mock_meta() -> &'static MetaService {
        Box::leak(Box::new(
            MetaService::load_from_dir("../../shroom-metadata", shroom_meta::MetaOption::Testing)
                .expect("Meta"),
        ))
    }

    async fn get_svc() -> anyhow::Result<(DataProvider, AccountId, CharacterId)> {
        let db = gen_sqlite(crate::SQL_OPT_MEMORY).await?;

        let data = DataProvider::create(db.clone(), get_mock_meta()).await?;

        let acc = &data.account;
        let acc_id = acc
            .create("test", "hunter3", Region::Europe, true, None)
            .await?;

        let item_svc = &data.item;
        let char = &data.char();
        let job = JobGroup::Legend;
        let char_id = char
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran".to_string(),
                    job: Either::Left(JobGroup::Legend),
                    face: FaceId::FEARFUL_STARE_F,
                    skin: Skin::White,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet {
                        bottom: job.get_starter_bottoms().next().unwrap(),
                        shoes: job.get_starter_shoes().next().unwrap(),
                        top: job.get_starter_tops().next().unwrap(),
                        weapon: job.get_starter_weapons().next().unwrap(),
                        guide: job.get_guide_item(),
                    },
                    gender: Gender::Male,
                    max_skills: false,
                    level: None
                },
                &item_svc,
            )
            .await?;

        Ok((data, acc_id, char_id))
    }

    #[tokio::test]
    async fn load_save_inventory() {
        let (svc, _acc_id, char_id) = get_svc().await.unwrap();
        let _inv = svc
            .item
            .load_inventory_for_character(char_id)
            .await
            .unwrap();
        svc.item
            .create_starter_set(
                char_id,
                ItemStarterSet::default_starter_set(JobGroup::Adventurer),
            )
            .await
            .unwrap();

        let mut inv = svc
            .item
            .load_inventory_for_character(char_id)
            .await
            .unwrap();
        assert_eq!(inv.equipped.len(), 4);
        assert_eq!(inv.etc.len(), 1);
        inv.equipped.try_remove(CharEquipSlot::Top.into()).unwrap();

        inv.etc.add_quantity(0, 5).unwrap();

        svc.item.save_inventory(&mut inv, char_id).await.unwrap();
        let inv = svc
            .item
            .load_inventory_for_character(char_id)
            .await
            .unwrap();
        assert_eq!(inv.equipped.len(), 3);
        assert_eq!(inv.etc.get(0).unwrap().quantity(), 1 + 5);

        let _eq = svc.item.load_equipped_items(char_id).await.unwrap();
    }
}
