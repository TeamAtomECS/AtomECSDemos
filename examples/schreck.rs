
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
use atomecs::magnetic::grid::PrecalculatedMagneticFieldGrid;
use atomecs::shapes::Cuboid;
use atomecs::sim_region::{SimulationVolume, VolumeType, SimulationRegionPlugin};
use atomecs::species::{Strontium88_461};
use atomecs_demos::atoms::{add_meshes_to_atoms, EmissiveColorConfig, MaterialColorConfig};
use atomecs_demos::camera::{control_camera, DemoCamera};
use atomecs_demos::lasers::add_meshes_to_lasers;
use atomecs_demos::{add_atomecs_watermark, BevyAtomECSPlugin};
use nalgebra::{Vector3};
use bevy::prelude::*;
use rand_distr::{Normal, Distribution};
extern crate serde;


const BEAM_NUMBER : usize = 22;

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
    //app.add_system(atomecs_demos::atoms::update_emissive_color::<Strontium88_461>);
    app.add_system(add_meshes_to_lasers::<Strontium88_461>);
    app.add_system(create_atoms);
    app.add_system(control_camera);
    app.add_startup_system(setup_camera);
    app.add_startup_system(add_atomecs_watermark);
    app.add_startup_system(spawn_cad);
    app.insert_resource(atomecs::bevy_bridge::Scale { 0: 3e1 });
    app.insert_resource(Timestep { delta: 2.0e-5 });
    app.insert_resource(EmissionForceOption::On(EmissionForceConfiguration {
        explicit_threshold: 5,
    }));
    app.insert_resource(EmissiveColorConfig { factor: 8.0 });
    app.insert_resource(MaterialColorConfig { factor: 1.0 });
    app.insert_resource(ScatteringFluctuationsOption::On);
    app.run();
}

pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Create magnetic field.
    // commands.spawn()
    //     .insert(QuadrupoleField2D::gauss_per_cm(
    //         27.0, 
    //         Vector3::x_axis(), 
    //         Unit::new_normalize(Vector3::new(0.0, 1.0, 1.0))
    //     ))
    //     .insert(Position::default());
    let grid: PrecalculatedMagneticFieldGrid = serde_json::from_str(SLOWER_FIELD)
            .expect("Could not load magnetic field grid from json file.");
    commands.spawn().insert(grid);


    // Zeeman slowing beam along x
    let zeeman_slower_e2_diameter = 25e-3;
    let zeeman_slower_power = 0.07;
    let zeeman_slower_detuning = -450.0;
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: zeeman_slower_e2_diameter / 2.0 / 2.0_f64.sqrt(),
            power: zeeman_slower_power,
            direction: -Vector3::x(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            zeeman_slower_detuning,
            -1,
        ));

    // Transverse cooling region.
    let tc_detuning = -18.0;
    let tc_power = 0.013;
    let tc_diameter = 23.0e-3;//33.0e-3 / (2.0 * 2.0_f64.sqrt()); // 33mm 1/e^2 diameter
    let tc_pos = -1.7;
    let tc_stride = 0.03;
    for i in 0..2 {
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(tc_pos + i as f64 * tc_stride, 0.0, 0.0),
            e_radius: tc_diameter / 2.0 / 2.0_f64.sqrt(),
            power: tc_power,
            direction: Vector3::new(0.0, 0.0, 1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            tc_detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(tc_pos + i as f64 * tc_stride, 0.0, 0.0),
            e_radius: tc_diameter / 2.0 / 2.0_f64.sqrt(),
            power: tc_power,
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            tc_detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(tc_pos + i as f64 * tc_stride, 0.0, 0.0),
            e_radius: tc_diameter / 2.0 / 2.0_f64.sqrt(),
            power: tc_power,
            direction: Vector3::new(0.0, 1.0, 0.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            tc_detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(tc_pos + i as f64 * tc_stride, 0.0, 0.0),
            e_radius: tc_diameter / 2.0 / 2.0_f64.sqrt(),
            power: tc_power,
            direction: Vector3::new(0.0, -1.0, 0.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            tc_detuning,
            1,
        ));
    }


    // MOT region
    let blue_mot_detuning = -25.0;
    let blue_mot_power = 0.0105;
    let blue_mot_e2_diameter = 22.0e-3;//33.0e-3 / (2.0 * 2.0_f64.sqrt()); // 33mm 1/e^2 diameter
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: blue_mot_e2_diameter / 2.0 / 2.0_f64.sqrt(),
            power: blue_mot_power,
            direction: Vector3::new(1.0, 0.0, 1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            blue_mot_detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: blue_mot_e2_diameter / 2.0 / 2.0_f64.sqrt(),
            power: blue_mot_power,
            direction: Vector3::new(1.0, 0.0, -1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            blue_mot_detuning,
            1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: blue_mot_e2_diameter / 2.0 / 2.0_f64.sqrt(),
            power: blue_mot_power,
            direction: Vector3::new(-1.0, 0.0, -1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            blue_mot_detuning,
            -1,
        ));
    commands.spawn()
        .insert(GaussianBeam {
            intersection: Vector3::new(0.0, 0.0, 0.0),
            e_radius: blue_mot_e2_diameter / 2.0 / 2.0_f64.sqrt(),
            power: blue_mot_power,
            direction: Vector3::new(-1.0, 0.0, 1.0).normalize(),
            rayleigh_range: f64::INFINITY,
            ellipticity: 0.0,
        })
        .insert(CoolingLight::for_transition::<Strontium88_461>(
            blue_mot_detuning,
            -1,
        ));

    // Define simulation bounds
    //  1. Zeeman slower pipe
    commands.spawn()
        .insert(Position {
            pos: Vector3::new(-1.0, 0.0, 0.0),
        })
        .insert(Cuboid {
            half_width: Vector3::new(1.1, 0.1, 0.1),
        })
        .insert(SimulationVolume {
            volume_type: VolumeType::Inclusive,
        });
}

fn create_atoms(mut commands: Commands) {
    let dist = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    let oven_position = -1.9; //m
    // Add atoms
    for _ in 0..3 {
        commands.spawn()
            .insert(Position {
                pos: Vector3::new(
                    oven_position, 0.001*dist.sample(&mut rng), 0.001*dist.sample(&mut rng)
                ),
            })
            .insert(Velocity {
                vel: Vector3::new(
                    dist.sample(&mut rng) * 80.0 + 350.0,
                    dist.sample(&mut rng) * 5.0,
                    dist.sample(&mut rng) * 5.0,
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
    camera.orthographic_projection.scale = 23.0;
    camera.orthographic_projection.near = -30.0;
    let look_at_target = Vec3::new(-1.0,0.0,0.0);
    camera.transform = Transform::from_xyz(4.0, 4.0, 3.5).looking_at(look_at_target, Vec3::Y);

    // camera
    commands.spawn_bundle(camera).insert(DemoCamera::new(8.0, look_at_target));

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

    commands.insert_resource(AmbientLight { brightness: 0.1, ..default() });
}

// Component that will be used to tag entities in the scene
#[derive(Component)]
struct EntityInMyScene;

fn spawn_cad(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // commands
    //     .spawn_bundle(TransformBundle::from(Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)).with_scale(Vec3::new(0.6,0.6,0.6))))
    //     .with_children(|parent| {
    //         parent.spawn_scene(asset_server.load("models/aion_source.gltf#Scene0"));
    //     });
}

//yeah its messy, but so is loading raw json via bevy asset lib right now.
static SLOWER_FIELD: &str = "{\"extent_spatial\":[1.4,0.1,0.1],\"extent_cells\":[100,1,1],\"position\":[-0.6,0,0],\"grid\":[[0,0,0],[-0.00010356086113661874,0,0],[-0.00012131415161718194,0,0],[-5.3259871441689581E-5,0,0],[8.8390279697289322E-5,0,0],[0.00028248096720701575,0,0],[0.00059618220775245544,0,0],[0.0010902402249203598,0,0],[0.0013788955775487766,0,0],[0.0018677685950413221,0,0],[0.003386332460406539,0,0],[0.0066049382716049368,0,0],[0.011197929710326406,0,0],[0.016485182402537775,0,0],[0.021948657063441273,0,0],[0.028529927372902587,0,0],[0.035079994929398055,0,0],[0.03978910006090907,0,0],[0.041496619083395941,0,0],[0.041206110693714007,0,0],[0.040275945868232368,0,0],[0.039806049475470971,0,0],[0.0392482853223594,0,0],[0.038576111075039762,0,0],[0.037939839162979654,0,0],[0.037388889404193971,0,0],[0.036840539874822095,0,0],[0.036219383921863253,0,0],[0.0354419761331301,0,0],[0.034516414012587877,0,0],[0.033588251662631838,0,0],[0.03280424590770474,0,0],[0.0321792344421668,0,0],[0.031629629629629633,0,0],[0.031080024817092468,0,0],[0.030494949494949495,0,0],[0.029909090909090909,0,0],[0.029323232323232327,0,0],[0.028737373737373739,0,0],[0.028151515151515153,0,0],[0.027565656565656568,0,0],[0.026979848479695433,0,0],[0.026425939839162978,0,0],[0.025879831886871985,0,0],[0.025266117969821675,0,0],[0.024515401953418486,0,0],[0.023641246667264425,0,0],[0.022717687022247784,0,0],[0.02181818181818182,0,0],[0.020939393939393938,0,0],[0.020060606060606063,0,0],[0.019181818181818178,0,0],[0.018303030303030304,0,0],[0.017424242424242422,0,0],[0.016545454545454547,0,0],[0.015666666666666662,0,0],[0.014787878787878789,0,0],[0.013909090909090916,0,0],[0.013030303030303034,0,0],[0.012208883035023233,0,0],[0.011413362272866405,0,0],[0.010493030498846236,0,0],[0.0093000425641992825,0,0],[0.0078343350864012087,0,0],[0.0062750193497056062,0,0],[0.0048057475066963975,0,0],[0.0033148148148148156,0,0],[0.0018667859082612719,0,0],[0.00072558407253846236,0,0],[0.00015717783899602084,0,0],[9.4712557675520967E-5,0,0],[0.00013235610878708495,0,0],[-8.039068369646871E-5,0,0],[-0.00011370051911833379,0,0],[-0.00041223375474982267,0,0],[-0.0017677602470990736,0,0],[-0.0047691381728724976,0,0],[-0.0089821673525377153,0,0],[-0.013652836909035225,0,0],[-0.018536119278696511,0,0],[-0.025158216178724331,0,0],[-0.030755822689707009,0,0],[-0.031856213394015648,0,0],[-0.025674242681894964,0,0],[-0.015363107660628206,0,0],[-0.0060504607342684695,0,0],[-0.0016689288559505863,0,0],[-0.00027436903469961536,0,0],[-0.00015912208504801175,0,0],[0.00012212318058660361,0,0],[0.00029451540195341865,0,0],[0.00015488009366185232,0,0],[4.8438677150042879E-6,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0],[0,0,0]]}";