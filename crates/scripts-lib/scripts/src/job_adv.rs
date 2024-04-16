use num_enum::{IntoPrimitive, TryFromPrimitive};
use shroom_meta::{
    fmt::ShroomMenuList,
    id::{job_id::JobId, FieldId, ItemId, QuestId},
    QuestDataId,
};
use shroom_script::npc::{EnumQuestData, NpcCtx};

const FINAL: ItemId = ItemId(4031012);
const LETTER: ItemId = ItemId(4031008);

const JOB2_INSTR_FIELD: FieldId = FieldId(102020300);
const JOB2_FIELD: FieldId = FieldId(910230000);

const JOB3_BOSS_ITEM: ItemId = ItemId(4031059);
const JOB3_MENTAL_ITEM: ItemId = ItemId(4031058);
const JOB3_APPROVAL_ITEM: ItemId = ItemId(4031057);

const JOB3_MIRROR_FIELD: FieldId = FieldId(105030500);
const JOB3_BOSS_FIELD: FieldId = FieldId(910540100);

#[derive(Debug, Default, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SecondJobQuestState {
    #[default]
    Initial = 0,
    InstructorNpc = 1,
    MarbleTest = 2,
    MarbleTestPassed = 3,
    Finished = 4,
}

impl EnumQuestData for SecondJobQuestState {
    const ID: shroom_meta::QuestDataId = QuestDataId(100004);
}

#[derive(Debug, Default, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum ThirdJobQuestState {
    // Job Chief
    #[default]
    Initial = 0,
    // Job instructor allowance
    JobNpc = 1,
    // Allow Mirror to warp
    PhysicalTest = 2,
    // Physical test passed
    PhysicalTestPassed = 3,
    // Mental test alloewd
    MentalTest = 4,
    // Mental test passed
    MentalTestPassed = 5,
    // Finished
    Finished = 6,
}

impl EnumQuestData for ThirdJobQuestState {
    const ID: shroom_meta::QuestDataId = QuestDataId(100005);
}

/*
Mirror:
1061009
1061010
*/

const DARK_MARBLE: ItemId = ItemId(4031013);
const MARBLE_REQ: usize = 5;

