use rand::seq::SliceRandom;
use serde::Serialize;

use crate::{Item, Quest};

const BASE_QUEST_POINTS: f64 = 100.0; // Replace this with the actual value from your config

pub fn generate_blacksmith_quest(multiplier: f64) -> Quest {
    let types = ["sword", "pickaxe", "axe", "armor"];

    let lores = [
        (
            "sword",
            "&7&l%target% &7adet &6Herhangi bir Kılıç &7üretmeniz gerekiyor.",
        ),
        (
            "pickaxe",
            "&7&l%target% &7adet &6Herhangi bir Kazma &7üretmeniz gerekiyor.",
        ),
        (
            "axe",
            "&7&l%target% &7adet &6Herhangi bir Balta &7üretmeniz gerekiyor.",
        ),
        (
            "armor",
            "&7&l%target% &7adet &6Herhangi bir Zırh &7üretmeniz gerekiyor.",
        ),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<_, _>>();

    let items = [
        ("sword", vec!["iron_sword", "golden_sword", "diamond_sword"]),
        (
            "pickaxe",
            vec!["iron_pickaxe", "golden_pickaxe", "diamond_pickaxe"],
        ),
        ("axe", vec!["iron_axe", "golden_axe", "diamond_axe"]),
        (
            "armor",
            vec![
                "iron_helmet",
                "golden_helmet",
                "diamond_helmet",
                "iron_chestplate",
                "golden_chestplate",
                "diamond_chestplate",
                "iron_leggings",
                "golden_leggings",
                "diamond_leggings",
                "iron_boots",
                "golden_boots",
                "diamond_boots",
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<_, _>>();

    let target = (multiplier * 4.0).round() as u32;
    let adjusted_multiplier = target as f64 / 4.0;
    let reward = f64::max(1.0, (BASE_QUEST_POINTS * adjusted_multiplier * 3.0).round()) as u32;

    let quest_type = types.choose(&mut rand::thread_rng()).unwrap();
    let variable = items.get(quest_type).unwrap().join(" OR ");
    let lore_text = lores
        .get(quest_type)
        .unwrap()
        .replace("%target%", &target.to_string());

    Quest {
        quest_type: "craft".to_string(),
        variable,
        name: "Demircilik".to_string(),
        required_progress: target,
        points: reward,
        item: Item {
            material: "anvil".to_string(),
            name: "Demircilik".to_string(),
            lore: vec![
                "&7Bu görevi tamamlamak için".to_string(),
                lore_text,
                "".to_string(),
                "&7(&a%percentage_progress%&7)".to_string(),
                format!("&7> &f&l{} &7Puan", reward),
            ],
        },
        exclusive: None,
    }
}
