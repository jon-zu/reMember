use anyhow::Context;
use rand::{thread_rng, Rng};
use shroom_meta::{
    fmt::{ItemIcon, MapName, ShroomDisplay},
    id::{job_id::JobId, skill_id, FieldId, ItemId, JobClass, MobId, Money},
};
use shroom_script::npc::NpcCtx;

pub async fn npc_fallback(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    api.say_end(format!("Npc not implemented: {}", api.npc_id())).await?;
    Ok(())
}

pub async fn npc_script_1000(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    api.say_next("Hello I'm a NPC!").await?;

    let item = ItemId::ALL_CURE_POTION;
    if api.ask_yes_no("Do you want starter items?").await? {
        api.say_next(format!(
            "I'll give you 3 {item:+} and {}",
            "500 mesos".blue()
        ))
        .await?;
        let _ = api.try_give_items(&[(item, 3)]);
        api.try_update_money(500);
    }

    let dispel = skill_id::PRIEST_DISPEL;

    api.say_next(format!(
        "Don't forget to use your spells like: {0} ## {0:+}",
        dispel,
    ))
    .await?;

    let sel = api
        .ask_selection("Language?", vec!["English", "Spanish", "German"].into())
        .await?;
    let bye = match sel {
        0 => "Goodbye!",
        1 => "Adios!",
        2 => "Auf Wiedersehen!",
        _ => unreachable!(),
    };
    api.say_end(bye).await?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MineCell {
    Mine,
    Number(u8),
    Revealed(u8),
    RevealedMine,
}

impl std::fmt::Display for MineCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const NUM: ItemId = ItemId(4000000);
        const BOMB: ItemId = ItemId(4000020);
        const HIDDEN: ItemId = ItemId(4000022);

        write!(
            f,
            "{}",
            ItemIcon(match self {
                Self::Mine => HIDDEN,
                Self::RevealedMine => BOMB,
                Self::Number(_) => HIDDEN,
                Self::Revealed(n) => ItemId(NUM.0 + *n as u32),
            })
        )
    }
}

impl ShroomDisplay for MineCell {}

pub async fn npc_script_minesweeper(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    // Generate board
    const W: usize = 5;
    const H: usize = 5;
    const MINES: usize = 5;
    let mut board = [MineCell::Number(0); W * H];
    let mut mines = 0;
    while mines < MINES {
        let x = rand::thread_rng().gen_range(0..W);
        let y = rand::thread_rng().gen_range(0..H);
        let ix = y * W + x;
        if board[ix] == MineCell::Mine {
            continue;
        }

        board[ix] = MineCell::Mine;
        // Update numbers
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= W as isize || ny < 0 || ny >= H as isize {
                    continue;
                }

                let ix = ny as usize * W + nx as usize;
                if let MineCell::Number(n) = board[ix] {
                    board[ix] = MineCell::Number(n + 1);
                }
            }
        }
        mines += 1;
    }

    // Game loop
    let mut rem = H * W - MINES;
    while rem > 0 {
        let ix = api.ask_grid("Minesweeper:", &board, H, W).await?;
        match board[ix] {
            MineCell::Mine => {
                board[ix] = MineCell::RevealedMine;
                api.say_end("You lost!").await?;
                return Ok(());
            }
            MineCell::Number(n) => {
                board[ix] = MineCell::Revealed(n);
                rem -= 1;
            }
            _ => {}
        }

        rem -= 1;
    }
    api.say_end("You won").await?;
    Ok(())
}

pub struct MemoryCell {
    pub revealed: bool,
    pub item: ItemId,
}

impl MemoryCell {
    pub fn new(item: ItemId) -> Self {
        Self {
            revealed: false,
            item,
        }
    }

    pub fn from_ix(ix: usize) -> Self {
        Self::new(ItemId(4000000 + ix as u32))
    }

    pub fn item_id(&self) -> ItemId {
        if self.revealed {
            self.item
        } else {
            ItemId(4000022)
        }
    }
}

impl std::fmt::Display for MemoryCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const HIDDEN: ItemId = ItemId(4000022);
        write!(
            f,
            "{}",
            ItemIcon(if self.revealed { self.item } else { HIDDEN })
        )
    }
}

impl ShroomDisplay for MemoryCell {}