pub async fn npc_script_warrior(mut api: NpcCtx) -> anyhow::Result<()> {
    const JOB_ADV_LEVELS: [u8; 4] = [10, 30, 70, 120];
    api.wait_for_start().await?;

    let level = api.char_level();
    let job = api.job();

    if job == JobId::Beginner {
        api.say_next(format!(
            "Do you want to become a #rwarrior#k? You need to meet some criteria in order to do so.#b You should be at least in level 10, and at least {}#k. Let's see...", "STR 35")).await?;

        // TODO check str
        if level < JOB_ADV_LEVELS[0] {
            api.say_end("Train a bit more until you reach the base requirements and I can show you the way of the #rWarrior#k.")
                .await?;
            return Ok(());
        }

        // Try to give starter set
        if !api.try_give_item(ItemId(1302077), 1)? {
            api.say_end("Make some room in your inventory and talk back to me.")
                .await?;
            return Ok(());
        }
        api.set_job(JobId::Warrior);
        api.say_next("From here on out, you are going to the Warrior path. This is not an easy job, but if you have discipline and confidence in your own body and skills, you will overcome any difficulties in your path. Go, young Warrior!").await?;
        api.say_next("You've gotten much stronger now. Plus every single one of your inventories have added slots. A whole row, to be exact. Go see for it yourself. I just gave you a little bit of #bSP#k. When you open up the #bSkill#k menu on the lower left corner of the screen, there are skills you can learn by using SP's. One warning, though: You can't raise it all together all at once. There are also skills you can acquire only after having learned a couple of skills first.").await?;
        api.say_end("Now a reminder. Once you have chosen, you cannot change up your mind and try to pick another path. Go now, and live as a proud Warrior.").await?;
        return Ok(());
    }

    if job == JobId::Warrior && level >= JOB_ADV_LEVELS[0] {
        let state = api.get_or_default_quest_data::<SecondJobQuestState>()?;

        // Either start or letter lost
        if state == SecondJobQuestState::Initial
            || (state == SecondJobQuestState::MarbleTest && !api.has_item(LETTER))
        {
            if !api.try_give_items(&[(LETTER, 1)])? {
                api.say_end("Please, make some space in your inventory.")
                    .await?;
                return Ok(());
            }
            api.update_quest_data(SecondJobQuestState::InstructorNpc)?;

            api.say_end("Please get this letter to #b#p1072000##k who's around #b#m102020300##k near Perion. He is taking care of the job of an instructor in place of me. Give him the letter and he'll test you in place of me. Best of luck to you.").await?;
            return Ok(());
        }

        // Marble test is pending
        if state == SecondJobQuestState::MarbleTest {
            api.say_end("Go and see the #b#p1072000##k.").await?;
            return Ok(());
        }

        // Marble test passed
        if state == SecondJobQuestState::MarbleTestPassed && api.has_item(FINAL) {
            let fighter_desc = "Warriors that master #rSwords or Axes#k.\r\n\r\n#rFighters#k get #bRage#k, which boosts your party's weapon attack by 10. During 2nd job this is strongly appreciated, as it is free (except for -10 wep def, which is not going to impact the damage you take much at all), takes no Use slots and increases each party member's damage (except Mages) by several hundreds. The other classes can give themselves a weapon attack boost as well, but need items to do so. #rFighters#k also get #bPower Guard#k, reducing touch damage by 40% and deals it back to the monster. This is the main reason why #rFighters#k are considered soloers is because this reduces pot costs immensely.";
            let page_desc = "Warriors that master #rPolearms or Spears#k.\r\n\r\n#rPage#k get #bThreaten#k, which increases the monster's aggro on you. This is useful for partying, as you can keep the monsters off of your party members. #rPage#k also get #bPower Guard#k, reducing touch damage by 40% and deals it back to the monster. This is the main reason why #rPages#k are considered soloers is because this reduces pot costs immensely.";
            let spearman_desc = "Warriors that master #rPolearms or Spears#k.\r\n\r\n#rSpearman#k get #bHyper Body#k, which increases your max HP by 60% and your max MP by 30%. This is useful for partying, as you can keep the monsters off of your party members. #rSpearman#k also get #bIron Will#k, which increases your max HP by 20% and your max MP by 10%. This is useful for partying, as you can keep the monsters off of your party members.";

            let sel = api.ask_selection("Now... have you made up your mind? Please choose the job you'd like to select for your 2nd job advancement.", ShroomMenuList::new(vec![fighter_desc, page_desc, spearman_desc], shroom_meta::fmt::MenuStyle::List)).await?;

            match sel {
                0 => {
                    api.try_take_item(FINAL, 1)?;
                    api.set_job(JobId::Fighter);
                    api.say_next("You have chosen the path of the #rFighter#k. The #rFighter#k is a class that specializes in close combat. They are known for their high HP and STR, and are able to use powerful weapons and armor. They are also able to use the #bRage#k skill, which increases the weapon attack of all party members by 10. The #bPower Guard#k skill reduces touch damage by 40% and deals it back to the monster. This is the main reason why #rFighters#k are considered soloers is because this reduces pot costs immensely.").await?;
                }
                1 => {
                    api.try_take_item(FINAL, 1)?;
                    api.set_job(JobId::Page);
                    api.say_next("You have chosen the path of the #rPage#k. The #rPage#k is a class that specializes in close combat. They are known for their high HP and STR, and are able to use powerful weapons and armor. They are also able to use the #bThreaten#k skill, which increases the monster's aggro on you. This is useful for partying, as you can keep the monsters off of your party members. The #bPower Guard#k skill reduces touch damage by 40% and deals it back to the monster. This is the main reason why #rPages#k are considered soloers is because this reduces pot costs immensely.").await?;
                }
                2 => {
                    api.try_take_item(FINAL, 1)?;
                    api.set_job(JobId::Spearman);
                    api.say_next("You have chosen the path of the #rSpearman#k. The #rSpearman#k is a class that specializes in close combat. They are known for their high HP and STR, and are able to use powerful weapons and armor. They are also able to use the #bHyper Body#k skill, which increases your max HP by 60% and your max MP by 30%. This is useful for partying, as you can keep the monsters off of your party members. The #bIron Will#k skill increases your max HP by 20% and your max MP by 10%. This is useful for partying, as you can keep the monsters off of your party members.").await?;
                }
                _ => unreachable!(),
            }

            api.say_end("k BYE!").await?;
            return Ok(());
        }
    }

    if job.level() == 2 && level >= JOB_ADV_LEVELS[2] {
        let state: ThirdJobQuestState = api.get_or_default_quest_data()?;

        if state == ThirdJobQuestState::JobNpc {
            api.set_quest_data(ThirdJobQuestState::PhysicalTest)?;
            api.say_end("I was waiting for you. Few days ago, I heard about you from #b#p2020008##k in Ossyria. Well... I'd like to test your strength. There is a secret passage near the ant tunnel. Nobody but you can go into that passage. If you go into the passage, you will meat my the other self. Beat him and bring #b#t4031059##k to me.").await?;
            return Ok(());
        }

        if state == ThirdJobQuestState::PhysicalTest {
            if !api.has_item(JOB3_BOSS_ITEM) {
                api.say_end("My the other self is quite strong. He uses many special skills and you should fight with him 1 on 1. However, people cannot stay long in the secret passage, so it is important to beat him ASAP. Well... Good luck I will look forward to you bringing #b#t4031059##k to me.").await?;
            }

            api.try_take_all_items(JOB3_BOSS_ITEM)?;
            api.try_give_item(JOB3_APPROVAL_ITEM, 1)?;
            api.set_quest_data(ThirdJobQuestState::PhysicalTestPassed)?;
            api.say_end("Wow... You beat my the other self and brought #b#t4031059##k to me. Good! this surely proves your strength. In terms of strength, you are ready to advance to 3th job. As I promised, I will give #b#t4031057##k to you. Give this necklace to #b#p2020008##k in Ossyria and you will be able to take second test of 3rd job advancement. Good Luck~").await?;
            return Ok(());
        }
    }

    api.say_end("You have chosen wisely.").await?;
    Ok(())
}

