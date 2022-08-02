pub mod atoms;
pub mod camera;
pub mod lasers;

use bevy::prelude::*;

pub struct BevyAtomECSPlugin;
impl Plugin for BevyAtomECSPlugin {
    fn build(&self, app: &mut App) {
        //app.add_system(atoms::add_meshes_to_atoms);
        app.init_resource::<atoms::MaterialColorConfig>();
    }
}

pub fn add_atomecs_watermark(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
        commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "AtomECS",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        );
}