extern crate amethyst;
extern crate rand;
extern crate nalgebra as na;

use amethyst::{Application, Event, State, Trans, VirtualKeyCode, WindowEvent};
use amethyst::asset_manager::AssetManager;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use amethyst::ecs::{World, RunArg, VecStorage, Component, System};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};

use rand::distributions::range;

use na::core::Vector2;

type Quaternion = na::geometry::UnitQuaternionBase<f32, na::core::MatrixArray<f32, na::U4, na::U1>>;

struct BoidsState;
struct BoidsSystem;

#[derive(Debug)]
struct Pos(Vector2<f32>);
#[derive(Debug)]
struct Vel(Vector2<f32>);
#[derive(Debug)]
struct Acc(Vector2<f32>);

impl Default for Pos {
    fn default() -> Pos {
        Pos(na::zero::<Vector2<f32>>())
    }
}

impl Default for Vel {
    fn default() -> Vel {
        Vel(na::zero::<Vector2<f32>>())
    }
}

impl Default for Acc {
    fn default() -> Acc {
        Acc(na::zero::<Vector2<f32>>())
    }
}

impl Component for Pos {
    type Storage = VecStorage<Pos>;
}

impl Component for Vel {
    type Storage = VecStorage<Vel>;
}

impl Component for Acc {
    type Storage = VecStorage<Acc>;
}

const NUM_BOIDS: usize = 400;
const MAX_VEL: f32 = 100.0;
const MAX_ACC: f32 = 100.0;
const SEPARATION: f32 = 50.0;
const SEPARATION_FORCE: f32 = 0.5;
const ALIGN_FORCE: f32 = 0.1;
const COHESION_FORCE: f32 = 0.01;

fn forces(pos_a: &Vector2<f32>, pos_b: &Vector2<f32>, vel_b: &Vector2<f32>) -> Option<Vector2<f32>> {
    let dis = pos_a - pos_b;
    let norm = dis.norm();

    // Separation distance
    if norm < SEPARATION {
        let mut force = dis.normalize() * (SEPARATION - norm) * SEPARATION_FORCE;
        if !force.x.is_normal() {
            force.x = 0.0;
        }
        if !force.y.is_normal() {
            force.y = 0.0;
        }
        force += vel_b * ALIGN_FORCE;
        Some(force)
    } else {
        None
    }
}

impl System<()> for BoidsSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        use amethyst::ecs::Join;
        use amethyst::ecs::resources::{Camera, Projection, Time};

        // Acquire storage for components and resources
        let (mut pos_all, mut vel_all, mut acc_all, mut trans_all, time, camera) = arg.fetch(|f| {(
            f.write::<Pos>(),
            f.write::<Vel>(),
            f.write::<Acc>(),
            f.write::<LocalTransform>(),
            f.read_resource::<Time>(),
            f.read_resource::<Camera>(),
        )});

        // Get world borders
        let (left, right, bottom, top) = match camera.proj {
            Projection::Orthographic{left, right, bottom, top, ..} => (left, right, bottom, top),
            _ => panic!(),
        };

        // World with and height
        let width = Vector2::new(right - left, 0.0);
        let height = Vector2::new(0.0, top - bottom);

        // Iterate entities with Pos and Acc components.
        for (pos, acc) in (&pos_all, &mut acc_all).join() {
            let z = (na::zero::<Vector2<f32>>(), na::zero::<Vector2<f32>>());
            
            // Iterate entities with Pos and Vel components.
            // Sum separation force and alignment.
            // Avg position.
            let (sum_forces, avg_pos) = (&pos_all, &vel_all)
                .join().filter_map(|(pos2, vel2)| {
                    // Wrap around world borders 
                    if let Some(force) = forces(&pos.0, &pos2.0, &vel2.0)
                        .or(forces(&pos.0, &(pos2.0 + width), &vel2.0))
                        .or(forces(&pos.0, &(pos2.0 - width), &vel2.0))
                        .or(forces(&pos.0, &(pos2.0 + height), &vel2.0))
                        .or(forces(&pos.0, &(pos2.0 - height), &vel2.0))
                    {
                        Some((force, pos2.0))
                    } else {
                        None
                    }
                }
            ).fold(z, |sum, i| (sum.0 + i.0, sum.1 + i.1));
            
            // add forces and clamp acceleration
            acc.0 += sum_forces + (pos.0 - avg_pos) * COHESION_FORCE;
            if acc.0.norm() > MAX_ACC {
                acc.0 = acc.0.normalize() * MAX_ACC;
            }
        }

        // Delta time, seconds
        let delta_time = time.delta_time.subsec_nanos() as f32 / 1000000000.0;

        for (pos, vel, acc, trans) in (&mut pos_all, &mut vel_all, &acc_all, &mut trans_all).join()
        {
            // Update and clamp velocity position
            vel.0 += acc.0 * delta_time;
            if vel.0.norm() > MAX_VEL {
                vel.0 = vel.0.normalize() * MAX_VEL;
            }

            // Update and wrap around 
            pos.0 += vel.0 * delta_time;
            if pos.0.x > right {
                pos.0 -= width;
            }
            if pos.0.x < left {
                pos.0 += width;
            }
            if pos.0.y > top {
                pos.0 -= height;
            }
            if pos.0.y < bottom {
                pos.0 += height;
            }

            // Rotation
            let inv = -vel.0;
            let mut roll = inv.x.atan2(inv.y);
            if !roll.is_normal() {
                roll = 0.0;
            }

            // Update local translation.
            trans.rotation = Quaternion::from_euler_angles(roll, 0.0, 0.0).coords.into();
            trans.translation = [pos.0.x, pos.0.y, 0.0];
        }

        // Translations are applied automatically and renderable entities
        // are rendered automatically.
    }
}

