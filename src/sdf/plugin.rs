use crate::sdf::systems::*;
use bevy::prelude::*;
pub struct SDFPlugin;

impl Plugin for SDFPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sdfs);
    }
}
