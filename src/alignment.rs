use crate::movement::{Velocity, Position, normalize_force};
use amethyst::core::math::Vector2;
use amethyst::core::ecs::{Component, VecStorage, System, ReadStorage, WriteStorage, Join, Entities};

#[derive(Debug)]
pub struct AlignmentComponent(pub Vector2<f32>);

impl AlignmentComponent {
    pub fn new() -> AlignmentComponent {
        AlignmentComponent(Vector2::<f32>::zeros())
    }
}

impl Component for AlignmentComponent {
    type Storage = VecStorage<Self>;
}

pub struct AlignmentSystem {
    pub distance: f32,
    pub force: f32,
}

impl<'s> System<'s> for AlignmentSystem {
    type SystemData = (
        ReadStorage<'s, Position>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, AlignmentComponent>,
        Entities<'s>
    );

    fn run(&mut self, (positions, velocities, mut alignments, entities): Self::SystemData) {
        let distance_squared = self.distance * self.distance;

        for (
            mut align_a,
            pos_a,
            vel_a,
            entity_a
        ) in (
            &mut alignments,
            &positions,
            &velocities,
            &entities
        ).join() {
            let (sum_vel, count): (Vector2<f32>, usize) = (&positions, &velocities, &entities)
                .join()
                .filter(|(_, _, entity_b)| &entity_a != entity_b)
                .filter(|(pos_b, _, _)| (&pos_b.0 - &pos_a.0).norm_squared() < distance_squared)
                .fold(
                    (Vector2::zeros(), 0),
                    |(sum_vel, count), (_, vel_b, _)| (&sum_vel + &vel_b.0, count + 1),
                );

            align_a.0 = if count > 0 {
                let avg_vel: Vector2<f32> = sum_vel / count as f32;
                normalize_force(&avg_vel + &vel_a.0, self.force)
            } else {
                Vector2::zeros()
            };

            debug_assert!(align_a.0.x.is_finite() && align_a.0.y.is_finite());
        }
    }
}
