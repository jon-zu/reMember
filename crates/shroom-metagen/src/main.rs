use schemas::item_mapper::ItemOptWithId;
use schemas::quest_mapper::SchQuest;
use schemas::shroom_schemas::Skill;
use schemas::skill_mapper::SkillWithId;
use shroom_meta::field::Field;
use shroom_meta::mob::{Mob, MobSkills};

use crate::schemas::item_mapper::{EquipWithId, ItemWithId};
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use shroom_meta::id::{FieldId, ItemId, ItemOptionId, MobId, QuestId, SkillId};
use shroom_meta::quest::Quest;
use shroom_meta::tmpl::equip::{EquipItemTmpl, WeaponItemTmpl};
use shroom_meta::tmpl::item::{BundleItemTmpl, ItemOption, EQ_TY, ITEM_TY};
use shroom_meta::{skill, FIELD_REGIONS};
use std::collections::{BTreeMap, HashMap};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub mod schemas;

fn parse_dir_name(s: &str) -> Option<usize> {
    s.strip_suffix(".img").and_then(|s| s.parse().ok())
}

/*fn load<T: DeserializeOwned>(name: &str) -> anyhow::Result<T> {
    // We use bincode for now
    let file = std::fs::File::open(format!("{name}.bincode"))?;
    Ok(bincode::deserialize_from(file)?)
}*/

fn save<T: serde::Serialize>(name: &str, v: &T, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    // We use bincode for now
    let file = std::fs::File::create(out_dir.as_ref().join(format!("{name}.bincode")))?;
    bincode::serialize_into(file, v)?;
    Ok(())
}

fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let file = std::fs::File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}

fn write_json<T: serde::Serialize>(
    name: &str,
    v: &T,
    out_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let file = BufWriter::new(std::fs::File::create(
        out_dir.as_ref().join(format!("{name}.json")),
    )?);
    serde_json::to_writer_pretty(file, v)?;
    Ok(())
}

fn parse_skill(
    p: impl AsRef<Path>,
) -> impl Iterator<Item = anyhow::Result<(SkillId, skill::Skill)>> {
    let skill_img: Skill = load_json(p).unwrap();

    skill_img.skill.into_iter().map(|(id, mut skill)| {
        let id = SkillId(id.parse::<u32>()?);
        if skill.common.is_none() {
            let levels = skill.level.len();
            let last_level = &skill.level[&levels.to_string()];

            skill.common = Some(last_level.try_into()?);
        }

        let skill = skill::Skill::try_from(SkillWithId(id, &skill))?;
        Ok((id, skill))
    })
}

fn parse_mob(p: impl AsRef<Path>, id: MobId) -> anyhow::Result<(MobId, Mob)> {
    let img: schemas::shroom_schemas::Mob = load_json(p).unwrap();
    //println!("{:?}", img);
    let mut mob = Mob::try_from(&img)?;
    mob.id = id;
    Ok((id, mob))
}

fn parse_field(p: impl AsRef<Path>, id: u32) -> anyhow::Result<(u32, Field)> {
    let img: schemas::shroom_schemas::Field = load_json(p).unwrap();
    let mut field = Field::try_from(&img)?;
    field.id = FieldId(id);
    Ok((id, field))
}

fn gen_skills(skill_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let skills = std::fs::read_dir(skill_dir.as_ref())?
        .filter_map(|dir| {
            if let Ok(dir) = dir {
                if let Ok(_num) = dir
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<u32>()
                {
                    return Some(dir);
                }
            }
            None
        })
        //.par_bridge()
        .flat_map(|dir| {
            parse_skill(dir.path()) //.par_bridge()
        })
        .collect::<anyhow::Result<BTreeMap<SkillId, skill::Skill>>>()?;

    write_json("skills", &skills, out_dir.as_ref())?;

    let mob_skills: schemas::shroom_schemas::MobSkill =
        load_json(skill_dir.as_ref().join("MobSkill.json"))?;
    let mob_skills = MobSkills::try_from(&mob_skills)?;
    write_json("mob_skills", &mob_skills, out_dir)?;

    Ok(())
}

fn gen_fields(dir: impl AsRef<Path>, region: u8, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    dbg!(dir.as_ref());
    let data = std::fs::read_dir(dir)?
        .filter_map(|f| {
            if let Ok(f) = f {
                let f = f.path();
                if let Ok(num) = f.file_stem().unwrap().to_str().unwrap().parse::<u32>() {
                    return Some((num, f));
                }
            }
            None
        })
        .par_bridge()
        .map(|(id, f)| parse_field(f, id as u32))
        .collect::<anyhow::Result<BTreeMap<u32, Field>>>()?;

    save(
        &format!("fields{region}"),
        &data,
        out_dir.as_ref().join("fields"),
    )?;

    Ok(())
}

fn gen_mobs(mob_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let mobs = std::fs::read_dir(mob_dir)?
        .map(|dir| {
            let f = dir.unwrap().path();
            println!("{f:?}");
            let id: u32 = f.file_stem().unwrap().to_str().unwrap().parse().unwrap();
            parse_mob(f, MobId(id))
        })
        .collect::<anyhow::Result<BTreeMap<MobId, Mob>>>()?;

    save("mobs", &mobs, out_dir)?;
    Ok(())
}

