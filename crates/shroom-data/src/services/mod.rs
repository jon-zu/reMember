use either::Either;
use sea_orm::DatabaseConnection;
use shroom_meta::{
    id::{
        job_id::{JobGroup, JobId},
        CharacterId, FaceId, HairId, ItemId, Skin,
    }, srv::ItemSet, CharLevel, MetaService
};
use shroom_proto95::shared::Gender;

use crate::entities::sea_orm_active_enums::GenderTy;

use self::{
    account::{AccountId, AccountService, Region},
    character::{CharacterCreateDTO, CharacterService, ItemStarterSet},
    item::ItemService,
};

pub mod account;
pub mod character;
pub mod item;
pub mod password;
pub mod server_service;
//pub mod shared;

#[derive(Debug, Clone)]
pub struct DbConn(pub DatabaseConnection);

#[derive(Debug)]
pub struct MetaSvc(pub &'static MetaService);

#[derive(Debug)]
pub struct DataProvider {
    pub db: DbConn,
    meta: &'static MetaService,
    pub account: AccountService,
    pub item: ItemService,
    //pub char: CharacterService,
}

impl DataProvider {
    pub async fn create(
        db: DatabaseConnection,
        meta: &'static MetaService,
    ) -> anyhow::Result<Self> {
        let db = DbConn(db);
        Ok(Self {
            db: db.clone(),
            meta,
            account: AccountService::new(db.clone()),
            item: ItemService::new(db, meta).await?,
            //char: CharacterService::new(db.clone(), meta),
        })
    }

    pub fn char(&self) -> CharacterService {
        CharacterService::new(self.db.clone(), self.meta, &self.account, &self.item)
    }

    /*pub fn new(
        db: DatabaseConnection,
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
    ) -> Self {
        let game = Arc::new(GameServices {
            data: DataServices::new(db, meta),
            server_info: ServerService::new(servers),
            meta,
        });

        let session_backend = ShroomSessionBackend::new(game.clone());

        Self {
            game,
            session_manager: ShroomSessionManager::new(session_backend, Duration::from_secs(30)),
        }
    }*/

    pub async fn seeded_in_memory(meta: &'static MetaService) -> anyhow::Result<Self> {
        let db = crate::gen_sqlite(crate::SQL_OPT_MEMORY).await?;
        Self::create(db, meta).await
    }

    pub async fn seeded_in_db(meta: &'static MetaService, db_url: &str) -> anyhow::Result<Self> {
        let db = crate::gen_psql(db_url).await?;
        Self::create(db, meta).await
    }

    pub async fn from_db_url(meta: &'static MetaService, db_url: &str) -> anyhow::Result<Self> {
        let db = crate::gen_psql(db_url).await?;
        Self::create(db, meta).await
    }

    pub async fn seed_acc_char(&self) -> anyhow::Result<(AccountId, CharacterId)> {
        let acc_id = self
            .account
            .create(
                "admin",
                "test1234",
                Region::Europe,
                true,
                Some(GenderTy::Female),
            )
            .await?;

        let job = JobGroup::Legend;
        let _char_id = Box::pin(self.char().create_character(
            acc_id,
            CharacterCreateDTO {
                name: "Aran".to_string(),
                job: Either::Left(JobGroup::Adventurer),
                face: FaceId::LEISURE_LOOK_M,
                skin: Skin::Normal,
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
            &self.item,
        ))
        .await?;

        let job = JobGroup::Legend;
        let char_id = Box::pin(self.char().create_character(
            acc_id,
            CharacterCreateDTO {
                name: "Aran2".to_string(),
                job: Either::Left(JobGroup::Legend),
                face: FaceId::LEISURE_LOOK_M,
                skin: Skin::Normal,
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
            &self.item,
        ))
        .await?;

        Ok((acc_id, char_id))
    }

    pub async fn seed_class(
        &self,
        name: &str,
        jobs: &[JobId],
        item_set: &ItemSet,
    ) -> anyhow::Result<()> {
        let acc_id = self
            .account
            .create(
                format!("test_{name}"),
                "test1234",
                Region::Europe,
                true,
                Some(GenderTy::Female),
            )
            .await?;


        for &job in jobs {
            let name = format!("{name}{}", job as u16);
            let job_group = job.job_group();
            let mut level = Some(CharLevel(150));
            let job = if job == JobId::ArchMageIceLightning {
                level = Some(CharLevel(10));
                JobId::Beginner
            } else {
                job
            };

            let char_id = Box::pin(self.char().create_character(
                acc_id,
                CharacterCreateDTO {
                    name,
                    job: Either::Right(job),
                    face: FaceId::LEISURE_LOOK_M,
                    skin: Skin::Normal,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet::from_job_group(job_group),
                    gender: Gender::Male,
                    max_skills: true,
                    level
                    
                },
                &self.item,
            ))
            .await?;

            let mut inv = self.item.load_inventory_for_character(char_id).await?;
            for eq in item_set.equips.iter() {
                let eq = self.item.create_equip(*eq)?;
                inv.equip.try_add(eq.into())?;
            }

            for consume in item_set.consumables.iter() {
                let stack = self
                    .item
                    .create_stack(consume.id, consume.quantity as u16)?;
                inv.get_stack_inventory_mut(stack.item_id.get_inv_type()?)?
                    .try_add(stack.into())?;
            }
            for _ in 0..3 {
                inv.get_cash_inventory_mut()
                    .try_add(crate::model::inv::CashItemSlot::Pet(
                        self.item.create_pet(ItemId(5000008)).unwrap()
                            .into()
                    ))
                    .unwrap();
            }
            self.item.save_inventory(&mut inv, char_id).await?;
        }

        Ok(())
    }
}
