use bevy::prelude::*;

#[derive(Component, Default, Clone, Debug)]
pub struct SdfTree {
    pub nodes: Vec<SDFNode>,
    pub root: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum SDFNode {
    Sphere { radius: f32 },
    Box { dimensions: Vec3 },
    // References index in the nodes Vec
    Union(usize, usize),
    SmoothUnion { a: usize, b: usize, k: f32 },
    Translation(Vec3, usize),
}

impl SdfTree {
    pub fn evaluate_from_root(&self, p: Vec3) -> f32 {
        self.evaluate(self.root.unwrap_or(0), p)
    }
    pub fn evaluate(&self, index: usize, p: Vec3) -> f32 {
        match &self.nodes[index] {
            SDFNode::Sphere { radius } => p.length() - radius,

            SDFNode::Box { dimensions } => {
                let d = p.abs() - *dimensions;
                d.max(Vec3::ZERO).length() + d.max(Vec3::ZERO).max_element().min(0.0)
            }

            SDFNode::Union(a, b) => self.evaluate(*a, p).min(self.evaluate(*b, p)),

            SDFNode::SmoothUnion { a, b, k } => {
                let d1 = self.evaluate(*a, p);
                let d2 = self.evaluate(*b, p);
                let h = (0.5 + 0.5 * (d2 - d1) / k).clamp(0.0, 1.0);
                d2 * (1.0 - h) + d1 * h - k * h * (1.0 - h)
            }

            SDFNode::Translation(offset, node) => self.evaluate(*node, p - *offset),
        }
    }
}
