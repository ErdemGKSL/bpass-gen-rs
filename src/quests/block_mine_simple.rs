use rand::seq::SliceRandom;

use crate::{constants::mine_blocks::MINE_BLOCKS, Item, Quest, BASE_QUEST_POINTS};

pub fn block_mine_simple_quest(multiplier: f64) -> Quest {
  let mut rng = rand::thread_rng();
  let block = MINE_BLOCKS
      .choose(&mut rng)
      .unwrap_or(&"stone") // Default to "stone" if no block is chosen.
      .to_string();

  let block_multiplier = if block.clone() == "stone" { 8.0 } else { 1.0 };
  let target = (multiplier * 24.0 * block_multiplier).round() as u32;
  let adjusted_multiplier = target as f64 / (24.0 * block_multiplier);

  let reward = std::cmp::max(1, (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 2.0).round() as u32);

  Quest {
      quest_type: "block-break".to_string(),
      variable: block.clone(),
      name: "Temel Madencilik".to_string(),
      required_progress: target,
      points: reward,
      item: Item {
          material: "wooden_pickaxe".to_string(),
          name: "Temel Madencilik".to_string(),
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