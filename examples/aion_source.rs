
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
use atomecs::magnetic::uniform::UniformMagneticField;
use atomecs::shapes::Cuboid;
use atomecs::sim_region::{SimulationVolume, VolumeType, SimulationRegionPlugin};
use atomecs::species::{Strontium88_461};
use atomecs_demos::atoms::{add_meshes_to_atoms, EmissiveColorConfig, MaterialColorConfig};
use atomecs_demos::camera::{control_camera, DemoCamera};
use atomecs_demos::lasers::add_meshes_to_lasers;
use atomecs_demos::{BevyAtomECSPlugin};
use bevy::render::camera::{Viewport, Projection, CameraProjection};
use bevy_egui::{EguiContext, egui, EguiPlugin};
use nalgebra::{Vector3, Unit};
use bevy::prelude::*;
use rand_distr::{Normal, Distribution};


const BEAM_NUMBER : usize = 6;

fn main() {

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    app.add_plugin(atomecs::integrator::IntegrationPlugin);
    app.add_plugin(atomecs::initiate::InitiatePlugin);
    app.add_plugin(atomecs::magnetic::MagneticsPlugin);
    app.add_plugin(LaserPlugin::<{BEAM_NUMBER}>);
    app.add_plugin(LaserCoolingPlugin::<Strontium88_461, {BEAM_NUMBER}>::default());
    app.add_plugin(SimulationRegionPlugin);
    app.add_plugin(BevyAtomECSPlugin);
    app.add_system(atomecs::output::console_output::console_output);
    app.add_system(atomecs::bevy_bridge::copy_positions);
    app.add_startup_system(setup_world);
    app.add_system(add_meshes_to_atoms::<Strontium88_461>);
    //app.add_system(atomecs_demos::atoms::update_emissive_color::<Strontium88_461>);
    app.add_system(add_meshes_to_lasers::<Strontium88_461>);
    app.add_system(create_atoms);
    app.add_system(control_camera);
    app.add_startup_system(setup_camera);
    //app.add_startup_system(add_atomecs_watermark);
    app.add_startup_system(spawn_cad);
    app.insert_resource(atomecs::bevy_bridge::Scale { 0: 7e1 });
    app.insert_resource(Timestep { delta: 2.0e-5 });
    app.insert_resource(EmissionForceOption::On(EmissionForceConfiguration {
        explicit_threshold: 5,
    }));
    app.insert_resource(EmissiveColorConfig { factor: 8.0 });
    app.insert_resource(MaterialColorConfig { factor: 1.0 });
    app.insert_resource(ScatteringFluctuationsOption::On);
    app.init_resource::<ExperimentConfiguration>();
    app.add_system(experiment_controls);
    app.add_system(update_cooling_beams);
    app.add_system(update_push_beam);
    app.add_system(update_magnetic_fields);
    app.add_system(update_cad);
    app
    .insert_resource(WindowDescriptor {
      fit_canvas_to_parent: true,
      ..default()
    });
    app.run();
}

pub fn setup_world(mut commands: Commands) {

    // Create magnetic field.
    commands.spawn()
        .insert(QuadrupoleField2D::gauss_per_cm(
            27.0, // value overridden below.
            Vector3::x_axis(), 
            Unit::new_normalize(Vector3::new(0.0, 1.0, 1.0))
        ))
        .insert(UniformMagneticField::gauss(Vector3::new(0.0,0.0,0.0)))
        .insert(Position::default());

    // Push beam along z
    let push_beam_radius = 4e-3;
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
        ))
        .insert(PushBeam::default());

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
        ))
        .insert(MOTBeam::default());
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
        ))
        .insert(MOTBeam::default());
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
        ))
        .insert(MOTBeam::default());
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
        ))
        .insert(MOTBeam::default());

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
    let camera = Camera3dBundle {
        projection: OrthographicProjection { scale: 0.03, near: -10.0, ..default() }.into(),
        transform: Transform::from_xyz(4.0, 4.0, 3.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

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

    commands.insert_resource(AmbientLight { brightness: 0.1, ..default() });
}

#[derive(Component)]
pub struct CAD;

fn spawn_cad(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(
            SceneBundle { 
                scene: asset_server.load("models/aion_source.gltf#Scene0"),
                transform: Transform {
                    scale: Vec3::new(0.6,0.6,0.6),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0),
                    ..default()
                },
                ..default()
            }
        )
        .insert(CAD);
}

