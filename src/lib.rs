pub mod atoms;

use bevy::prelude::*;

pub struct BevyAtomECSPlugin;
impl Plugin for BevyAtomECSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(atoms::add_meshes_to_atoms);
    }
}