pub async fn npc_script_warrior2(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    let mut state = api.get_or_default_quest_data::<SecondJobQuestState>()?;

    if state == SecondJobQuestState::InstructorNpc {
        if !api.has_item(LETTER) {
            api.say_end("You're truly a hero!").await?;
            return Ok(());
        }

        api.try_take_item(LETTER, 1)?;
        state = api.update_quest_data(SecondJobQuestState::MarbleTest)?;
    }

    if state == SecondJobQuestState::MarbleTestPassed && !api.has_item(FINAL) {
        // If item was reset
        state = api.update_quest_data(SecondJobQuestState::MarbleTest)?;
    }

    if state != SecondJobQuestState::MarbleTest {
        api.say_end("You're truly a hero!").await?;
        return Ok(());
    }

    api.say_next("I'll send you to a hidden map. You'll see monsters you don't normally see. They look the same like the regular ones, but with a totally different attitude. They neither boost your experience level nor provide you with item.").await?;
    api.say_next("You'll be able to acquire a marble called #b#t4031013##k while knocking down those monsters. It is a special marble made out of their sinister, evil minds. Collect 30 of those, and then go talk to a colleague of mine in there. That's how you pass the test.").await?;
    api.say_next("Once you go inside, you can't leave until you take care of your mission. If you die, your experience level will decrease..so you better really buckle up and get ready...well, do you want to go for it now?").await?;
    api.say_next("Alright I'll let you in! Defeat the monsters inside, collect 30 Dark Marbles, then strike up a conversation with a colleague of mine inside. He'll give you #bThe Proof of a Hero#k, the proof that you've passed the test. Best of luck to you.").await?;
    api.transfer_field(JOB2_FIELD)?;

    Ok(())
}

