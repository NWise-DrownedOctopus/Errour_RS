use crate::assets::animations::enemy1_idel_animation;
use crate::assets::art_assets::GameArtAssets;
use crate::utils::GameContext;
use crate::components::common::{Position, Velocity, Rotation, RotationalVelocity, Size, Target, Collided, Health, Damage, Dead};
use crate::components::creature::{CreatureAnimator, CreatureCollider};
use crate::components::collider::CircleCollider;
use crate::components::animation::Animator;
use macroquad::prelude::*;

pub fn spawn_creature<'a>(
    context: &mut GameContext<'a>,
    pos: Vec2,
    art_assets: &'a GameArtAssets,
) -> usize {
    let id = context.positions.len();

    context.positions.push(Position(pos));
    context.velocities.push(Velocity(0.5));
    context.rotations.push(Rotation(0.));
    context.rotational_velocities.push(RotationalVelocity(rand::gen_range(-2., 2.)));
    context.sizes.push(Size(30.0));
    context.targets.push(Target(vec2(525.0, 500.0)));
    context.collided_flags.push(Collided(false));
    context.healths.push(Health(10.0));
    context.damages.push(Damage(10.0));
    context.dead_flags.push(Dead(false));
    context.colliders.push(CreatureCollider(CircleCollider {
        center: pos,
        radius: 12.0,
    }));
    context.animators.push(CreatureAnimator(Animator {
        texture: &art_assets.enemy_texture,
        frame_width: 48.0,
        frame_height: 48.0,
        columns: 3,
        animation: enemy1_idel_animation(),
        shadow_offset: 25.0,
    }));

    context.creature_ids.push(id);
    id
}
