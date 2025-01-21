use rand::seq::SliceRandom;

use crate::{constants::simple_blocks::SIMPLE_BLOCKS, Item, Quest, BASE_QUEST_POINTS};

pub fn block_break_simple_quest(
    multiplier: f64
) -> Quest {
    let mut rng = rand::thread_rng();
    let block = SIMPLE_BLOCKS
        .choose(&mut rng)
        .unwrap_or(&"stone") // Default to "stone" if `simple_blocks` is empty.
        .to_string();

    let target = (multiplier * 64.0).round() as u32;
    let adjusted_multiplier = target as f64 / 64.0;

    let reward = std::cmp::max(1, (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 2.0).round() as u32);

    Quest {
        quest_type: "block-break".to_string(),
        variable: block.clone(),
        name: "Block Kırma".to_string(),
        required_progress: target,
        points: reward,
        item: Item {
            material: block.clone(),
            name: "Block Kırma".to_string(),
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
