use clap::{Command, FromArgMatches, Parser, Subcommand};

use itertools::Itertools;
use shroom_meta::{
    id::{job_id::JobId, FieldId, ItemId, MobId, NpcId, QuestId, SkillId},
    twod::Rect32,
};
use shroom_proto95::game::{
    field::{FieldEffectResp, TrembleEffectData},
    quest::{ConstantU8, QuestRecordMessageResp, QuestState},
    shop::{OpenShopResp, ShopItem},
    user::pet::PetActionResp,
};

use crate::{
    field,
    life::{
        drop_item::{DropItem, DropTypeValue},
        minor::{AffectedArea, TownPortal},
        mob::Mob,
    },
};
use crate::{
    game::{GameContext, GameSession},
    life::char::pet::Pet,
};
use shroom_srv::act::Context;

#[derive(Parser, Debug)]
pub enum ReplCmd {
    Mob { id: Option<u32> },
    Pet { id: u32 },
    Mesos { amount: u32 },
    Item { id: Option<u32> },
    Chat { msg: String },
    FakeUser { id: u32 },
    Aggro,
    Dispose,
    Teleport { id: Option<u32> },
    Sp { add: u32 },
    Job { id: u32 },
    TestSet,
    Level { level: u8 },
    MaxSkills,
    SpamDrop,
    StopSpamDrop,
    //Dialog,
    Shop,
    Img,
    Stats { add: u16 },
    Freeze,
    Zakum,
    Go { q: String },
    ItemSet { q: String },
    Script { q: String },
    MysticDoor,
    AffectedArea,
    Exp { amount: u32 },
    StartQuest { id: u16 },
    UpdateQuest { id: u16, state: String },
    GiveScroll,
    EarthQuake,
}

pub struct GameRepl {
    cli: Command,
}

impl Default for GameRepl {
    fn default() -> Self {
        Self::new()
    }
}

impl GameRepl {
    pub fn new() -> Self {
        const PARSER_TEMPLATE: &str = "\
    {all-args}
";
        let cmd = Command::new("repl")
            .multicall(true)
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand_value_name("APPLET")
            .subcommand_help_heading("APPLETS")
            .help_template(PARSER_TEMPLATE);

        let cmd = ReplCmd::augment_subcommands(cmd);

        Self { cli: cmd }
    }
    pub fn match_cmd(&mut self, s: &str) -> anyhow::Result<ReplCmd> {
        let args = s.split_whitespace();
        let matches = self.cli.try_get_matches_from_mut(args)?;
        Ok(ReplCmd::from_arg_matches(&matches)?)
    }

    pub fn help(&mut self) -> String {
        self.cli.render_help().to_string()
    }
}

