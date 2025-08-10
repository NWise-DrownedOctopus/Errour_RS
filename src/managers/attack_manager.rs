use macroquad::prelude::*;
// Componenets
use crate::{components::{collider::CircleCollider}};
use crate::components::common::{Dead};
// Managers

// Systems
use crate::systems::collision::intersects;
//Utils
use crate::utils::Timer;

#[derive(Copy, Clone)]
pub enum AttackType {
    Projectile,
    AOE,
    Chain,
}

pub struct AttackCommand {
    pub attacker_pos: Vec2,
    pub target_index: usize,
    pub attack_type: AttackType,
}

pub struct Attacker {
    pub range_collider_index: usize,
    pub cooldown_index: usize,
    pub target_ref_index: usize,
    pub attack_type_index: usize,
    pub attack_pos_index: usize,
}

pub struct AttackManager {
    pub attackers: Vec<Attacker>,
    pub range_colliders: Vec<CircleCollider>,
    pub cooldowns: Vec<Timer>,
    pub target_indices: Vec<Option<usize>>,
    pub attack_types: Vec<AttackType>,
    pub attacker_positions: Vec<Vec2>,
}

impl AttackManager {
    pub fn new() -> Self {
        Self {
            attackers: Vec::new(),
            cooldowns: Vec::new(),
            range_colliders: Vec::new(),            
            target_indices: Vec::new(),
            attack_types: Vec::new(),   
            attacker_positions: Vec::new(),                    
        }
    }

    pub fn register_attacker(
        &mut self,
        cd: Timer,
        range_collider: CircleCollider,
        attack_type: AttackType,
        attacker_pos: Vec2,
    ) -> usize {
        let cd_index = self.cooldowns.len();
        let range_collider_index = self.range_colliders.len();
        let target_indicy_index = self.target_indices.len();
        let attack_type_index = self.attack_types.len();
        let attack_pos_index = self.attacker_positions.len();

        self.cooldowns.push(cd);
        self.range_colliders.push(range_collider);
        self.target_indices.push(None);
        self.attack_types.push(attack_type);
        self.attacker_positions.push(attacker_pos);
        self.attackers.push(Attacker { 
            range_collider_index: range_collider_index,
            cooldown_index: cd_index,
            target_ref_index: target_indicy_index,
            attack_type_index: attack_type_index,
            attack_pos_index: attack_pos_index,
        });
        self.attackers.len() -1
    }

    pub fn update_attack_solutions(
        &mut self,
        target_positions: &[Vec2],
        target_colliders: &[CircleCollider],
        valid_target_flags: &[Dead],
    ) {
        // println!("Running attack solution update...");
        // This loop is operating under the assumption that all of our attacker data vector indicies have parity with eachother
        for i in 0..self.attackers.len() {
            let attacker = &self.attackers[i];

            // 1. Update Cooldown Timer
            self.cooldowns[attacker.cooldown_index].update();

            // 2. Get Attacker Pos and Collider
            let self_pos = self.attacker_positions[i];
            let self_collider = &self.range_colliders[attacker.range_collider_index];

            // 3. Try to find valid target
            let mut found_target = None;
            // This rust syntax is kinda wild so I want to explain for refernce what is going on
            // We are creating a for loop that is iterating though a pair of data indicies which we have bundled together and
            // given another index (j). j is then used as an index within other data containers because it's index will have parity with
            // The data we bundled together for use inside the for loop.
            // the for loop needs an index (j) and a tuple that is a (vec2, &CircleCollider), which is what the .enumerate() is returning
            for (j, (&target_pos, target_collider)) in target_positions.iter().zip(target_colliders).enumerate() {
                if !valid_target_flags[j].is_alive() {
                    continue; 
                }

                if intersects(target_pos, target_collider, self_pos, self_collider) {
                    // Lets try and select the target only if it's the closest target
                    if let Some(prev_index) = found_target {
                        let prev_pos = target_positions[prev_index];
                        let prev_dist = self_pos.distance(prev_pos);
                        let curr_dist = self_pos.distance(target_pos);

                        if curr_dist < prev_dist {
                            found_target = Some(j);
                        }                        
                    } else {
                        found_target = Some(j);
                    }
                }
            }

            // 4. Save selected target 
            self.target_indices[attacker.target_ref_index] = found_target;
        }
    }

    pub fn fire_ready_attackers(
        &mut self,
        commands: &mut Vec<AttackCommand>
    ) {
        for i in 0..self.attackers.len() {
            if self.is_ready_to_attack(i) {
                // We would like to fire now
                println!("We are signialing we are ready to fire at our target");
                // Here we should fire at the target
                if let Some(target_index) = self.target_indices[i] {
                    commands.push(AttackCommand {
                        attacker_pos: self.attacker_positions[i],
                        target_index, 
                        attack_type: self.attack_types[i],
                    });
                }
                // Here we reset the cooldown timer of the attacker
                self.cooldowns[i].reset();
            }
        }
    }

    fn is_ready_to_attack(&self, index: usize) -> bool {
        let attacker = &self.attackers[index];
        self.cooldowns[attacker.cooldown_index].is_ready() && self.target_indices[attacker.target_ref_index].is_some()
    }
}



