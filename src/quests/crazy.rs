use rand::seq::SliceRandom;
use std::cmp::max;

use crate::{constants::crazy_sentences::CRAZY_SENTENCES, Item, Quest, BASE_QUEST_POINTS};

pub fn crazy_quest(multiplier: f64) -> Quest {
    let sentence = CRAZY_SENTENCES
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Kebap salonunu yerim"); // Fallback to the first sentence

    let message_size = (multiplier * 5.0).round() as u32;
    let adjusted_multiplier = message_size as f64 / 5.0;

    let reward = max(1, (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 2.0).round() as u32);

    Quest {
        quest_type: "chat-stripped".to_string(),
        variable: sentence.to_string(),
        name: "Delilik".to_string(),
        required_progress: message_size,
        points: reward,
        item: Item {
            material: "paper".to_string(),
            name: "Delilik".to_string(),
            lore: vec![
                "&7Bu görevi tamamlamak için sohbete".to_string(),
                format!(
                    "&7&l{} &7adet &6\"{}\" &7yazmanız gerekiyor.",
                    message_size, sentence
                ),
                "".to_string(),
                "&7(&a%percentage_progress%&7)".to_string(),
                format!("&7> &f&l{} &7Puan", reward),
            ],
        },
        exclusive: None,
        special_progress: None,
    }
}