pub async fn npc_script_warrior2_inside(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    if !api.has_item_quantity(DARK_MARBLE, MARBLE_REQ) {
        // TODO allow leave option
        api.say_end("You will have to collect me #b30 #t4031013##k. Good luck.")
            .await?;
        return Ok(());
    }

    api.try_take_all_items(DARK_MARBLE)?;
    api.try_give_item(FINAL, 1)?;
    api.set_quest_data(SecondJobQuestState::MarbleTestPassed)?;
    api.say_end("Ohhhhh.. you collected all 30 Dark Marbles!! It should have been difficult... just incredible! Alright. You've passed the test and for that, I'll reward you #bThe Proof of a Hero#k. Take that and go back to Perion.").await?;
    api.transfer_field(JOB2_INSTR_FIELD)?;

    Ok(())
}

pub async fn npc_script_warrior_chief(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    let state: ThirdJobQuestState = api.get_or_default_quest_data()?;

    if state == ThirdJobQuestState::Initial {
        let res = api.ask_yes_no("Welcome. I'm #b#p2020008##k, the chief of all warriors, in charge of bringing out the best in each and every warrior that needs my guidance. You seem like the kind of warrior that wants to make the leap forward, the one ready to take on the challenges of the 3th job advancement. But I've seen countless warriors eager to make the jump just like you, only to see them fail. What about you? Are you ready to be tested and make the 3th job advancement?").await?;
        if res {
            api.update_quest_data(ThirdJobQuestState::JobNpc)?;
            api.say_end("Good. You will be tested on two important aspects of the warrior: strength and wisdom. I'll now explain to you the physical half of the test. Remember #b#p1022000##k from Perion? Go see him, and he'll give you the details on the first half of the test. Please complete the mission, and get #b#t4031057##k from #p1022000#.").await?;
        } else {
            api.say_end("Make up your mind.").await?;
        }
        return Ok(());
    }

    if state == ThirdJobQuestState::JobNpc || state == ThirdJobQuestState::PhysicalTest {
        api.say_end("Go, talk with #b#p1022000##k and bring me #b#t4031057##k.")
            .await?;
    }

    if state == ThirdJobQuestState::PhysicalTestPassed && api.has_item(JOB3_APPROVAL_ITEM) {
        if !api.has_item(JOB3_APPROVAL_ITEM) {
            api.say_end("Go, talk with #b#p1022000##k and bring me #b#t4031057##k.")
                .await?;
            return Ok(());
        }
        api.try_take_all_items(JOB3_APPROVAL_ITEM)?;
        api.update_quest_data(ThirdJobQuestState::MentalTest)?;

        api.say_next("Here's the 2nd half of the test. This test will determine whether you are smart enough to take the next step towards greatness. There is a dark, snow-covered area called the Holy Ground at the snowfield in Ossyria, where even the monsters can't reach. On the center of the area lies a huge stone called the Holy Stone. You'll need to offer a special item as the sacrifice, then the Holy Stone will test your wisdom right there on the spot.").await?;
        api.say_end("You'll need to answer each and every question given to you with honesty and conviction. If you correctly answer all the questions, then the Holy Stone will formally accept you and hand you #b#t4031058##k. Bring back the necklace, and I will help you to the next step forward. Good luck.").await?;
        return Ok(());
    }

    if state == ThirdJobQuestState::MentalTest {
        api.say_end("Go, talk with #b#p2030006##k and bring me #b#t4031058##k.")
            .await?;
        return Ok(());
    }

    if state == ThirdJobQuestState::MentalTestPassed {
        if !api.has_item(JOB3_MENTAL_ITEM) {
            api.say_end("Go, talk with #b#p2030006##k and bring me #b#t4031058##k.")
                .await?;
            return Ok(());
        }

        api.try_take_all_items(JOB3_MENTAL_ITEM)?;
        api.say_next("Great job completing the mental part of the test. You have wisely answered all the questions correctly. I must say, I am quite impressed with the level of wisdom you have displayed there. Please hand me the necklace first, before we takeon the next step.").await?;
        api.say_next("Okay! Now, you'll be transformed into a much more powerful warrior through me. Before doing that, though, please make sure your SP has been thoroughly used, You'll need to use up at least all of SP's gained until level 70 to make the 3rd job advancement. Oh, and since you have already chosen your path of the occupation by the 2nd job adv., you won't have to choose again for the 3rd job adv. Do you want to do it right now?").await?;

        let next = api.job().next_jobs().next().unwrap();
        api.set_job(next);
        let desc = match next {
            JobId::Crusader => "You have just become the #bCrusader#k. A number of new attacking skills such as #bShout#k and #bCombo Attack#k are devastating, while #bArmor Crash#k will put a dent on the monsters' defensive abilities. It'll be best to concentrate on acquiring skills with the weapon you mastered during the days as a Fighter.",
            JobId::WhiteKnight => "You have just become the #bWhite Knight#k. You'll be introduced to a new skill book featuring various new attacking skills as well as element-based attacks. It's recommended that the type of weapon complementary to the Page, whether it be a sword or a blunt weapon, should be continued as the White Knight. There's a skill called #bCharge#k, which adds an element of ice, fire and lightning to the weapon, making White Knight the only warrior that can perform element-based attacks. Charge up your weapon with an element that weakens the monster, and then apply massive damage with the #bCharged Blow#k. This will definitely make you a devastating force around here.",
            JobId::DragonKnight => "You're #bDragon Knight#k from here on out. You'll be introduced to a range of new attacking skills for spears and pole arms, and whatever weapon was chosen as the Spearman should be continued as the Dragon Knigth. Skills such as #bCrusher#k (maximum damage to one monster) and #bDragon Fury#k (damage to multiple monsters) are recommended as main attacking skills of choice, while a skill called #bDragon Roar#k will damage everything on screen with devasting force. The downside is the fact that the skill uses up over half of the available HP.",
            _ => unreachable!()
        };

        api.say_next(desc).await?;
        api.say_end("I've also given you some SP and AP, which will help you get started. You have now become a powerful, powerful warrior, indeed. Remember, though, that the real world will be awaiting your arrival with even tougher obstacles to overcome. Once you feel like you cannot train yourself to reach a higher place, then, and only then, come see me. I'll be here waiting.").await?;
        return Ok(());
    }

    api.say_end("Make up your mind.").await?;
    Ok(())
}

