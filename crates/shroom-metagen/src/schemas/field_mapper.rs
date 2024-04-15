use std::{collections::BTreeMap, time::Duration};

/*


<member name="FIELDOPT_MOVELIMIT" value="1" />
<member name="FIELDOPT_SKILLLIMIT" value="2" />
<member name="FIELDOPT_SUMMONLIMIT" value="4" />
<member name="FIELDOPT_MYSTICDOORLIMIT" value="8" />
<member name="FIELDOPT_MIGRATELIMIT" value="16" />
<member name="FIELDOPT_PORTALSCROLLLIMIT" value="32" />
<member name="FIELDOPT_TELEPORTITEMLIMIT" value="64" />
<member name="FIELDOPT_MINIGAMELIMIT" value="128" />
<member name="FIELDOPT_SPECIFICPORTALSCROLLLIMIT" value="256" />
<member name="FIELDOPT_TAMINGMOBLIMIT" value="512" />
<member name="FIELDOPT_STATCHANGEITEMCONSUMELIMIT" value="1024" />
<member name="FIELDOPT_PARTYBOSSCHANGELIMIT" value="2048" />
<member name="FIELDOPT_NOMOBCAPACITYLIMIT" value="4096" />
<member name="FIELDOPT_WEDDINGINVITATIONLIMIT" value="8192" />
*/

/*use shroom_proto95::{
    id::FieldId,
    shared::{FootholdId, Rect2D, Vec2}, game::life::{npc::NpcId, mob::MobId, reactor::ReactorId},
};*/
use shroom_meta::{
    field::{Field, FieldLife, FieldMob, FieldNpc, FieldPortal, FieldReactor, Foothold},
    id::{FieldId, FootholdId, MobId, NpcId, ReactorId},
    twod::{Rect2D, Vec2},
};

use shroom_meta::field::FhTree;

use super::shroom_schemas as sch;

fn map_bool(value: &Option<sch::Bool>) -> bool {
    value.as_ref().map(|v| v.into()).unwrap_or(false)
}

impl TryFrom<&sch::FieldLifeValue> for FieldLife {
    type Error = anyhow::Error;

    fn try_from(value: &sch::FieldLifeValue) -> Result<Self, Self::Error> {
        let pos = Vec2::new(value.x.unwrap() as i16, value.y.unwrap() as i16);
        let hide = map_bool(&value.hide);
        let fh = FootholdId(*value.fh.as_ref().unwrap() as u16);
        let id: u32 = value.id.as_ref().unwrap().into();
        Ok(match value.type_.as_deref().unwrap().as_str() {
            "m" => Self::Mob(FieldMob {
                id: MobId(id),
                pos,
                fh,
                hide,
                respawn_time: value
                    .mob_time
                    .filter(|v| v >= &0)
                    .map(|v| Duration::from_secs(v as u64)),
                range_x: value.rx0.unwrap() as i16..=value.rx1.unwrap() as i16,
                flip: value.f.unwrap_or(0) == 1,
                cy: value.cy.map(|v| v as i16),
            }),
            "n" => Self::Npc(FieldNpc {
                id: NpcId(id),
                pos,
                fh,
                hide,
                range_x: value.rx0.unwrap() as i16..=value.rx1.unwrap() as i16,
                flip: value.f.unwrap_or(0) == 1,
            }),
            _ => {
                anyhow::bail!("Unknown life type")
            }
        })
    }
}

impl TryFrom<&sch::FieldReactorValue> for FieldReactor {
    type Error = anyhow::Error;

    fn try_from(value: &sch::FieldReactorValue) -> Result<Self, Self::Error> {
        let id: u32 = value.id.as_ref().unwrap().into();
        Ok(Self {
            pos: Vec2::new(value.x.unwrap() as i16, value.y.unwrap() as i16),
            name: value.name.clone(),
            id: ReactorId(id),
            time: value.reactor_time.map(|v| v as u32),
        })
    }
}

impl TryFrom<&sch::FieldPortalValue> for FieldPortal {
    type Error = anyhow::Error;

