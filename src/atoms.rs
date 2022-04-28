//! Functionality for rendering atoms.

use bevy::prelude::*;
use atomecs::{atom::Atom, laser_cooling::{transition::AtomicTransition, photons_scattered::TotalPhotonsScattered}, integrator::Timestep};

/// adds meshes to atoms so they can be rendered.
pub fn add_meshes_to_atoms(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, (With<Atom>, Without<Handle<Mesh>>)>
) {

    for entity in query.iter() {
        commands.entity(entity).insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 0.05, subdivisions: 0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(1.5, 0.5, 1.5),
            ..default()
        });
    }
}

pub fn update_emissive_color<T : AtomicTransition>(
    query: Query<(&Handle<StandardMaterial>, &TotalPhotonsScattered<T>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time_step: Res<Timestep>
)
where T : Default + Copy + Component
{
    for (material, total_scattered) in query.iter() {
        let expected_max = (T::gamma() / 2.0 * time_step.delta) as f32;
        match materials.get_mut(material) {
            None => {}
            Some(material_instance) => {
                
                material_instance.emissive = material_instance.base_color * (total_scattered.total as f32 / expected_max);
            }
        }
    }
}