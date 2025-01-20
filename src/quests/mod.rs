use crate::QuestMap;
use std::collections::HashMap;

pub mod blacksmith;

pub fn quests() -> QuestMap {
    let mut quest_map: QuestMap = HashMap::new();

    quest_map.insert(
        "blacksmith",
        Box::new(blacksmith::generate_blacksmith_quest),
    );

    quest_map
}
