//! # Doppler Sweep
//!
//! Simulate a cloud of atoms in a 3D MOT to measure the Doppler temperature limit for laser cooling.
//!
//! The Doppler Limit depends on temperature, see eg https://journals.aps.org/prl/abstract/10.1103/PhysRevLett.61.169.
//!
//! Some parameters of the simulation can be set by writing a configuration file called `doppler.json`. This file
//! allows the user to control parameters, eg detuning. If the file is not written, a default detuning of 0.5 Gamma
//! is used, which corresponds to the minimum Doppler temperature.
//! 
//! cargo build --example doppler_limit --target wasm32-unknown-unknown --release
//! wasm-bindgen --out-dir target/web target/wasm32-unknown-unknown/release/examples/doppler_limit.wasm --target web

extern crate atomecs;
extern crate nalgebra;
use atomecs::atom::{Atom, Force, Mass, Position, Velocity};
use atomecs::initiate::NewlyCreated;
use atomecs::integrator::Timestep;
use atomecs::laser::LaserPlugin;
use atomecs::laser::gaussian::GaussianBeam;
use atomecs::laser_cooling::force::{EmissionForceConfiguration, EmissionForceOption};
use atomecs::laser_cooling::photons_scattered::ScatteringFluctuationsOption;
use atomecs::laser_cooling::{CoolingLight, LaserCoolingPlugin};
use atomecs::magnetic::quadrupole::QuadrupoleField3D;
use atomecs::species::{Rubidium87_780D2};
use atomecs_demos::atoms::add_meshes_to_atoms;
use atomecs_demos::{BevyAtomECSPlugin, add_atomecs_watermark};
use nalgebra::Vector3;
use rand_distr::{Distribution, Normal};
use bevy::prelude::*;

const BEAM_NUMBER : usize = 6;

pub struct DopperSimulationConfiguration {
    /// Detuning of laser beams, in units of MHz.
    pub detuning: f64,
    /// Number of simulation steps to evolve for.
    pub number_of_steps: i32,
}
impl Default for DopperSimulationConfiguration {
    fn default() -> Self {
        DopperSimulationConfiguration {
            detuning: -8.0,
            number_of_steps: 5000,
        }
    }
}

fn main() {

    let mut app = App::new();
    app.add_plugin(atomecs::integrator::IntegrationPlugin);
    app.add_plugin(atomecs::initiate::InitiatePlugin);
    app.add_plugin(atomecs::magnetic::MagneticsPlugin);
    app.add_plugin(LaserPlugin::<{BEAM_NUMBER}>);
    app.add_plugin(LaserCoolingPlugin::<Rubidium87_780D2, {BEAM_NUMBER}>::default());
    app.add_plugin(BevyAtomECSPlugin);
    app.add_system(add_meshes_to_atoms::<Rubidium87_780D2>);
    app.add_system(atomecs::output::console_output::console_output);
    app.add_plugins(DefaultPlugins);
    app.add_system(atomecs::bevy_bridge::copy_positions);
    app.add_startup_system(setup_world);
    app.add_startup_system(create_atoms);
    app.add_startup_system(setup_camera);
    //app.add_startup_system(add_atomecs_watermark);
    app.insert_resource(atomecs::bevy_bridge::Scale { 0: 1e3 });
    app.insert_resource(Timestep { delta: 2.0e-5 });
    app.insert_resource(EmissionForceOption::On(EmissionForceConfiguration {
        explicit_threshold: 5,
    }));
    app.insert_resource(ScatteringFluctuationsOption::On);
    app.insert_resource(WindowDescriptor {
            fit_canvas_to_parent: true,
            ..default()
        });
    app.run();
}

pub fn setup_world(mut commands: Commands) {

    let configuration = DopperSimulationConfiguration::default();

    // Create magnetic field.
    commands.spawn()
        .insert(QuadrupoleField3D::gauss_per_cm(0.001 * 18.2, Vector3::z()))
        .insert(Position {
            pos: Vector3::new(0.0, 0.0, 0.0),
        });

    // Create cooling lasers.
    let detuning = configuration.detuning;
    let power = 0.02;
    let radius = 66.7e-3 / (2.0_f64.sqrt());
    let beam_centre = Vector3::new(0.0, 0.0, 0.0);

    commands.spawn()
        .insert(GaussianBeam {
            intersection: beam_centre,
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, 0.0, 1.0),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Rubidium87_780D2>(
            detuning,
            -1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: beam_centre,
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, 0.0, -1.0),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Rubidium87_780D2>(
            detuning,
            -1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: beam_centre,
            e_radius: radius,
            power,
            direction: Vector3::new(-1.0, 0.0, 0.0),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Rubidium87_780D2>(
            detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: beam_centre,
            e_radius: radius,
            power,
            direction: Vector3::new(1.0, 0.0, 0.0),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Rubidium87_780D2>(
            detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: beam_centre,
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, 1.0, 0.0),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Rubidium87_780D2>(
            detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: beam_centre,
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, -1.0, 0.0),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Rubidium87_780D2>(
            detuning,
            1,
        ));
}

fn create_atoms(mut commands: Commands) {
    let vel_dist = Normal::new(0.0, 0.42).unwrap();
    let pos_dist = Normal::new(0.0, 1.2e-4).unwrap();
    let mut rng = rand::thread_rng();

    // Add atoms
    for _ in 0..1000 {
        commands.spawn()
            .insert(Position {
                pos: Vector3::new(
                    pos_dist.sample(&mut rng),
                    pos_dist.sample(&mut rng) - 0.002,
                    pos_dist.sample(&mut rng),
                ),
            })
            .insert(Velocity {
                vel: Vector3::new(
                    vel_dist.sample(&mut rng),
                    vel_dist.sample(&mut rng) + 3.5,
                    vel_dist.sample(&mut rng),
                ),
            })
            .insert(Force::default())
            .insert(Mass { value: 87.0 })
            .insert(Rubidium87_780D2)
            .insert(Atom)
            .insert(NewlyCreated)
            ;
        }
    }

fn setup_camera(
    mut commands: Commands
) {
    // set up the camera
    let mut camera = Camera3dBundle {
        projection: OrthographicProjection { scale: 0.01, ..default() }.into(),
        ..default()
    };
    camera.transform = Transform::from_xyz(4.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn_bundle(camera);

    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 30000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-2.0),
            ..default()
        },
        ..default()
    });
}