
//! A 2D+ mot configuration, loaded directly from oven.

use atomecs::atom::{Atom, Force, Mass};
use atomecs::atom::{Position, Velocity};
use atomecs::initiate::NewlyCreated;
use atomecs::integrator::Timestep;
use atomecs::laser::LaserPlugin;
use atomecs::laser::gaussian::GaussianBeam;
use atomecs::laser_cooling::force::{EmissionForceOption, EmissionForceConfiguration};
use atomecs::laser_cooling::photons_scattered::ScatteringFluctuationsOption;
use atomecs::laser_cooling::{CoolingLight, LaserCoolingPlugin};
use atomecs::magnetic::quadrupole::QuadrupoleField2D;
use atomecs::shapes::Cuboid;
use atomecs::sim_region::{SimulationVolume, VolumeType, SimulationRegionPlugin};
use atomecs::species::{Strontium88_461};
use atomecs_demos::atoms::{add_meshes_to_atoms, update_emissive_color, EmissiveColorConfig, MaterialColorConfig};
use atomecs_demos::camera::{control_camera, DemoCamera};
use atomecs_demos::{add_atomecs_watermark, BevyAtomECSPlugin};
use nalgebra::{Vector3, Unit};
use bevy::prelude::*;
use rand_distr::{Normal, Distribution};


const BEAM_NUMBER : usize = 6;

fn main() {

    let mut app = App::new();
    app.add_plugin(atomecs::integrator::IntegrationPlugin);
    app.add_plugin(atomecs::initiate::InitiatePlugin);
    app.add_plugin(atomecs::magnetic::MagneticsPlugin);
    app.add_plugin(LaserPlugin::<{BEAM_NUMBER}>);
    app.add_plugin(LaserCoolingPlugin::<Strontium88_461, {BEAM_NUMBER}>::default());
    app.add_plugin(SimulationRegionPlugin);
    app.add_plugin(BevyAtomECSPlugin);
    app.add_system(atomecs::output::console_output::console_output);
    app.add_plugins(DefaultPlugins);
    app.add_system(atomecs::bevy_bridge::copy_positions);
    app.add_startup_system(setup_world);
    app.add_system(add_meshes_to_atoms::<Strontium88_461>);
    //app.add_system(update_emissive_color::<Strontium88_461>);
    app.add_system(create_atoms);
    app.add_system(control_camera);
    app.add_startup_system(setup_camera);
    app.add_startup_system(add_atomecs_watermark);
    app.add_startup_system(spawn_cad);
    app.insert_resource(atomecs::bevy_bridge::Scale { 0: 7e1 });
    app.insert_resource(Timestep { delta: 2.0e-5 });
    app.insert_resource(EmissionForceOption::On(EmissionForceConfiguration {
        explicit_threshold: 5,
    }));
    app.insert_resource(EmissiveColorConfig { factor: 8.0 });
    app.insert_resource(MaterialColorConfig { factor: 0.5 });
    app.insert_resource(ScatteringFluctuationsOption::On);
    app.run();
}

pub fn setup_world(mut commands: Commands) {

    // Create magnetic field.
    commands.spawn()
        .insert(QuadrupoleField2D::gauss_per_cm(
            27.0, 
            Vector3::x_axis(), 
            Unit::new_normalize(Vector3::new(0.0, 1.0, 1.0))
        ))
        .insert(Position::default());

    // Push beam along z
    let push_beam_radius = 11e-3;
    let push_beam_power = 0.020;
    let push_beam_detuning = -103.0;

    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: push_beam_radius,
            power: push_beam_power,
            direction: Vector3::x(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            push_beam_detuning,
            -1,
        ));

    // Create cooling lasers.
    let detuning = -40.0;
    let power = 0.23;
    let radius = 17.0e-3;//33.0e-3 / (2.0 * 2.0_f64.sqrt()); // 33mm 1/e^2 diameter
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, 1.0, 1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, -1.0, -1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, 1.0, -1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            detuning,
            -1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: radius,
            power,
            direction: Vector3::new(0.0, -1.0, 1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            detuning,
            -1,
        ));

    // Use a simulation bound so that atoms that escape the capture region are deleted from the simulation.
    commands.spawn()
        .insert(Position {
            pos: Vector3::new(0.0, 0.0, 0.0),
        })
        .insert(Cuboid {
            half_width: Vector3::new(0.02, 0.1, 0.02),
        })
        .insert(SimulationVolume {
            volume_type: VolumeType::Inclusive,
        });

    // The simulation bound also now includes a small pipe to capture the 2D MOT output properly.
    commands.spawn()
        .insert(Position {
            pos: Vector3::new(0.05, 0.0, 0.0),
        })
        .insert(Cuboid {
            half_width: Vector3::new(0.05, 0.01, 0.01),
        })
        .insert(SimulationVolume {
            volume_type: VolumeType::Inclusive,
        });
}

fn create_atoms(mut commands: Commands) {
    let dist = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    // Add atoms
    for _ in 0..3 {
        commands.spawn()
            .insert(Position {
                pos: Vector3::new(
                    0.001*dist.sample(&mut rng), -0.08, 0.001*dist.sample(&mut rng)
                ),
            })
            .insert(Velocity {
                vel: Vector3::new(
                    dist.sample(&mut rng) * 10.0,
                    dist.sample(&mut rng) * 30.0 + 95.0,
                    dist.sample(&mut rng) * 10.0,
                ),
            })
            .insert(Force::default())
            .insert(Mass { value: 88.0 })
            .insert(Strontium88_461)
            .insert(Atom)
            .insert(NewlyCreated)
            ;
        }
    }


fn setup_camera(
    mut commands: Commands
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 10.0;
    camera.orthographic_projection.near = -10.0;
    camera.transform = Transform::from_xyz(4.0, 4.0, 3.5).looking_at(Vec3::ZERO, Vec3::Y);

    // camera
    commands.spawn_bundle(camera).insert(DemoCamera::default());

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
                near: -20.0 * HALF_SIZE,
                far: 20.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_y(2.2) * Quat::from_rotation_x(-1.2),
            ..default()
        },
        ..default()
    });
}

// Component that will be used to tag entities in the scene
#[derive(Component)]
struct EntityInMyScene;

fn spawn_cad(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)).with_scale(Vec3::new(0.6,0.6,0.6))))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/aion_source.gltf#Scene0"));
        });
}