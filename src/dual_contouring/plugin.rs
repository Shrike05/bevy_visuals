use bevy::prelude::*;

use crate::dual_contouring::systems::*;

pub struct ContouringPlugin;

impl Plugin for ContouringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sdfs);
    }
}
