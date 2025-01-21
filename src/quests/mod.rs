use crate::QuestMap;
use std::collections::HashMap;

pub mod blacksmith;
pub mod activity;
pub mod socialize;
pub mod block_break_simple;
pub mod block_mine_hard;
pub mod block_mine_simple;
pub mod timber;
pub mod crazy;

pub fn quests() -> QuestMap {
    let mut quest_map: QuestMap = HashMap::new();

    quest_map.insert(
        "blacksmith",
        Box::new(blacksmith::generate_blacksmith_quest),
    );

    quest_map.insert(
        "activity",
        Box::new(activity::activity_quest),
    );

    quest_map.insert(
        "socialize",
        Box::new(socialize::socialize_quest),
    );

    quest_map.insert(
        "block_break_simple",
        Box::new(block_break_simple::block_break_simple_quest),
    );

    quest_map.insert(
        "block_mine_hard",
        Box::new(block_mine_hard::block_mine_hard_quest),
    );

    quest_map.insert(
        "block_mine_simple",
        Box::new(block_mine_simple::block_mine_simple_quest),
    );

    quest_map.insert(
        "timber",
        Box::new(timber::timber_quest),
    );

    quest_map.insert(
        "crazy",
        Box::new(crazy::crazy_quest),
    );

    quest_map
}