    fn try_from(value: &sch::FieldPortalValue) -> Result<Self, Self::Error> {
        Ok(Self {
            pos: Vec2::new(value.x.unwrap() as i16, value.y.unwrap() as i16),
            only_once: map_bool(&value.only_once),
            hide_tooltip: map_bool(&value.hide_tooltip),
            has_delay: map_bool(&value.delay),
            teleport: map_bool(&value.teleport),
            reactor_name: value.reactor_name.clone(),
            script: value.script.clone(),
            session_value: value.session_value.clone(),
            session_value_key: value.session_value_key.clone(),
            pn: value.pn.clone(),
            tn: value.tn.clone(),
            tm: value.tm.map(|v| FieldId(v as u32)),
            pt: value.pt.map(|v| FieldId(v as u32)),
        })
    }
}

impl From<&sch::Fh> for Foothold {
    fn from(value: &sch::Fh) -> Self {
        Self {
            pt1: Vec2::new(value.x1 as i16, value.y1 as i16),
            pt2: Vec2::new(value.x2 as i16, value.y2 as i16),
            next: FootholdId(value.next as u16),
            prev: FootholdId(value.prev as u16),
            forbid_falldown: map_bool(&value.forbid_fall_down),
            cant_through: map_bool(&value.cant_through),
            force: value.force.as_ref().map(|v| *v as i32),
            piece: value.piece.as_ref().map(|v| *v as i32),
        }
    }
}

impl TryFrom<&sch::Field> for Field {
    type Error = anyhow::Error;

    fn try_from(value: &sch::Field) -> Result<Self, Self::Error> {
        let info = value.info.as_ref().ok_or(anyhow::anyhow!("No info"))?;

        let fhs = &value.foothold;

        let footholds = fhs.iter().map(
            |(id, fh)| {
                let id = id.parse::<FootholdId>()?;
                let fhs = fh.iter().map(
                    |(id, fh)| {
                        let id = id.parse::<FootholdId>()?;
                        let fhs = fh.iter().map(
                            |(id, fh)| {
                                let id = id.parse::<FootholdId>()?;
                                let fh = Foothold::from(fh);
                                Ok((id, fh))
                            }
                        ).collect::<anyhow::Result<BTreeMap<FootholdId, Foothold>>>()?;
                        Ok((id, fhs))
                    }
                ).collect::<anyhow::Result<BTreeMap<FootholdId, BTreeMap<FootholdId, Foothold>>>>()?;
                Ok((id, fhs))
            }
        ).collect::<anyhow::Result<BTreeMap<FootholdId, BTreeMap<FootholdId, BTreeMap<FootholdId, Foothold>>>>>()?;

        let rect = Rect2D::new(
            Vec2::new(
                info.vr_left.unwrap_or(0) as i16,
                info.vr_bottom.unwrap_or(0) as i16,
            )
            .to_point(),
            Vec2::new(
                info.vr_right.unwrap_or(0) as i16,
                info.vr_top.unwrap_or(0) as i16,
            )
            .to_point(),
        );

        let fh_tree = FhTree::from_meta(&footholds, rect.clone());

        Ok(Self {
            id: FieldId::NONE,
            cloud: map_bool(&info.cloud),
            scroll_disable: map_bool(&info.scroll_disable),
            no_regen: map_bool(&info.no_regen_map),
            fly: map_bool(&info.fly),
            zakum_hack: map_bool(&info.zakum2_hack),
            return_field: info.return_map.map(|v| FieldId(v as u32)),
            forced_return_field: info.forced_return.map(|v| FieldId(v as u32)),
            rect,
            portals: value
                .portal
                .iter()
                .map(|(id, portal)| {
                    let id = id.parse::<u8>()?;
                    let portal = FieldPortal::try_from(portal)?;
                    Ok((id, portal))
                })
                .collect::<anyhow::Result<BTreeMap<u8, FieldPortal>>>()?,
            life: value
                .life
                .iter()
                .map(|(id, life)| {
                    let id = id.parse::<u32>()?;
                    let life = FieldLife::try_from(life)?;
                    Ok((id, life))
                })
                .collect::<anyhow::Result<BTreeMap<u32, FieldLife>>>()?,
            footholds,
            reactors: value
                .reactor
                .iter()
                .map(|(id, reactor)| {
                    let id = id.parse::<u32>()?;
                    let reactor = FieldReactor::try_from(reactor)?;
                    Ok((id, reactor))
                })
                .collect::<anyhow::Result<BTreeMap<u32, FieldReactor>>>()?,
            fh_tree,
        })
    }
}
