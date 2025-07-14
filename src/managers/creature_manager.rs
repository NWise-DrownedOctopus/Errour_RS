use crate::assets::animations::enemy1_idel_animation;
use crate::assets::art_assets::GameArtAssets;
use crate::utils::GameContext;
use crate::components::common::{Velocity, Rotation, RotationalVelocity, Size, Target, Collided, Health, Damage, Dead};
use crate::components::collider::CircleCollider;
use macroquad::prelude::*;

pub fn spawn_creature<'a>(
    context: &mut GameContext,
    pos: Vec2,
    art_assets: &'a GameArtAssets,
) -> usize {
    let id = context.positions.len();

    context.positions.push(pos);
    context.velocities.push(Velocity(0.5));
    context.rotations.push(Rotation(0.));
    context.rotational_velocities.push(RotationalVelocity(rand::gen_range(-2., 2.)));
    context.sizes.push(Size(30.0));
    context.targets.push(Target(vec2(525.0, 500.0)));
    context.collided_flags.push(Collided(false));
    context.healths.push(Health(10.0));
    context.damages.push(Damage(10.0));
    context.dead_flags.push(Dead(false));
    context.colliders.push(CircleCollider {
        center: pos,
        radius: 12.0,
    });

    context.creature_ids.push(id);
    id
}
