use crate::{Item, Quest, BASE_QUEST_POINTS};

pub fn activity_quest(multiplier: f64) -> Quest {
    let login_size = (multiplier * 10.0).round() as u32;
    let adjusted_multiplier = login_size as f64 / 10.0;

    let name = "Giriş Yapma".to_string();
    let reward = std::cmp::max(1, (BASE_QUEST_POINTS as f64 * adjusted_multiplier * 1.5).round() as u32);

    Quest {
        quest_type: "login".to_string(),
        variable: "none".to_string(),
        name: name.clone(),
        required_progress: login_size,
        points: reward,
        item: Item {
            material: "clock".to_string(),
            name: name.clone(),
            lore: vec![
                "&7Bu görevi tamamlamak için".to_string(),
                format!("&7&l{} &7kere giriş yapmanız gerekiyor.", login_size),
                "".to_string(),
                "&7(&a%percentage_progress%&7)".to_string(),
                format!("&7> &f&l{} &7Puan", reward),
            ],
        },
        special_progress: None,
        exclusive: None,
    }
}