pub async fn npc_script_mirror(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    let state = api.get_or_default_quest_data::<ThirdJobQuestState>()?;
    if state == ThirdJobQuestState::PhysicalTest {
        api.transfer_field(JOB3_BOSS_FIELD)?;
        return Ok(());
    }

    api.say_end("...")
        .await?;

    Ok(())
}

pub async fn npc_script_mirror_inside(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;
    api.transfer_field(JOB3_MIRROR_FIELD)?;

    Ok(())
}

pub async fn npc_script_holy_stone(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    let state: ThirdJobQuestState = api.get_or_default_quest_data()?;
    if state == ThirdJobQuestState::MentalTest
        || (state == ThirdJobQuestState::MentalTestPassed && !api.has_item(JOB3_MENTAL_ITEM))
    {
        api.say_next("Alright... I'll be testing out your wisdom here. Answer all the questions correctly, and you will pass the test BUT, if you even lie to me once, then you'll have to start over again ok, here we go.").await?;
        if !api.try_give_item(JOB3_MENTAL_ITEM, 1)? {
            api.say_end("Make some room in your inventory and talk back to me.")
                .await?;
            return Ok(());
        }
        api.update_quest_data(ThirdJobQuestState::MentalTestPassed)?;
        //TODO quiz
        api.say_end("Passed mental test").await?;
        return Ok(());
    }


    api.say_end("You have chosen wisely.").await?;
    Ok(())
}

pub async fn npc_script_priest(mut api: NpcCtx) -> anyhow::Result<()> {
    api.wait_for_start().await?;

    if api.job().level() == 3 && api.has_completed_quest(QuestId(6904)) {
        let next = api.job().next_jobs().next().unwrap();
        api.set_job(next);
        api.say_end("You have job advanced").await?;
        return Ok(());
    }

    api.say_end("???").await?;
    Ok(())
}
