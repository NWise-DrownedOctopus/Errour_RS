use crate::components::animation::Animator;
use crate::components::collider::CircleCollider;
pub struct CreatureCollider (pub CircleCollider);
pub struct CreatureAnimator<'a>(pub Animator<'a>);
 