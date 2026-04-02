use bevy::prelude::*;

use crate::dual_contouring::types::*;

pub fn spawn_sdfs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_length(0.1));
    let mat = materials.add(Color::srgb(0., 0., 0.));

    let mut sdf = SdfTree::default();
    sdf.nodes.push(SDFNode::Sphere { radius: 1. });
    sdf.nodes.push(SDFNode::Sphere { radius: 1. });
    sdf.nodes
        .push(SDFNode::Translation(Vec3::new(0.6, 0., 0.6), 1));
    sdf.nodes.push(SDFNode::SmoothUnion { a: 0, b: 2, k: 0.4 });

    sdf.root = Some(3);

    let subdivisions = 30;
    let size = 5.;
    let ran = (0..subdivisions).map(|i| i as f32 / subdivisions as f32);
    for x in ran.clone() {
        for y in ran.clone() {
            for z in ran.clone() {
                let pos = Vec3::new(x, y, z);
                let t_pos = (pos - 0.5) * size;

                let v = sdf.evaluate_from_root(t_pos);
                if v.abs() < 0.1 {
                    commands.spawn((
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(mat.clone()),
                        Transform::from_xyz(pos.x, pos.y, pos.z),
                    ));
                }
            }
        }
    }
}