pub struct ExperimentConfiguration {
    pub cooling_beam_detuning: f64,
    pub cooling_beam_power: f64,
    pub push_beam_power: f64,
    pub push_beam_detuning: f64,
    pub bias_field_x: f64,
    pub bias_field_y: f64,
    pub bias_field_z: f64,
    pub quad_gradient: f64,
    pub show_cad: bool,
}
impl Default for ExperimentConfiguration {
    fn default() -> Self {
        ExperimentConfiguration {
            cooling_beam_detuning: -40.0,
            cooling_beam_power: 230.0,
            push_beam_power: 20.0,
            push_beam_detuning: -103.0,
            bias_field_x: 0.0,
            bias_field_y: 0.0,
            bias_field_z: 0.0,
            quad_gradient: 27.0,
            show_cad: true
        }
    }
}

#[derive(Component, Default)]
pub struct MOTBeam;

#[derive(Component, Default)]
pub struct PushBeam;

fn experiment_controls(
    mut egui_context: ResMut<EguiContext>,
    mut config: ResMut<ExperimentConfiguration>,
    mut camera_query: Query<(&mut Camera, &mut Projection)>
) {
    
    let rect = egui::SidePanel::right("right")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("AION Source");
            ui.add(egui::Hyperlink::from_label_and_url(
                "powered by AtomECS",
                "https://github.com/TeamAtomECS/AtomECS/",
            ));
            ui.add_space(0.1);
            ui.label("A simulation of the cold atom source used to laser cool and capture atoms ejected from a hot oven.");
            ui.label("Click and drag the left mouse button to rotate the view.");
            ui.add_space(0.1);
            ui.label("Cooling Beams:");
            ui.add(egui::Slider::new(&mut config.cooling_beam_detuning, -120.0..=-15.0).text("Cooling beam detuning (MHz)"));
            ui.add(egui::Slider::new(&mut config.cooling_beam_power, 0.0..=230.0).text("Cooling beam power (mW)"));
            ui.add_space(0.1);
            ui.label("Push Beam:");
            ui.add(egui::Slider::new(&mut config.push_beam_detuning, -400.0..=100.0).text("Push beam detuning (MHz)"));
            ui.add(egui::Slider::new(&mut config.push_beam_power, 0.0..=30.0).text("Push beam power (mW)"));
            ui.add_space(0.1);
            ui.label("Magnetic fields:");
            ui.add(egui::Slider::new(&mut config.quad_gradient, 0.0..=80.0).text("Quadrupole gradient (G/cm)"));
            ui.add(egui::Slider::new(&mut config.bias_field_x, -30.0..=30.0).text("Bias field, x (G)"));
            ui.add(egui::Slider::new(&mut config.bias_field_y, -30.0..=30.0).text("Bias field, y (G)"));
            ui.add(egui::Slider::new(&mut config.bias_field_z, -30.0..=30.0).text("Bias field, z (G)"));
            ui.add_space(1.0);
            ui.label("Miscellaneous:");
            ui.add(egui::Checkbox::new(&mut config.show_cad, "Show CAD?"));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        }).response.rect;
    for (mut camera, mut projection) in camera_query.iter_mut() {
        camera.viewport = Some(Viewport {
            physical_size: UVec2 { x: rect.left() as u32, y: rect.height() as u32 },
            ..default()
        });
        projection.update(rect.left(), rect.height());
    }
}

fn update_cooling_beams(
    mut query: Query<(&mut CoolingLight, &mut GaussianBeam), With<MOTBeam>>,
    config: Res<ExperimentConfiguration>
) {
    for (mut light, mut gaussian) in query.iter_mut() {
        let wavelength = CoolingLight::for_transition::<Strontium88_461>(config.cooling_beam_detuning, 1).wavelength;
        light.wavelength = wavelength;
        gaussian.power = 1e-3 * config.cooling_beam_power;
    }
}

fn update_push_beam(
    mut query: Query<(&mut CoolingLight, &mut GaussianBeam), With<PushBeam>>,
    config: Res<ExperimentConfiguration>
) {
    for (mut light, mut gaussian) in query.iter_mut() {
        let wavelength = CoolingLight::for_transition::<Strontium88_461>(config.push_beam_detuning, 1).wavelength;
        light.wavelength = wavelength;
        gaussian.power = 1e-3 * config.push_beam_power;
    }
}

fn update_magnetic_fields(
    mut query: Query<(&mut QuadrupoleField2D, &mut UniformMagneticField)>,
    config: Res<ExperimentConfiguration>
) {
    for (mut quad, mut uniform) in query.iter_mut() {
        quad.gradient = 0.01 * config.quad_gradient;
        uniform.field = UniformMagneticField::gauss(Vector3::new(config.bias_field_x, config.bias_field_y, config.bias_field_z)).field;
    }
}

fn update_cad(
    mut query: Query<&mut Visibility, With<CAD>>,
    config: Res<ExperimentConfiguration>
) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = config.show_cad;
    }
}