use crate::components::*;
use crate::StateAction;
use specs::prelude::*;

pub struct DamageSystem {}

fn kill(target: Entity, dead_tags: &mut WriteStorage<DeadTag>) {
    dead_tags.insert(target, DeadTag {}).expect("failed to add kill tag!");
}

fn add_damage(instigator: Entity, target: Entity, amount: i32, damages: &mut WriteStorage<ApplyDamageComponent>) {
    if let Some(damage) = damages.get_mut(target) {
        damage.amounts.push(amount);
        damage.instigator = instigator;
    } else {
        let damage = ApplyDamageComponent {
            amounts: vec![amount],
            instigator: instigator,
        };
        damages.insert(target, damage).expect("failed to add apply damage");
    }
}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, ApplyDamageComponent>,
        WriteStorage<'a, DeadTag>,
        WriteStorage<'a, WantsToAttack>,
        ReadStorage<'a, AppliesDamage>,
        WriteStorage<'a, CombatLog>,
        ReadStorage<'a, Description>,
    );

    fn run(
        &mut self,
        (
            entities,
            players,
            mut combat_stats,
            mut apply_damages,
            mut dead_tags,
            mut wants_to_attack,
            applies_damages,
            mut combat_logs,
            descriptions,
        ): Self::SystemData,
    ) {
        // check for people that want to apply damage to an entity
        for (entity, applies_damage, wants_to_attack) in (&entities, &applies_damages, &mut wants_to_attack).join() {
            add_damage(entity, wants_to_attack.target, applies_damage.damage, &mut apply_damages);
        }
        wants_to_attack.clear();

        // apply damages
        for (entity, combat_stat, apply_damages) in (&entities, &mut combat_stats, &apply_damages).join() {
            if combat_stat.health > 0 {
                combat_stat.health -= apply_damages.amounts.iter().sum::<i32>();
                if let Some(combat_log) = combat_logs.get_mut(entity) {
                    combat_log.push(format!("You were hit"));
                } else {
                    for (_player, combat_log) in (&players, &mut combat_logs).join() {
                        let maybe_name = get_entity_name(entity, &descriptions);
                        match maybe_name {
                            Some(name) => {
                                let msg = format!("{} was hit!", name.as_str());
                                combat_log.push(msg);
                            }
                            None => combat_log.push("unknown was hit!".to_string()),
                        }
                    }
                }

                if combat_stat.health <= 0 {
                    kill(entity, &mut dead_tags);
                    println!("killed!");
                }
            }
        }
        apply_damages.clear();
    }
}

pub fn get_entity_name(e: Entity, descriptions: &ReadStorage<Description>) -> Option<String> {
    let desc = descriptions.get(e);
    match desc {
        Some(description) => Some(description.name.clone()),
        None => None,
    }
}

pub fn attack(attacker: Entity, target: Entity, attacks: &mut WriteStorage<WantsToAttack>) {
    if let Some(attack) = attacks.get_mut(attacker) {
        attack.target = target;
    } else {
        let attack = WantsToAttack { target: target };
        attacks.insert(attacker, attack).expect("failed to add wants to attack");
    }
}

pub struct MeleeCombatSystem {}

fn find_target_at(target_x: i32, target_y: i32, entities: &Entities, positions: &ReadStorage<Position>) -> Option<Entity> {
    for (entity, position) in (entities, positions).join() {
        if position.equals_xy(target_x, target_y) {
            return Some(entity);
        }
    }
    None
}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Movement>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, AppliesDamage>,
        WriteStorage<'a, WantsToAttack>,
    );

    fn run(&mut self, (entities, movements, positions, damage_stats, mut wants_to_attack): Self::SystemData) {
        for (entity, movement, position, _damage_stats) in (&entities, &movements, &positions, &damage_stats).join() {
            if movement.was_move_blocked() {
                let (delta_x, delta_y) = movement.get_attempted_move();
                let (target_x, target_y) = (position.x + delta_x, position.y + delta_y);
                match find_target_at(target_x, target_y, &entities, &positions) {
                    Some(target) => {
                        attack(entity, target, &mut wants_to_attack);
                    }
                    None => {}
                }
            }
        }
    }
}

pub struct ClearDeadSystem {}

impl ClearDeadSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for ClearDeadSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, DeadTag>, WriteExpect<'a, Vec<StateAction>>,);

    fn run(&mut self, (entities, dead_tags, mut state_actions): Self::SystemData) {
        let mut entities_to_delete = Vec::new();

        for (entity, _dead_tag) in (&entities, &dead_tags).join() {
            entities_to_delete.push(entity);
        }

        state_actions.push(StateAction::DeleteEntities {
            entities: entities_to_delete,
        });
    }
}
