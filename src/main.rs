use bevy::prelude::*;

mod camera;
mod dual_contouring;
mod scene;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        dual_contouring::ContouringPlugin,
        DefaultPlugins,
        camera::CameraPlugin,
    ));

    app.run();
}
