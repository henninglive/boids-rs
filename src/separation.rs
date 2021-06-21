use crate::movement::{Position, normalize_force};
use amethyst::core::math::Vector2;
use amethyst::core::ecs::{Component, VecStorage, System, WriteStorage, ReadStorage, Join, Entities};
use amethyst::core::num::Zero;

#[derive(Debug)]
pub struct SeparationComponent(pub Vector2<f32>);

impl SeparationComponent {
    pub fn new() -> SeparationComponent {
        SeparationComponent(Vector2::<f32>::zeros())
    }
}

impl Component for SeparationComponent {
    type Storage = VecStorage<Self>;
}

pub struct SeparationSystem {
    pub distance: f32,
    pub force: f32,
}

impl<'s> System<'s> for SeparationSystem {
    type SystemData = (
        ReadStorage<'s, Position>,
        WriteStorage<'s, SeparationComponent>,
        Entities<'s>,
    );

    fn run(&mut self, (positions, mut separations, entities): Self::SystemData) {
        let distance_squared = self.distance * self.distance;

        for (
            mut sep_a,
            pos_a,
            entity_a
        ) in (
            &mut separations,
            &positions,
            &entities
        ).join() {

            let force: Vector2<f32> = (&positions, &entities)
                .join()
                .filter(|(_, entity_b)| &entity_a != entity_b)
                .filter(|(pos_b, _)| (&pos_b.0 - &pos_a.0).norm_squared() < distance_squared)
                .fold(
                    Vector2::zeros(),
                    |force, (pos_b, _)| &force - (&pos_b.0 - &pos_a.0),
                );

            sep_a.0 = if !force.is_zero() {
                normalize_force(force, self.force)
            } else {
                Vector2::zeros()
            };

            debug_assert!(sep_a.0.x.is_finite() && sep_a.0.y.is_finite());
        }
    }
}