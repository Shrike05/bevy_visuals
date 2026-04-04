use bevy::{
    core_pipeline::prepass::DepthPrepass, prelude::*, render::render_resource::TextureUsages,
};

use crate::camera::post_processing::PostProcessSettings;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d {
            depth_texture_usages: (TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING)
                .into(),
            ..Default::default()
        },
        Transform::from_xyz(0., 10., 4.).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
        PostProcessSettings::new(0.02),
        Msaa::Off,
        DepthPrepass,
    ));
}

pub fn setup_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1., 0.8, 1.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
