use quests::quests;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Mapping};
use std::cmp::min;
use std::collections::HashMap;
use std::fs;

pub mod quests;
pub mod constants;

const BASE_QUEST_POINTS: f64 = 100.0;

// Mock `quests` and `config` for this example
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    name: String,
    variable: String,
    exclusive: Option<String>,
    #[serde(rename = "type")]
    quest_type: String,
    required_progress: u32,
    points: u32,
    item: Item,
    #[serde(rename = "special-progress")]
    special_progress: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    material: String,
    name: String,
    lore: Vec<String>,
}

#[derive(Debug)]
struct Config {
    daily_quest_size: usize,
    week_quests_size: usize,
    randomiser: u32,
    max_page: u32,
    free_reward_interval: u32,
    last_free_reward: u32,
    last_premium_reward: u32,
    premium_reward_chance: f64,
    premium_xp_multiplier: f64,
    tier_xp_calculator: fn(usize) -> u32,
}
pub type QuestCallback = Box<dyn Fn(f64) -> Quest>;
pub type QuestMap = HashMap<&'static str, QuestCallback>;

fn config() -> Config {
    Config {
        daily_quest_size: 14,
        week_quests_size: 7,
        randomiser: 2,
        max_page: 7,
        free_reward_interval: 3,
        last_free_reward: 5,
        last_premium_reward: 5,
        premium_reward_chance: 25.0,
        premium_xp_multiplier: 1.5,
        tier_xp_calculator: |tier| (tier as u32 + 1) * 100,
    }
}

fn generate_quests(
    quest_size: usize,
    randomise: bool,
    randomiser: u32,
    quests_map: &QuestMap,
) -> Vec<Quest> {
    let mut result = HashMap::new();
    let mut rng = rand::thread_rng();

    while result.len() < quest_size {
        let multiplier = if randomise {
            rng.gen_range(1.0..=(randomiser as f64))
        } else {
            1.0
        };
        let quest_func = quests_map
            .values()
            .nth(rng.gen_range(0..quests_map.len()))
            .unwrap();
        let mut quest = quest_func(multiplier);

        if randomise && result.len() >= 7 {
            quest.exclusive = Some("premium".to_string());
        }

        result.insert(format!("{}{}", quest.name, quest.variable), quest);
    }

    result.into_values().collect()
}

fn write_yaml<T: Serialize>(path: &str, data: &T) {
    let yaml_data = serde_yaml::to_string(data).expect("Failed to serialize to YAML");
    fs::write(path, yaml_data).expect("Failed to write YAML file");
}

fn main() {
    let config = config();
    let quests_map: QuestMap = quests();
    let mut quest_id_counter = 0;

    // Clear and prepare directories
    let output_quests_dir = "./output/quests";
    let output_passes_dir = "./output/passes";
    fs::remove_dir_all(output_quests_dir).ok();
    fs::remove_dir_all(output_passes_dir).ok();
    fs::create_dir_all(output_quests_dir).expect("Failed to create quests directory");
    fs::create_dir_all(output_passes_dir).expect("Failed to create passes directory");

    // Daily quests
    println!("Generating daily quests...");
    let daily_quests = generate_quests(
        min(quests_map.len(), config.daily_quest_size),
        false,
        config.randomiser,
        &quests_map,
    );

    let daily_quests: HashMap<u32, Quest> = daily_quests
        .into_iter()
        .map(|quest| {
            quest_id_counter += 1;
            (quest_id_counter, quest)
        })
        .collect();

    write_yaml(
        &format!("{}/daily-quests.yml", output_quests_dir),
        &daily_quests,
    );

    // Weekly quests
    println!("Generating weekly quests...");
    for i in 0..config.week_quests_size {
        let weekly_quests = generate_quests(
            min(quests_map.len(), 14),
            true,
            config.randomiser,
            &quests_map,
        );

        let weekly_quests: HashMap<u32, Quest> = weekly_quests
            .into_iter()
            .map(|quest| {
                quest_id_counter += 1;
                (quest_id_counter, quest)
            })
            .collect();

        write_yaml(
            &format!("{}/week-{}-quests.yml", output_quests_dir, i + 1),
            &weekly_quests,
        );
    }

    // Passes
    println!("Generating passes...");
    let last_tier = (config.max_page * 9) as usize - 1;

    let mut free_pass = vec![];
    let mut premium_pass: Vec<Mapping> = vec![];

    let free_pass_offset = rand::thread_rng().gen_range(0..config.free_reward_interval);

    for i in 0..=last_tier {
        let required_points = (config.tier_xp_calculator)(i);

        // Free pass
        if (i as u32 % config.free_reward_interval) == free_pass_offset || i == last_tier {
            let reward = rand::thread_rng().gen_range(1..=config.last_free_reward);
            free_pass.push(serde_yaml::Value::from(serde_yaml::Mapping::from_iter([
                ("required-points".into(), required_points.into()),
                ("rewards".into(), vec![reward.to_string()].into()),
            ])));
        } else {
            free_pass.push(serde_yaml::Value::from(serde_yaml::Mapping::new()));
        }

        // Premium pass
        let reward_type = if rand::thread_rng().gen_bool(config.premium_reward_chance / 100.0) {
            "premium"
        } else {
            "free"
        };

        let reward = if reward_type == "premium" {
            format!(
                "p{}",
                rand::thread_rng().gen_range(1..=config.last_premium_reward)
            )
        } else {
            rand::thread_rng()
                .gen_range(1..=config.last_free_reward)
                .to_string()
        };

        let mut premium_entry = serde_yaml::Mapping::from_iter([
            (
                "required-points".into(),
                (required_points as f64 * config.premium_xp_multiplier).into(),
            ),
            ("rewards".into(), vec![reward].into()),
        ]);

        if reward_type == "premium" {
            premium_entry.insert(
                "locked-tier-item".into(),
                serde_yaml::Mapping::from_iter([
                    ("material".into(), "gold_nugget".into()),
                    ("customModelData".into(), 1509.into()),
                    ("name".into(), "&e%tier%. Aşama".into()),
                ])
                .into(),
            );
        }

        premium_pass.push(premium_entry.into());
    }

    write_yaml(&format!("{}/free.yml", output_passes_dir), &free_pass);
    write_yaml(&format!("{}/premium.yml", output_passes_dir), &premium_pass);

    println!("Done!");
}
