use crate::surface_net::systems::*;
use bevy::prelude::*;

pub struct SurfaceNetPlugin;

impl Plugin for SurfaceNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_surface_nets);
    }
}
