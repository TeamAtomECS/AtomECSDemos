//! Things for visualising lasers

use atomecs::{laser::gaussian::GaussianBeam, laser_cooling::transition::AtomicTransition, bevy_bridge::Scale};
use bevy::{prelude::*, pbr::NotShadowCaster};

use crate::atoms::{MaterialColorConfig, get_color};

/// adds meshes to atoms so they can be rendered.
pub fn add_meshes_to_lasers<T: AtomicTransition>(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    color_factor: Res<MaterialColorConfig>,
    scale: Res<Scale>,
    query: Query<(Entity, &GaussianBeam), Without<Handle<Mesh>>>
) {
    let mut color = get_color::<T>() * color_factor.factor;
    color.set_a(0.03);
    for (entity, beam) in query.iter() {
        let pos = beam.intersection * scale.0;
        let mut mat: StandardMaterial = color.into();
        mat.alpha_mode = AlphaMode::Blend;
        let rotation = Transform::default().looking_at(Vec3::new(
            (beam.direction[0] ) as f32,
            (beam.direction[1] ) as f32,
            (beam.direction[2] ) as f32
        ), 
        Vec3::Y).rotation * Quat::from_rotation_x(std::f32::consts::PI / 2.0);
        commands.entity(entity).insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: (beam.e_radius * scale.0) as f32, depth: 40.0, ..Default::default() })),
            material: materials.add(mat),
            transform: Transform::from_xyz(pos[0] as f32, pos[1] as f32, pos[2] as f32).with_rotation(rotation),
            ..default()
        })
        .insert(NotShadowCaster);
    }
}