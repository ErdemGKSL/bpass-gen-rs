use rand::seq::SliceRandom;

use crate::{constants::logs::LOGS, Item, Quest, BASE_QUEST_POINTS};

pub fn timber_quest(multiplier: f64) -> Quest {

    let block = LOGS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"oak_log");

    let target = (multiplier * 64.0).round() as u32;
    let adjusted_multiplier = target as f64 / 64.0;
    let reward = (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 2.0).round().max(1.0) as u32;

    Quest {
        quest_type: "block-break".to_string(),
        variable: block.to_string(),
        name: "Ağaç Kırma".to_string(),
        required_progress: target,
        points: reward,
        item: Item {
            material: block.to_string(),
            name: "Ağaç Kırma".to_string(),
            lore: vec![
                "&7Bu görevi tamamlamak için".to_string(),
                format!("&7&l{} &7adet &6{} &7kırmanız gerekiyor.", target, block),
                "".to_string(),
                "&7(&a%percentage_progress%&7)".to_string(),
                format!("&7> &f&l{} &7Puan", reward),
            ],
        },
        exclusive: None,
        special_progress: None,
    }
}