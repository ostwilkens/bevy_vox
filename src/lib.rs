mod loader;
use bevy::prelude::{AddAsset, AppBuilder, Plugin};
pub use loader::*;

/// Adds support for Vox file loading to Apps
#[derive(Default)]
pub struct VoxPlugin;

impl Plugin for VoxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<VoxLoader>();
    }
}
