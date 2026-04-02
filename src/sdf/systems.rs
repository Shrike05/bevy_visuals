use crate::sdf::*;
use bevy::prelude::*;

pub fn spawn_sdfs(mut commands: Commands) {
    let mut sdf = SdfTree::default();
    sdf.nodes.push(SDFNode::Sphere { radius: 0.3 });
    sdf.nodes.push(SDFNode::Box {
        dimensions: Vec3::ONE * 0.15,
    });
    sdf.nodes
        .push(SDFNode::Translation(Vec3::new(0.2, 0., 0.2), 1));
    sdf.nodes.push(SDFNode::SmoothUnion { a: 0, b: 2, k: 0.4 });

    sdf.root = Some(3);

    commands.spawn(sdf);
}
