use crate::movement::{Position, normalize_force};
use amethyst::core::math::Vector2;
use amethyst::core::ecs::{Component, VecStorage, System, ReadStorage, WriteStorage, Join, Entities};

#[derive(Debug)]
pub struct CohesionComponent(pub Vector2<f32>);

impl CohesionComponent {
    pub fn new() -> CohesionComponent {
        CohesionComponent(Vector2::<f32>::zeros())
    }
}

impl Component for CohesionComponent {
    type Storage = VecStorage<Self>;
}

pub struct CohesionSystem {
    pub distance: f32,
    pub force: f32,
}

impl<'s> System<'s> for CohesionSystem {
    type SystemData = (
        ReadStorage<'s, Position>,
        WriteStorage<'s, CohesionComponent>,
        Entities<'s>
    );

    fn run(&mut self, (positions, mut cohesions, entities): Self::SystemData) {
        let distance_squared = self.distance * self.distance;

        for (
            mut coh_a,
            pos_a,
            entity_a
        ) in (
            &mut cohesions,
            &positions,
            &entities
        ).join() {
            let (sum_pos, count): (Vector2<f32>, usize) = (&positions, &entities)
                .join()
                .filter(|(_, entity_b)| &entity_a != entity_b)
                .filter(|(pos_b, _)| (&pos_b.0 - &pos_a.0).norm_squared() < distance_squared)
                .fold(
                    (Vector2::zeros(), 0),
                    |(sum_pos, count), (pos_b, _)| (&sum_pos + &pos_b.0, count + 1),
                );

            coh_a.0 = if count > 0 {
                let avg_pos: Vector2<f32> = sum_pos / count as f32;
                normalize_force(&avg_pos - &pos_a.0, self.force)
            } else {
                Vector2::zeros()
            };

            debug_assert!(coh_a.0.x.is_finite() && coh_a.0.y.is_finite());
        }
    }
}