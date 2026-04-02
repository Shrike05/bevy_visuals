use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use fast_surface_nets::{
    ndshape::{RuntimeShape, Shape},
    *,
};

use crate::sdf::SdfTree;

pub fn spawn_surface_nets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sdf_query: Query<&SdfTree>,
) {
    let mat = materials.add(Color::srgb(1., 1., 1.));

    for sdf in sdf_query {
        let mesh = meshes.add(generate_sdf_mesh(sdf));

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(Vec3::ZERO),
        ));
    }
}

fn generate_sdf_mesh(sdf: &SdfTree) -> Mesh {
    let side = 128;
    let shape = RuntimeShape::<u32, 3>::new([side, side, side]);
    let mut samples = vec![1.0; shape.size() as usize];

    for i_ in 0..shape.size() {
        let i = i_ as f32;
        let x = i % side as f32;
        let y = (i / side as f32) % side as f32;
        let z = i / (side * side) as f32;
        let pos = (Vec3::new(x, y, z) / (side as f32 - 1.)) - Vec3::splat(0.5);
        let p = sdf.evaluate_from_root(pos);
        samples[i_ as usize] = p;
    }

    let mut buffer = SurfaceNetsBuffer::default();
    surface_nets(&samples, &shape, [0; 3], [side - 1; 3], &mut buffer);

    let scale_factor = 1.0 / side as f32;
    for position in buffer.positions.iter_mut() {
        position[0] *= scale_factor;
        position[1] *= scale_factor;
        position[2] *= scale_factor;
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, buffer.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, buffer.normals);
    mesh.insert_indices(Indices::U32(buffer.indices));

    mesh
}