pub async fn npc_script_memory(mut api: NpcCtx) -> anyhow::Result<()> {
    use rand::seq::SliceRandom;
    api.wait_for_start().await?;

    // Generate board
    const N: usize = 4;
    const CARDS: usize = N * N;

    let mut cards = (0..CARDS / 2)
        .map(MemoryCell::from_ix)
        .cycle()
        .take(CARDS)
        .collect::<Vec<MemoryCell>>();
    cards.shuffle(&mut thread_rng());

    // Game loop
    let mut revealed = 0;
    let mut selected: Option<usize> = None;
    while revealed != CARDS {
        let ix = api.ask_grid("Memory:", &cards, N, N).await?;

        if cards[ix].revealed {
            continue;
        }

        match selected.take() {
            Some(last) if cards[last].item == cards[ix].item => {
                cards[ix].revealed = true;
                revealed += 2;
            }
            Some(last) => {
                cards[last].revealed = false;
                selected = Some(ix);
                cards[ix].revealed = true;
            }
            None => {
                selected = Some(ix);
                cards[ix].revealed = true;
            }
        }
    }

    api.say_end("You won").await?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct TaxiEntry(&'static str, &'static str, FieldId, Money);

impl std::fmt::Display for TaxiEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.2 == FieldId::NONE {
            write!(f, "Tell me more about the towns")
        } else {
            write!(
                f,
                "{}({}) - {} mesos",
                MapName(self.2).blue(),
                self.0,
                self.3.blue()
            )
        }
    }
}
impl ShroomDisplay for TaxiEntry {}

pub async fn npc_script_taxi(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    const TARGETS: [TaxiEntry; 7] = [
        TaxiEntry(
            "Henesys",
            "Alright I'll explain to you more about #bHenesys#k.",
            FieldId::HENESYS,
            100,
        ),
        TaxiEntry(
            "Kerning City",
            "Alright I'll explain to you more about #bKerning City#k.",
            FieldId::KERNING_CITY,
            100,
        ),
        TaxiEntry(
            "Ellinia",
            "Alright I'll explain to you more about #bEllinia#k.",
            FieldId::ELLINIA,
            100,
        ),
        TaxiEntry(
            "Perion",
            "Alright I'll explain to you more about #bPerion#k.",
            FieldId::PERION,
            100,
        ),
        TaxiEntry(
            "Lith Harbor",
            "The town you are at is Lith Harbor!",
            FieldId::LITH_HARBOUR,
            100,
        ),
        TaxiEntry(
            "Nautilus",
            "Here's a little information on #b#m120000000##k.",
            FieldId::NAUTILUS_HARBOR,
            100,
        ),
        TaxiEntry("Tell me about", "..", FieldId::NONE, 0),
    ];

    loop {
        let sel = api
            .ask_selection("Where do you want to go?!", TARGETS.to_vec().into())
            .await?;

        if !api.ask_yes_no("You sure").await? {
            return Ok(());
        }
        let target = &TARGETS[sel];

        // Tell me more about
        if target.2 == FieldId::NONE {
            let sel = api
                .ask_selection("There are 7 big towns here in Victoria Island. Which of those do you want to know more of?",
                 TARGETS.to_vec().into())
                .await?;

            //TODO use an array for a multi-step dialog
            api.say_next(TARGETS[sel].1).await?;
            // Restart the dialog
            continue;
        }

        // Else try to teleport
        if !api.try_take_money(target.3) {
            api.say_end("You don't have enough mesos").await?;
            return Ok(());
        }

        api.transfer_field(target.2)?;
        return Ok(());
    }
}

pub async fn npc_script_warrior(mut api: NpcCtx) -> anyhow::Result<()> {
    const JOB_ADV_LEVELS: [u8; 4] = [10, 30, 70, 120];

    api.wait_for_start().await?;

    let level = api.char_level();
    let job = api.job();

    if job == JobId::Beginner {
        if level < JOB_ADV_LEVELS[0] {
            api.say_end("You are not ready to make a job advancement yet.")
                .await?;
            return Ok(());
        }

        let sel = api
            .ask_yes_no("Would you like to become a Warrior?")
            .await?;
        if sel {
            api.set_job(JobId::Warrior);
            api.say_end("You are now a Warrior").await?;
        } else {
            api.say_end("Come back when you are ready").await?;
        }
        return Ok(());
    }

    if job.class() != JobClass::Warrior {
        api.say_end("You are not a Warrior").await?;
        return Ok(());
    }

    if job.is_max_level() {
        api.say_end("You are already a 4th job Warrior").await?;
        return Ok(());
    }

    let job_level = job.level();
    if level < JOB_ADV_LEVELS[job_level] {
        api.say_end("You are not ready to make a job advancement yet.")
            .await?;
        return Ok(());
    }

    if job == JobId::Warrior {
        loop {
            // offer choices
            let sel = api
                .ask_selection(
                    "Which warrior job advancement do you choose?",
                    vec!["Fighter", "Page", "Spearman"].into(),
                )
                .await?;

            let job = match sel {
                0 => JobId::Fighter,
                1 => JobId::Page,
                2 => JobId::Spearman,
                _ => unreachable!(),
            };

            let sel = api
                .ask_yes_no(format!(
                    "Would you like to make a job as a {:?} advancement?",
                    job
                ))
                .await?;

            if sel {
                api.set_job(job);
                api.say_end(format!("You have made a job advancement to {:?}", job))
                    .await?;
                return Ok(());
            }
        }
    }

    // 3rd,4th
    let job = job.next_jobs().next().context("Next job")?;

    let sel = api
        .ask_yes_no(format!(
            "Would you like to make a job as a {:?} advancement?",
            job
        ))
        .await?;

    if sel {
        api.set_job(job);
        api.say_end(format!("You have made a job advancement to {:?}", job))
            .await?;
        return Ok(());
    }

    let sel = api
        .ask_yes_no("Would you like to make a job advancement?")
        .await?;
    if !sel {
        api.say_end("Come back when you are ready").await?;
    }

    api.set_job(job);
    api.say_end(format!("You have made a job advancement to {:?}", job))
        .await?;

    Ok(())
}

