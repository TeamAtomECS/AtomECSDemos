//! Functionality for rendering atoms.

use bevy::prelude::*;
use atomecs::{atom::{Atom, Position}, laser_cooling::{transition::AtomicTransition, photons_scattered::TotalPhotonsScattered}, integrator::Timestep, bevy_bridge::Scale};
use nalgebra::clamp;

/// adds meshes to atoms so they can be rendered.
pub fn add_meshes_to_atoms<T: AtomicTransition>(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    color_factor: Res<MaterialColorConfig>,
    scale: Res<Scale>,
    query: Query<(Entity, &Position), (With<Atom>, Without<Handle<Mesh>>)>
) {
    let color = get_color::<T>() * color_factor.factor;
    for (entity, pos) in query.iter() {
        let p = pos.pos * scale.0;
        commands.entity(entity).insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.05, subdivisions: 0 })),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(p[0] as f32, p[1] as f32, p[2] as f32),
            ..default()
        });
    }
}

#[derive(Resource)]
pub struct MaterialColorConfig {
    pub factor: f32
}
impl Default for MaterialColorConfig {
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

pub fn get_color<T : AtomicTransition>() -> Color {
    let wavelength_nm = T::wavelength() * 1e9;
    let color = 
        if wavelength_nm < 480.0 {
            Color::BLUE
        } else if wavelength_nm < 550.0 {
            Color::GREEN
        } else {
            Color::RED
        };
    return color;
}

#[derive(Resource)]
pub struct EmissiveColorConfig {
    pub factor: f32
}
impl Default for EmissiveColorConfig {
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

pub fn update_emissive_color<T : AtomicTransition>(
    query: Query<(&Handle<StandardMaterial>, &TotalPhotonsScattered<T>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time_step: Res<Timestep>,
    config: Res<EmissiveColorConfig>
)
where T : Default + Copy + Component
{
    for (material, total_scattered) in query.iter() {
        let expected_max = (T::gamma() / 2.0 * time_step.delta) as f32;
        match materials.get_mut(material) {
            None => {}
            Some(material_instance) => {
                let emissive_strength  = clamp(config.factor * total_scattered.total as f32 / expected_max, 0.0, 1.0);
                material_instance.emissive = material_instance.base_color * emissive_strength;
            }
        }
    }
}