//! Time-Orbiting Potential (TOP) trap
//! 
//! cargo install -f wasm-bindgen-cli
//! cargo build --example top_trap --target wasm32-unknown-unknown
//! wasm-bindgen --out-dir target/web target/wasm32-unknown-unknown/release/examples/doppler_limit.wasm
//! 
extern crate atomecs;
extern crate nalgebra;
use atomecs::atom::{Atom, Force, Mass, Position, Velocity};
use atomecs::initiate::NewlyCreated;
use atomecs::integrator::Timestep;
use atomecs::magnetic::force::{MagneticDipole};
use atomecs::magnetic::quadrupole::QuadrupoleField3D;
use atomecs::magnetic::top::UniformFieldRotator;
use atomecs_demos::atoms::add_meshes_to_atoms;
use atomecs_demos::camera::{control_camera, DemoCamera};
use atomecs_demos::{add_atomecs_watermark, BevyAtomECSPlugin};
use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};
use bevy::prelude::*;
use atomecs::species::{Rubidium87_780D2};

fn main() {
    let mut app = App::new();

    // Add magnetics systems (todo: as plugin)
    app.add_plugin(atomecs::integrator::IntegrationPlugin);
    app.add_plugin(atomecs::magnetic::MagneticsPlugin);
    app.add_system(atomecs::output::console_output::console_output);
    app.add_plugins(DefaultPlugins);
    app.add_system(atomecs::bevy_bridge::copy_positions);
    app.add_system(control_camera);
    app.add_system(add_meshes_to_atoms::<Rubidium87_780D2>);
    app.add_startup_system(add_atomecs_watermark);
    app.add_plugin(BevyAtomECSPlugin);
    app.add_startup_system(setup);
    app.insert_resource(atomecs::bevy_bridge::Scale { 0: 1e4 });
    app.add_startup_system(setup_atoms);

    // Create magnetic field.
    app.world.spawn()
        .insert(QuadrupoleField3D::gauss_per_cm(80.0, Vector3::z()))
        .insert(Position::default());

    app.world.spawn()
        .insert(UniformFieldRotator { amplitude: 20.0, frequency: 3000.0 }) // Time averaged TOP theory assumes rotation frequency much greater than velocity of atoms
        .insert(atomecs::magnetic::uniform::UniformMagneticField { field: Vector3::new(0.0,0.0,0.0)}) // Time averaged TOP theory assumes rotation frequency much greater than velocity of atoms
        ;

    // Define timestep
    app.world.insert_resource(Timestep { delta: 5e-5 }); //Aliasing of TOP field or other strange effects can occur if timestep is not much smaller than TOP field period.
                                                //Timestep must also be much smaller than mean collision time.

    // Run the simulation for a number of steps.
    // for _i in 0..10000 {
    //      app.update();
    // }
    app.run();
}

fn setup_atoms(mut commands: Commands
) {
    let p_dist = Normal::new(0.0, 50e-6).unwrap();
    let v_dist = Normal::new(0.0, 0.004).unwrap(); // ~100nK

    for _i in 0..5000 {
        commands
            .spawn()
            .insert(Position {
                pos: Vector3::new(
                    p_dist.sample(&mut rand::thread_rng()),
                    p_dist.sample(&mut rand::thread_rng()),
                    0.35 * p_dist.sample(&mut rand::thread_rng()), //TOP traps have tighter confinement along quadrupole axis
                ),
            })
            .insert(Atom)
            .insert(Force::default())
            .insert(Velocity {
                vel: Vector3::new(
                    v_dist.sample(&mut rand::thread_rng()),
                    v_dist.sample(&mut rand::thread_rng()),
                    v_dist.sample(&mut rand::thread_rng()),
                ),
            })
            .insert(NewlyCreated)
            .insert(MagneticDipole { mFgF: 0.5 })
            .insert(Mass { value: 87.0 })
            // .insert_bundle(PbrBundle {
            //     mesh: meshes.add(Mesh::from(shape::Cube { size: 0.05 })),
            //     material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            //     transform: Transform::from_xyz(1.5, 0.5, 1.5),
            //     ..default()
            // })
            .insert(Rubidium87_780D2)
            ;
    }
}

fn setup(
    mut commands: Commands
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn_bundle(camera).insert(DemoCamera::default());


    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}