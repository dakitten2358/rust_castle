use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::*;
use std::fs::File;

use crate::components::*;

use crate::ai::AiMoveToPlayer;
use crate::components::{AppliesDamage, CombatStats};
use crate::game::DynamicMarker;
use crate::render::Renderable;

pub fn create_enemy(world: &mut World, room: i32, enemy_name: &str, x: i32, y: i32, health: Option<i32>) {
    let enemy = find_enemy_by_name(enemy_name, &world.fetch::<Vec<EnemyData>>())
        .expect("failed to find enemy")
        .clone();
    spawn_enemy(world, room, &enemy, x, y, health);
}

fn spawn_enemy(world: &mut World, room: i32, item: &EnemyData, x: i32, y: i32, health: Option<i32>) {
    let mut entity = world
        .create_entity()
        .with(Position { x: x, y: y })
        .with(Renderable::new_with_z(item.glyph, rltk::RED, 1))
        .with(Movement::new())
        .with(ColliderComponent {})
        .with(CombatStats {
            max_health: item.health,
            health: if let Some(valid_health) = health {
                valid_health
            } else {
                item.health
            },
        })
        .with(DebugName { text: item.name.clone() })
        .with(crate::room::BelongsToRoom { room: room })
        .with(if let Some(explicit_name) = &item.input_name {
            Description::new_explicit(&explicit_name, &item.name, &item.description)
        } else {
            Description::new(&item.name, &item.description)
        })
        .marked::<SimpleMarker<DynamicMarker>>();

    if let Some(damage) = item.damage {
        entity = entity.with(AppliesDamage { damage: damage }).with(AiMoveToPlayer {})
    }

    entity.build();
}

fn find_enemy_by_name<'a>(enemy_to_find: &str, enemies: &'a Vec<EnemyData>) -> Option<&'a EnemyData> {
    for enemy in enemies {
        if let Some(input_name) = &enemy.input_name {
            if input_name == enemy_to_find {
                return Some(enemy);
            }
        }
        if enemy.name.to_ascii_lowercase() == enemy_to_find.to_ascii_lowercase() {
            return Some(enemy);
        }
    }
    return None;
}

#[derive(Serialize, Deserialize, Clone)]
struct EnemyData {
    pub name: String,
    pub input_name: Option<String>,
    pub description: String,
    pub glyph: char,
    pub health: i32, // max health
    pub damage: Option<i32>,
}

pub fn load_enemies(world: &mut World) {
    save_enemies();

    let f = File::open("data/enemies.json").expect("enemy data not found");
    let enemies: Vec<EnemyData> = serde_json::from_reader(f).expect("failed to deserializer!");

    world.insert(enemies);
}

#[allow(dead_code)]
fn save_enemies() {
    let mut enemies = Vec::new();

    let e = EnemyData {
        name: "Ogre".to_string(),
        input_name: Some("ogre".to_string()),
        description: "It's smelly!".to_string(),
        glyph: '\u{263A}',
        health: 1,
        damage: Some(1),
    };
    enemies.push(e);

    let writer = std::fs::File::create("./data/enemies_ex.json").unwrap();
    let mut serializer = serde_json::Serializer::pretty(writer);

    (&enemies).serialize(&mut serializer).expect("failed to write example enemy data");
}