/*
fn parse_quest(id: QuestId, q: SchQuest) -> anyhow::Result<Option<(QuestId, Quest)>> {
    if img.quest_info.is_none() {
        println!("No quest info for quest: {:?}", id);
        return Ok(None);
    }
    if img.check.is_empty() {
        println!("No check for quest: {:?}", id);
        return Ok(None);
    }

    img.check = check.unwrap().clone();
    img.act = act.unwrap().clone();



    //println!("{:?}", img);
    let d = QuestWithId(id, &img);
    Ok(Some((id, d.try_into()?)))
}*/

fn gen_quests(dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let check: schemas::shroom_schemas::QuestCheck = load_json(dir.as_ref().join("Check.json"))?;
    let act: schemas::shroom_schemas::QuestAct = load_json(dir.as_ref().join("Act.json"))?;
    let info: schemas::shroom_schemas::QuestInfo = load_json(dir.as_ref().join("QuestInfo.json"))?;

    let mut quests = BTreeMap::new();
    for (sid, v) in info.iter() {
        println!("Quest: {:?}", sid);
        let id = QuestId(sid.parse::<u16>().unwrap());
        let check = check.0.get(sid).unwrap();
        let act = act.0.get(sid).unwrap();

        let q = SchQuest {
            check_0: check.get("0").unwrap(),
            check_1: check.get("1").unwrap(),
            act_0: act.get("0").unwrap(),
            act_1: act.get("1").unwrap(),
            id,
            info: v,
        };
        let q = Quest::try_from(q)?;
        quests.insert(id, q);
    }

    write_json("quest", &quests, out_dir)?;
    Ok(())
}

fn gen_wep(dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let eq = std::fs::read_dir(dir)?
        .map(|dir| {
            let f = dir.unwrap().path();
            dbg!(&f);
            let id: u32 = f.file_stem().unwrap().to_str().unwrap().parse().unwrap();
            let id = ItemId(id);
            Ok((
                id,
                WeaponItemTmpl::try_from(EquipWithId(
                    id,
                    &load_json::<schemas::shroom_schemas::CharItem>(f)?,
                ))?,
            ))
        })
        .collect::<anyhow::Result<BTreeMap<ItemId, WeaponItemTmpl>>>()?;

    save("wep", &eq, out_dir)?;
    Ok(())
}

fn gen_eq(dir: impl AsRef<Path>, eq_ty: &str, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let eq = std::fs::read_dir(dir.as_ref().join(eq_ty))?
        .par_bridge()
        .map(|dir| {
            let f = dir.unwrap().path();
            dbg!(&f);
            let id: u32 = f.file_stem().unwrap().to_str().unwrap().parse().unwrap();
            let id = ItemId(id);
            Ok((
                id,
                EquipItemTmpl::try_from(EquipWithId(
                    id,
                    &load_json::<schemas::shroom_schemas::CharItem>(f)?,
                ))?,
            ))
        })
        .collect::<anyhow::Result<BTreeMap<ItemId, EquipItemTmpl>>>()?;

    save(eq_ty, &eq, out_dir)?;
    Ok(())
}

fn gen_item(dir: impl AsRef<Path>, item_ty: &str, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let items = std::fs::read_dir(dir.as_ref().join(item_ty))?
        //.par_bridge()
        .flat_map(|dir| {
            let f = dir.unwrap().path();
            dbg!(&f);
            let items: schemas::shroom_schemas::Item = load_json(f).unwrap();
            items.0.into_iter()
        })
        .map(|(id, item)| {
            let id = ItemId(id.parse::<u32>().unwrap());
            Ok((id, BundleItemTmpl::try_from(ItemWithId(id, &item))?))
        })
        .collect::<anyhow::Result<BTreeMap<ItemId, BundleItemTmpl>>>()?;

    save(item_ty, &items, out_dir)?;
    Ok(())
}

fn gen_item_opt(dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let ops: schemas::shroom_schemas::ItemOptions =
        load_json(dir.as_ref().join("ItemOption.json"))?;

    let opts: BTreeMap<ItemOptionId, ItemOption> = ops
        .0
        .iter()
        .map(|v| {
            let id = ItemOptionId(v.0.parse::<u16>().unwrap());
            let v = ItemOptWithId(id, v.1);
            anyhow::Ok((id, ItemOption::try_from(v)?))
        })
        .collect::<anyhow::Result<_>>()?;

    write_json("item_options", &opts, out_dir)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let p = PathBuf::from_str("/home/jonas/projects/shroom/data")?;
    let out_dir = PathBuf::from_str("/home/jonas/projects/shroom/ShroomMS/shroom-metadata")?;
    
    //    gen_skills(p.join("skills"), &out_dir)?;
        for i in FIELD_REGIONS {
            gen_fields(p.join(format!("maps/Map/Map{i}")), i, &out_dir)?;
        }

      /*   gen_mobs(p.join("mobs"), &out_dir)?;

        gen_quests(p.join("quest"), &out_dir)?;

        gen_wep(p.join("char/Weapon"), &out_dir)?;
        for eq_ty in EQ_TY {
            gen_eq(p.join("char"), eq_ty, &out_dir)?;
        }
        */
    
    /*for eq_ty in ITEM_TY {
        gen_item(p.join("item"), eq_ty, &out_dir)?;
    }
    gen_item_opt(p.join("item"), &out_dir)?;*/

    Ok(())
}
