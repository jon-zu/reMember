use shroom_meta::id::{job_id::JobId, CharacterId, FaceId, FieldId, HairId, Skin};
use shroom_proto95::shared::char::{CharStat, Pets};

use crate::entities::character;

impl From<&character::Model> for CharStat {
    fn from(char: &character::Model) -> Self {
        let job = JobId::try_from(char.job as u16).unwrap();

        let skill_page = char.get_skill_pages();
        Self {
            char_id: CharacterId(char.id as u32),
            name: char.name.as_str().try_into().unwrap(),
            gender: (&char.gender).into(),
            skin_color: Skin::try_from(char.skin as u8).unwrap(),
            face: FaceId(char.face as u32),
            hair: HairId(char.hair as u32),
            pets: Pets::default(),
            level: char.level as u8,
            job_id: job,
            str: char.str as u16,
            dex: char.dex as u16,
            int: char.int as u16,
            luk: char.luk as u16,
            hp: char.hp as u32,
            max_hp: char.max_hp as u32,
            mp: char.mp as u32,
            max_mp: char.max_mp as u32,
            ap: char.ap as u16,
            sp: skill_page.to_proto(job.has_extended_sp()).into(),
            exp: char.exp,
            fame: char.fame as u16,
            tmp_exp: char.gacha_exp as u32,
            map_id: FieldId(char.field_id as u32),
            portal: char.spawn_point as u8,
            playtime: 0,
            sub_job: 0,
        }
    }
}
