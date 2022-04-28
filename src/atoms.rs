//! Functionality for rendering atoms.

use bevy::prelude::*;
use atomecs::atom::Atom;

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