use bevy::prelude::*;

mod camera;
mod scene;
mod sdf;
mod surface_net;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        sdf::SDFPlugin,
        surface_net::SurfaceNetPlugin,
        DefaultPlugins,
        camera::CameraPlugin,
    ));

    app.run();
}
