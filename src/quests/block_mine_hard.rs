use rand::seq::SliceRandom;

use crate::{constants::rare_mine_blocks::RARE_BLOCK_MINE_BLOCKS, Item, Quest, BASE_QUEST_POINTS};

pub fn block_mine_hard_quest(multiplier: f64) -> Quest {
    let mut rng = rand::thread_rng();
    let block = RARE_BLOCK_MINE_BLOCKS
        .choose(&mut rng)
        .unwrap_or(&"diamond_ore") 
        .to_string();

    let target = (multiplier * 16.0).round() as u32;
    let adjusted_multiplier = target as f64 / 16.0;

    let reward = std::cmp::max(1, (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 3.0).round() as u32);

    Quest {
        quest_type: "block-break".to_string(),
        variable: block.clone(),
        name: "Usta Madencilik".to_string(),
        required_progress: target,
        points: reward,
        item: Item {
            material: "iron_pickaxe".to_string(),
            name: "Usta Madencilik".to_string(),
            lore: vec![
                "§7Bu görevi tamamlamak için".to_string(),
                format!("§7§l{} §7adet §6{} §7kazmanız gerekiyor.", target, block),
                "".to_string(),
                "§7(§a%percentage_progress%§7)".to_string(),
                format!("§7> §f§l{} §7Puan", reward),
            ],
        },
        exclusive: None,
        special_progress: None,
    }
}
