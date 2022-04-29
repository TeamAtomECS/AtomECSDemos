pub mod atoms;
pub mod camera;

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
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "AtomECS",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        });
}