impl GameSession {
    pub fn handle_repl_cmd(
        &mut self,
        ctx: &mut GameContext,
        cmd: ReplCmd,
    ) -> anyhow::Result<Option<String>> {
        let chr = &self.session.char;
        Ok(match cmd {
            ReplCmd::EarthQuake => {
                ctx.socket
                    .reply(FieldEffectResp::Tremble(TrembleEffectData {
                        heavy_n_short: true,
                        delay: std::time::Duration::from_secs(0).into(),
                    }))?;
                None
            }
            ReplCmd::GiveScroll => {
                let chr = &mut self.session.char;
                chr.add_items(ItemId::CHAOS_SCROLL_60, Some(50))?;
                chr.add_items(ItemId::WHITE_SCROLL, Some(50))?;
                chr.add_items(ItemId(2049300), Some(50))?;
                chr.add_items(ItemId(2049301), Some(50))?;
                chr.add_items(ItemId(2043805), Some(50))?;

                None
            }
            ReplCmd::UpdateQuest { id, state } => {
                ctx.socket.reply(QuestRecordMessageResp {
                    marker: ConstantU8,
                    id: QuestId(id),
                    state: QuestState::Accept(state),
                })?;
                None
            }

            ReplCmd::StartQuest { id } => {
                ctx.socket.reply(QuestRecordMessageResp {
                    marker: ConstantU8,
                    id: QuestId(id),
                    state: QuestState::Accept("".to_string()),
                })?;
                None
            }
            ReplCmd::Exp { amount } => {
                self.session.char.add_exp(amount);
                None
            }
            ReplCmd::AffectedArea => {
                field!(ctx).create_affected_area(AffectedArea {
                    ty: shroom_proto95::game::life::affected_area::AffectedAreaType::MobSkill,
                    owner_id: chr.id,
                    skill_id: SkillId(123),
                    skill_level: 3,
                    start_delay: 60,
                    area: Rect32 {
                        left: (chr.pos.x - 60) as i32,
                        top: (chr.pos.y - 50) as i32,
                        right: (chr.pos.x + 60) as i32,
                        bottom: (chr.pos.y + 50) as i32,
                    },
                    phase: 0,
                    elem_attr: 0,
                })?;
                None
            }
            ReplCmd::Mob { id } => {
                let mob = MobId(id.unwrap_or(9300039));
                let _meta = self.meta().get_mob_data(mob).unwrap();
                field!(ctx).add_mob(Mob::new_at(
                    self.meta(),
                    mob,
                    self.session.char.pos,
                    self.session.char.fh,
                    None,
                ))?;
                None
            }
            ReplCmd::Pet { .. } => {
                /*ctx.socket.reply(PetLocalActivateResp {
                    char: chr.id,
                    pet_id: 0,
                    pet_data: PetLocalActivateResult::Ok(PetInitData {
                        reset_active: false,
                        pet_tmpl_id: 5000008,
                        pet_name: "Monkey".to_string(),
                        pet_locker_sn: 0,
                        pos: chr.pos,
                        move_action: 0,
                        fh: chr.fh,
                        name_tag: true,
                        chat_balloon: true,
                    }),
                })?;*/
                let msg = ["I hate nexon packets!", "me too", "nim sucks"];
                for (ix, msg) in msg.iter().enumerate() {
                    let chr = &mut self.session.char;
                    let pet = chr.inventory.get_pet(ix).unwrap();
                    let pet = Pet::new(pet.item_id.0, "PET".to_string(), pet.game_id as u64);
                    let char_id = chr.id;
                    self.session.char.add_pet(pet, ctx)?;
                    ctx.socket.reply(PetActionResp {
                        user: char_id,
                        pet_id: ix as u8,
                        ty: 0,
                        action: 0,
                        chat: msg.to_string(),
                        chat_balloon: true,
                    })?;
                }

                None
            }
            ReplCmd::Img => None,
            ReplCmd::Mesos { amount } => {
                field!(ctx).add_drop(DropItem {
                    owner: shroom_proto95::game::drop::DropOwner::None,
                    pos: chr.pos,
                    start_pos: chr.pos,
                    value: DropTypeValue::Mesos(amount),
                    quantity: 1,
                })?;
                None
            }
            ReplCmd::Item { id } => {
                let item = id.map_or(ItemId::ADVANCED_MONSTER_CRYSTAL_1, ItemId);
                field!(ctx).add_drop(DropItem {
                    owner: shroom_proto95::game::drop::DropOwner::None,
                    pos: chr.pos,
                    start_pos: chr.pos,
                    value: DropTypeValue::Item(item),
                    quantity: 1,
                })?;
                None
            }
            ReplCmd::FakeUser { id } => {
                log::info!("Adding fake user {id} not implemented yet");
                /*self.field.add_user(User {
                    avatar_data: self.session.char.get_avatar_data(),
                    char_id: id,
                    pos: self.char().pos,
                    fh: self.char().fh,
                })?;*/
                None
            }
            ReplCmd::Aggro => None,
            ReplCmd::Dispose => {
                self.enable_char();
                None
            }
            ReplCmd::Chat { msg } => Some(msg),
            ReplCmd::Teleport { id } => {
                let field = FieldId(id.unwrap_or(240040521));
                self.do_field_transfer(ctx, field, None)?;
                None
            }
            ReplCmd::Sp { add } => {
                self.session.char.add_sp(add);
                None
            }
            ReplCmd::Stats { add } => {
                *self.session.char.stats.str_mut() += add;
                *self.session.char.stats.int_mut() += add;
                *self.session.char.stats.dex_mut() += add;
                *self.session.char.stats.luk_mut() += add;
                None
            }
            ReplCmd::Job { id } => {
                let job = JobId::try_from(id as u16)?;
                self.session.char.change_job(job, true)?;
                None
            }
            ReplCmd::Level { level } => {
                self.session.char.set_level(level);
                None
            }
            ReplCmd::TestSet => {
                let item = &self.services.game.data.item;
                self.session.char.give_test_set(item)?;
                None
            }
            ReplCmd::MaxSkills => {
                self.session.char.skills.max_all();
                None
            }
            ReplCmd::SpamDrop => None,
            ReplCmd::StopSpamDrop => None,
            ReplCmd::Script { q } => {
                let Some(script) = self.services.game.scripts.get_npc_script(NpcId::ADMIN) else {
                    return Ok(Some("Script not found".to_string()));
                };
                log::info!("Loading script: {q}");
                self.start_script(ctx, script)?;
                None
            }
            ReplCmd::Shop => {
                let npc_tmpl_id: NpcId = 21000.into();
                let shop = self.meta().get_npc_shop(npc_tmpl_id).unwrap();
                ctx.socket.reply(OpenShopResp {
                    npc_tmpl_id,
                    items: shop
                        .items
                        .iter()
                        .map(|item| ShopItem {
                            item_id: ItemId(item.item_id),
                            price: item.price,
                            quantity: 10,
                            discount_rate: 100,
                            token_item_id: ItemId(0),
                            token_price: 0,
                            item_period: 0,
                            level_limited: 0,
                            max_per_slot: u8::MAX as u16,
                        })
                        .collect(),
                })?;
                None
            }
            ReplCmd::Freeze => {
                //field!(ctx).apply_freeze_on_all()?;
                None
            }
            ReplCmd::Zakum => {
                let start = 8800003;
                let mobs = 8;

                let _meta = self.meta().get_mob_data(8800000.into()).unwrap();
                field!(ctx).add_mob(Mob::new_at(
                    self.meta(),
                    8800000.into(),
                    self.session.char.pos,
                    self.session.char.fh,
                    None,
                ))?;

                for i in 0..mobs {
                    let mob = start + i;
                    let _meta = self.meta().get_mob_data(mob.into()).unwrap();
                    field!(ctx).add_mob(Mob::new_at(
                        self.meta(),
                        mob.into(),
                        self.session.char.pos,
                        self.session.char.fh,
                        None,
                    ))?;
                }

                None
            }
            ReplCmd::Go { q } => {
                match self.meta().goto().get_or_query(&q) {
                    Ok(field) => {
                        self.do_field_transfer(ctx, field.0, None)?;
                    }
                    Err(alt) => {
                        let alt = alt.map(|f| f.0).join(", ");
                        return Ok(Some(format!("Did you mean: {alt}")));
                    }
                }
                None
            }
            ReplCmd::ItemSet { q } => {
                match self.meta().item_sets().get_or_query(&q) {
                    Ok(set) => {
                        for eq in &set.equips {
                            self.session.char.add_equip_item(*eq)?;

                            for c in &set.consumables {
                                self.session.char.add_stack_item(
                                    c.id.get_inv_type()?,
                                    c.id,
                                    c.quantity,
                                )?;
                            }
                        }
                    }
                    Err(alt) => {
                        let alt = alt.map(|f| f.0).join(", ");
                        return Ok(Some(format!("Did you mean: {alt}")));
                    }
                }
                None
            }
            ReplCmd::MysticDoor => {
                field!(ctx).add_town_portal(TownPortal {
                    char_id: dbg!(chr.id),
                    state: 1,
                    pos: chr.pos,
                    target_map: FieldId::HENESYS,
                })?;
                None
            }
        })
    }

    pub fn handle_repl(
        &mut self,
        ctx: &mut GameContext,
        s: &str,
    ) -> anyhow::Result<Option<String>> {
        Ok(match self.repl.match_cmd(s) {
            Err(_) => Some(self.repl.help()),
            Ok(cmd) => self.handle_repl_cmd(ctx, cmd)?,
        })
    }
}
