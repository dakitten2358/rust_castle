use specs::prelude::*;
use specs_derive::Component;

use crate::components::*;

use crate::render::Renderable;
use crate::components::{CombatStats, AppliesDamage};

#[derive(Component)]
pub struct AiMoveToPlayer {}

#[allow(dead_code)]
pub fn create_test_ai(world: &mut World) {
    world.create_entity()
        .with(Position{ x: 4, y: 4})
        .with(Renderable::new_with_z('\u{2663}', rltk::RED, 1))
        .with(Movement::new())
        .with(ColliderComponent{})
        .with(AiMoveToPlayer{})
        .with(CombatStats { max_health: 10, health: 10 })
        .with(AppliesDamage { damage: 10 })
        .with(DebugName { text: "ai".to_string() })
        .build();
}

pub struct AiMoveToPlayerSystem {}

impl AiMoveToPlayerSystem {
    fn find_nearest_player_position<'a>(my_position: &Position, players: &ReadStorage<'a, Player>, positions: &ReadStorage<'a, Position>) -> Option<Position> {
        let mut found_position: Option<Position> = None;
        let mut shortest_distance_sq_so_far = i32::MAX;

        for (_player, position) in (players, positions).join() {
            let distance = Position::distance_sq(my_position, position);
            if distance < shortest_distance_sq_so_far {
                shortest_distance_sq_so_far = distance;
                found_position = Some(position.clone());
            }
        }
        found_position
    }
}

impl<'a> System<'a> for AiMoveToPlayerSystem {
    type SystemData = (ReadStorage<'a, AiMoveToPlayer>, ReadStorage<'a, Position>, ReadStorage<'a, Player>, WriteStorage<'a, Movement>);

    fn run(&mut self, (ais, positions, players, mut movements): Self::SystemData) {
        for (_ai, position, movement) in (&ais, &positions, &mut movements).join() {
            match AiMoveToPlayerSystem::find_nearest_player_position(position, &players, &positions) {
                Some(target_position) => {
                    let (move_x, move_y) = Position::delta(position, &target_position);
                    movement.add_movement_input(move_x, move_y);
                },
                None => {}
            }
        }
    }
}
