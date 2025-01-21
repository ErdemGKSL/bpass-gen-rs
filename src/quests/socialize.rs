use crate::{Item, Quest, BASE_QUEST_POINTS};

pub fn socialize_quest(multiplier: f64) -> Quest {
    let message_size = (multiplier * 300.0).round() as u32;
    let adjusted_multiplier = message_size as f64 / 300.0;

    let name = "Sosyalleş".to_string();
    let reward = std::cmp::max(1, (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 2.0).round() as u32);

    Quest {
        quest_type: "chat".to_string(),
        variable: "none".to_string(),
        name: name.clone(),
        required_progress: message_size,
        points: reward,
        item: Item {
            material: "paper".to_string(),
            name: name.clone(),
            lore: vec![
                "&7Bu görevi tamamlamak için".to_string(),
                format!("&7&l{} &7adet mesaj yazmanız gerekiyor.", message_size),
                "".to_string(),
                "&7(&a%percentage_progress%&7)".to_string(),
                format!("&7> &f&l{} &7Puan", reward),
            ],
        },
        special_progress: None,
        exclusive: None,
    }
}