pub async fn npc_guess_game(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    const MAX: u32 = 5;
    let number = rand::thread_rng().gen_range(1..MAX);
    let mut tries = 0;
    loop {
        let guess = api
            .ask_number(&format!("Guess the number up to {MAX}"), 1, MAX, 1)
            .await?;
        tries += 1;
        if guess == number {
            api.say_end(format!("You guessed the number in {} tries", tries))
                .await?;
            return Ok(());
        }
        if guess < number {
            api.say_next("Too low").await?;
        } else {
            api.say_next("Too high").await?;
        }
    }
}

const BOSS_IDS: [MobId; 5] = [
    MobId(100100),
    MobId(100101),
    MobId(100102),
    MobId(100103),
    MobId(100104),
];

pub async fn npc_boss_spawner(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    loop {
        let sel = api
            .ask_selection("Which boss do you want to spawn?", BOSS_IDS.to_vec().into())
            .await?;
        // TODO fix
        if sel == 5 {
            return Ok(());
        }

        let count = api.ask_number("How many?", 1, 10, 1).await?;
        for _ in 0..count {
            // TODO actual spawning
            println!("Spawned boss: {}", BOSS_IDS[sel])
        }
    }
}

pub async fn npc_style_change(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    loop {
        let sel = api
            .ask_selection(
                "Which style do you want to change to?",
                vec!["Hair", "Face", "Skin Color", "Exit"].into(),
            )
            .await?;

        match sel {
            0 => {
                let _hair = api.ask_number("Which hair?", 30000, 30050, 30000).await?;
                if api.ask_yes_no("You sure?").await? {
                    //api.change_hair(hair);
                }
            }
            1 => {
                let _face = api.ask_number("Which face?", 20000, 20030, 20000).await?;
                if api.ask_yes_no("You sure?").await? {
                    //api.change_face(face);
                }
            }
            2 => {
                let _color = api.ask_number("Which skin color?", 0, 3, 0).await?;
                if api.ask_yes_no("You sure?").await? {
                    //api.change_skin_color(color);
                }
            }
            _ => return Ok(()),
        }
    }
}

pub async fn npc_field_finder(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    loop {
        let search = api
            .ask_text(
                "Which field are you looking for?",
                1,
                16,
                "Kerning".to_string(),
            )
            .await?;
        let fields = api.search_fields(&search);

        if let Ok(fields) = fields {
            if api.ask_yes_no("Do you want to go to the field?").await? {
                api.transfer_field(fields)?;
                return Ok(());
            } else {
                continue;
            }
        }

        let options = fields.unwrap_err();
        if options.is_empty() {
            continue;
        }
        const PAGE_ELEMS: usize = 5;
        let mut page_ix = 0;
        let pages = options.len() / PAGE_ELEMS;

        loop {
            let mut cur_page = options
                .iter()
                .map(|p| p.1.as_str())
                .skip(page_ix * PAGE_ELEMS)
                .take(PAGE_ELEMS)
                .collect::<Vec<_>>();
            let n = cur_page.len();
            cur_page.push("Next");
            cur_page.push("Prev");

            let sel = api
                .ask_selection("Which field are you looking for?", cur_page.into())
                .await?;
            if sel < n {
                let field = &options[page_ix * PAGE_ELEMS + sel];
                if api.ask_yes_no("Do you want to go to the field?").await? {
                    api.transfer_field(field.0)?;
                    api.say_end("Transfering...").await?;
                    return Ok(());
                }
            }

            match sel - n {
                0 => {
                    if page_ix < pages {
                        page_ix += 1;
                    }
                }
                1 => {
                    page_ix = page_ix.saturating_sub(1);
                }
                _ => unreachable!("invalid option: {sel}"),
            }
        }
    }
}