fn update_camera(world: &mut World, width: f32, height: f32) {
    use amethyst::ecs::resources::{Camera, Projection};
    use amethyst::ecs::Gate;

    let mut camera = world.write_resource::<Camera>().pass(); // pass() = rwlock
    camera.eye = [0.0, 0.0, 0.1];
    camera.target = [0.0, 0.0, 0.0];
    camera.up = [0.0, 1.0, 0.0];
    camera.proj = Projection::Orthographic {
        left: -(width / 2.0),
        right: (width / 2.0),
        bottom: -(height / 2.0),
        top: (height / 2.0),
        near: 0.0,
        far: 1.0,
    };
}

impl State for BoidsState {
    fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipe: &mut Pipeline) {
        use amethyst::renderer::pass::{Clear, DrawFlat};
        use amethyst::renderer::Layer;
        use amethyst::ecs::resources::ScreenDimensions;
        use amethyst::ecs::Gate;

        pipe.layers.push(Layer::new("main", vec![
            Clear::new([0.0, 0.0, 0.0, 1.0]),
            DrawFlat::new("main", "main"),
        ]));

        // Set camera to Orthographic with size equal to screen dimensions.
        let screen = world.read_resource::<ScreenDimensions>().pass();
        update_camera(world, screen.w, screen.h);

        // Create a renderable component for our boids
        assets.register_asset::<Mesh>();
        assets.register_asset::<Texture>();
        assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
        assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("arrow", mesh());
        let ren = assets.create_renderable("arrow", "white", "white", "white", 1.0).unwrap();

        let pos_range_x = range::Range::new(-(screen.w / 2.0), (screen.w / 2.0));
        let pos_range_y = range::Range::new(-(screen.h / 2.0), (screen.h / 2.0));
        let vel_range = range::Range::new(-10.0f32, 10.0f32);
        let mut rng = rand::thread_rng();

        // Spawn boids
        for _ in 0..NUM_BOIDS {
            use rand::distributions::IndependentSample;

            // Random position and velocity
            let pos = Pos(Vector2::new(pos_range_x.ind_sample(&mut rng), pos_range_y.ind_sample(&mut rng)));
            let vel = Vel(Vector2::new(vel_range.ind_sample(&mut rng), vel_range.ind_sample(&mut rng)));
            let acc = Acc(na::zero::<Vector2<_>>());
 
            let mut ltrans = LocalTransform::default();
            ltrans.scale = [10.0, 10.0, 10.0];

            // New boid entity with components
            world.create_now()
                .with(pos)
                .with(vel)
                .with(acc)
                .with(ltrans)
                .with(Transform::default())
                .with(ren.clone())
                .build();
        }
    }

    fn handle_events(&mut self, events: &[WindowEvent], world: &mut World,
        _: &mut AssetManager, _: &mut Pipeline) -> Trans
    {
        for e in events {
            match **e {
                Event::Resized(w, h) => update_camera(world, w as f32, h as f32),
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return Trans::Quit,
                Event::Closed => return Trans::Quit,
                _ => (),
            }
        }
        Trans::None
    }
}

fn main() {
    let mut cfg = DisplayConfig::default();
    cfg.title = "Boids".to_owned();

    // Disable resizing of window, resizing is currently broken in amethyst.
    // Min and Max includes window borders. 
    cfg.dimensions = Some((1024, 768));
    cfg.min_dimensions = Some((1024 + 16, 768 + 38));
    cfg.max_dimensions = Some((1024 + 16, 768 + 38));

    // New Application with Entity Component System.
    // Registrer custom components. Some built in components
    // are registered automatically.
    let mut game = Application::build(BoidsState, cfg)
        .register::<Pos>()
        .register::<Vel>()
        .register::<Acc>()
        .with::<BoidsSystem>(BoidsSystem, "boids_system", 1)
        .done();
    
    game.run();
}

/// Build arrow mesh
fn mesh() -> Vec<VertexPosNormal> {
    let data: Vec<VertexPosNormal> = vec![
        VertexPosNormal {
            pos: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coord: [0.0, 0.0],
        },
        VertexPosNormal {
            pos: [-0.64, -0.77, 0.0],
            normal: [0., 0., 1.],
            tex_coord: [1.0, 0.0],
        },
        VertexPosNormal {
            pos: [0.0, -0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coord: [1.0, 1.0],
        },
        VertexPosNormal {
            pos: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coord: [0.0, 0.0],
        },
        VertexPosNormal {
            pos: [0.0, -0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coord: [1.0, 1.0],
        },
        VertexPosNormal {
            pos: [0.64, -0.77, 0.0],
            normal: [0., 0., 1.],
            tex_coord: [1.0, 0.0],
        },
    ];
    data
}
