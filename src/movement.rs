use crate::separation::SeparationComponent;
use crate::alignment::AlignmentComponent;
use crate::cohesion::CohesionComponent;

use amethyst::core::math::{Vector2, Vector3};
use amethyst::core::ecs::{Component, VecStorage, System, WriteStorage, ReadStorage, Join, ReadExpect};
use amethyst::core::Transform;
use amethyst::window::ScreenDimensions;
use amethyst::core::num::Zero;

#[derive(Debug)]
pub struct Position(pub Vector2<f32>);

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position(Vector2::new(x, y))
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub Vector2<f32>);

impl Velocity {
    pub fn new() -> Velocity {
        Velocity(Vector2::new(0.0, 0.0))
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub struct MovementSystem {
    pub max_velocity: f32,
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, AlignmentComponent>,
        ReadStorage<'s, CohesionComponent>,
        ReadStorage<'s, SeparationComponent>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (screen, alignments, cohesions, separations, mut positions, mut velocities, mut transforms): Self::SystemData) {
        for (
            align,
            cohesion,
            separation,
            position,
            velocity,
            transform
        ) in (
            (&alignments).maybe(),
            (&cohesions).maybe(),
            (&separations).maybe(),
            &mut positions,
            &mut velocities,
            &mut transforms
        ).join() {
            let mut force = Vector2::<f32>::zeros();

            match align {
                Some(align) => force += &align.0,
                None => (),
            }
            match cohesion {
                Some(cohesion) => force += &cohesion.0,
                None => (),
            }
            match separation {
                Some(separation) => force += &separation.0,
                None => (),
            }

            velocity.0 = clamp(&velocity.0 + &force, self.max_velocity);

            position.0 += &velocity.0;
            if position.0.x > screen.width() / 2.0 {
                position.0.x -= screen.width();
            } else if position.0.x < -screen.width() / 2.0 {
                position.0.x += screen.width();
            }
            if position.0.y > screen.height() / 2.0 {
                position.0.y -= screen.height();
            } else if position.0.y < -screen.height() / 2.0 {
                position.0.y += screen.height();
                position.0.y += screen.height();
            }

            transform.set_translation(Vector3::new(position.0.x, position.0.y, 0.0));

            let mut yaw = f32::atan2(velocity.0.y, velocity.0.x);
            if velocity.0.y < 0.0 {
                yaw += std::f32::consts::TAU;
            }
            yaw -= std::f32::consts::FRAC_PI_2;

            transform.set_rotation_euler(0.0, 0.0, yaw);
        }
    }
}

fn clamp(v: Vector2<f32>, limit: f32) -> Vector2<f32> {
    let norm_squared = v.norm_squared();
    if norm_squared > limit * limit {
        (v / norm_squared.sqrt()) * limit
    } else {
        v
    }
}

pub fn normalize_force(direction: Vector2<f32>, magnitude: f32) -> Vector2<f32> {
    if direction.is_zero() {
        Vector2::zeros()
    } else {
        direction.normalize() * magnitude
    }
}