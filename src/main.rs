mod alignment;
mod cohesion;
mod separation;
mod movement;

use std::iter::repeat;

use crate::movement::{Velocity, Position, MovementSystem};

use rand::{Rng, thread_rng};
use amethyst::prelude::*;
use amethyst::assets::AssetLoaderSystemData;
use amethyst::core::{Transform, TransformBundle};
use amethyst::renderer::{Camera, Material, MaterialDefaults, Mesh, RenderFlat3D, RenderingBundle, RenderToWindow, Texture};
use amethyst::renderer::loaders::load_from_linear_rgba;
use amethyst::renderer::palette::LinSrgba;
use amethyst::renderer::rendy::mesh;
use amethyst::renderer::types::DefaultBackend;
use amethyst::window::{DisplayConfig, ScreenDimensions};
use crate::alignment::{AlignmentSystem, AlignmentComponent};
use crate::cohesion::{CohesionSystem, CohesionComponent};
use crate::separation::{SeparationSystem, SeparationComponent};

const NUM_BOIDS: usize = 250;
const BOID_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);
const BOID_SCALE: [f32; 3] = [8.0, 8.0, 1.0];
const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<AlignmentComponent>();
        world.register::<CohesionComponent>();
        world.register::<SeparationComponent>();
        world.register::<movement::Position>();
        world.register::<movement::Velocity>();

        let screen = (*world.read_resource::<ScreenDimensions>()).clone();

        initialize_boids(world, &screen);
        initialize_camera(world, &screen);
    }
}

fn initialize_camera(world: &mut World, screen: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0 * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(screen.width(), screen.height()))
        .with(transform)
        .build();
}

fn initialize_boids(world: &mut World, screen: &ScreenDimensions) {
    let vertices: Vec<mesh::Position> = vec![
        [0.0, 1.0, 0.0].into(),
        [-0.64, -0.77, 0.0].into(),
        [0.0, -0.5, 0.0].into(),
        [0.0, 1.0, 0.0].into(),
        [0.0, -0.5, 0.0].into(),
        [0.64, -0.77, 0.0].into()
    ];

    let tex_cords: Vec<mesh::TexCoord> = repeat::<mesh::TexCoord>([1.0, 1.0].into())
        .take(6)
        .collect();

    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            mesh::MeshBuilder::from((vertices, tex_cords))
                .into(),
            (),
        )
    });

    let albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::from_components(BOID_COLOR)).into(),
            (),
        )
    });

    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();

    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                albedo,
                ..mat_defaults
            },
            (),
        )
    });

    let mut rng = thread_rng();

    for _ in 0..NUM_BOIDS {
        let position = Position::new(
            rng.gen_range(-screen.width() / 2.0, screen.width() / 2.0),
            rng.gen_range(-screen.height() / 2.0, screen.height() / 2.0),
        );

        let mut transform = Transform::default();
        transform.set_translation_xyz(
            position.0.x,
            position.0.y,
            0.0,
        );
        transform.set_scale(BOID_SCALE.into());

        world
            .create_entity()
            .with(position)
            .with(Velocity::new())
            .with(AlignmentComponent::new())
            .with(CohesionComponent::new())
            .with(SeparationComponent::new())
            .with(transform)
            .with(mesh.clone())
            .with(material.clone())
            .build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let mut display_config = DisplayConfig::default();
    display_config.title = "Boids".to_owned();
    display_config.dimensions = Some((1200, 800));
    display_config.resizable = false;

    let game_data = GameDataBuilder::default()
        .with(
            AlignmentSystem {
                distance: 100.0,
                force: 0.1,
            },
            "alignment",
            &[],
        )
        .with(
            CohesionSystem {
                distance: 200.0,
                force: 0.1,
            },
            "cohesion",
            &[],
        )
        .with(
            SeparationSystem {
                distance: 50.0,
                force: 0.1,
            },
            "separation",
            &[],
        )
        .with(
            MovementSystem {
                max_velocity: 5.0
            },
            "movement",
            &["alignment", "cohesion", "separation"],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config(display_config)
                                 .with_clear(BACKGROUND_COLOR),
                )
                .with_plugin(RenderFlat3D::default()),
        )?;

    let mut game = Application::new("assets/", GameState, game_data)?;
    game.run();

    Ok(())
}
