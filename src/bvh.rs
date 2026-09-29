use crate::camera::Ray;
use crate::math::Vec3;
use crate::scene::{Cube, Triangle};
use std::cmp::Ordering;

const LEAF_SIZE: usize = 6;
const EPSILON: f32 = 0.001;

pub struct Bvh {
    root: Node,
    triangle_root: Option<TriangleNode>,
}

struct Node {
    min: Vec3,
    max: Vec3,
    kind: NodeKind,
}

enum NodeKind {
    Leaf(Vec<usize>),
    Branch(Box<Node>, Box<Node>),
}

impl Bvh {
    pub fn build(cubes: &[Cube], triangles: &[Triangle]) -> Self {
        let mut indices: Vec<usize> = (0..cubes.len()).collect();
        let mut triangle_indices: Vec<usize> = (0..triangles.len()).collect();
        Self {
            root: Node::build(cubes, &mut indices),
            triangle_root: if triangle_indices.is_empty() {
                None
            } else {
                Some(TriangleNode::build(triangles, &mut triangle_indices))
            },
        }
    }

    pub fn nearest(&self, cubes: &[Cube], ray: Ray) -> Option<(usize, f32, Vec3)> {
        let mut closest = f32::INFINITY;
        let mut result = None;
        self.root.nearest(cubes, ray, &mut closest, &mut result);
        result
    }

    pub fn any_hit(&self, cubes: &[Cube], ray: Ray) -> bool {
        self.root.any_hit(cubes, ray)
    }

    pub fn nearest_triangle(&self, triangles: &[Triangle], ray: Ray) -> Option<(usize, f32, Vec3)> {
        let mut closest = f32::INFINITY;
        let mut result = None;
        if let Some(root) = &self.triangle_root {
            root.nearest(triangles, ray, &mut closest, &mut result);
        }
        result
    }

    pub fn any_triangle_hit(&self, triangles: &[Triangle], ray: Ray) -> bool {
        self.triangle_root
            .as_ref()
            .map(|root| root.any_hit(triangles, ray))
            .unwrap_or(false)
    }
}

struct TriangleNode {
    min: Vec3,
    max: Vec3,
    kind: TriangleNodeKind,
}

enum TriangleNodeKind {
    Leaf(Vec<usize>),
    Branch(Box<TriangleNode>, Box<TriangleNode>),
}

impl TriangleNode {
    fn build(triangles: &[Triangle], indices: &mut [usize]) -> Self {
        let (min, max) = triangle_bounds_for(triangles, indices);
        if indices.len() <= LEAF_SIZE {
            return Self {
                min,
                max,
                kind: TriangleNodeKind::Leaf(indices.to_vec()),
            };
        }
        let extent = max - min;
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        };
        indices.sort_unstable_by(|left, right| {
            triangle_center_axis(triangles[*left], axis)
                .partial_cmp(&triangle_center_axis(triangles[*right], axis))
                .unwrap_or(Ordering::Equal)
        });
        let middle = indices.len() / 2;
        let (left, right) = indices.split_at_mut(middle);
        Self {
            min,
            max,
            kind: TriangleNodeKind::Branch(
                Box::new(Self::build(triangles, left)),
                Box::new(Self::build(triangles, right)),
            ),
        }
    }

    fn nearest(
        &self,
        triangles: &[Triangle],
        ray: Ray,
        closest: &mut f32,
        result: &mut Option<(usize, f32, Vec3)>,
    ) {
        let Some(near) = intersect_bounds(ray, self.min, self.max) else {
            return;
        };
        if near > *closest {
            return;
        }
        match &self.kind {
            TriangleNodeKind::Leaf(indices) => {
                for &index in indices {
                    if let Some((distance, normal)) = intersect_triangle(ray, triangles[index]) {
                        if distance > EPSILON && distance < *closest {
                            *closest = distance;
                            *result = Some((index, distance, normal));
                        }
                    }
                }
            }
            TriangleNodeKind::Branch(left, right) => {
                left.nearest(triangles, ray, closest, result);
                right.nearest(triangles, ray, closest, result);
            }
        }
    }

    fn any_hit(&self, triangles: &[Triangle], ray: Ray) -> bool {
        if intersect_bounds(ray, self.min, self.max).is_none() {
            return false;
        }
        match &self.kind {
            TriangleNodeKind::Leaf(indices) => indices
                .iter()
                .any(|&index| intersect_triangle(ray, triangles[index]).is_some()),
            TriangleNodeKind::Branch(left, right) => {
                left.any_hit(triangles, ray) || right.any_hit(triangles, ray)
            }
        }
    }
}

impl Node {
    fn build(cubes: &[Cube], indices: &mut [usize]) -> Self {
        let (min, max) = bounds_for(cubes, indices);
        if indices.len() <= LEAF_SIZE {
            return Self {
                min,
                max,
                kind: NodeKind::Leaf(indices.to_vec()),
            };
        }

        let extent = max - min;
        let axis = if extent.x >= extent.y && extent.x >= extent.z {
            0
        } else if extent.y >= extent.z {
            1
        } else {
            2
        };
        indices.sort_unstable_by(|left, right| {
            let a = center_axis(cubes[*left], axis);
            let b = center_axis(cubes[*right], axis);
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        });

        let middle = indices.len() / 2;
        let (left_indices, right_indices) = indices.split_at_mut(middle);
        Self {
            min,
            max,
            kind: NodeKind::Branch(
                Box::new(Self::build(cubes, left_indices)),
                Box::new(Self::build(cubes, right_indices)),
            ),
        }
    }

    fn nearest(
        &self,
        cubes: &[Cube],
        ray: Ray,
        closest: &mut f32,
        result: &mut Option<(usize, f32, Vec3)>,
    ) {
        let Some(near) = intersect_bounds(ray, self.min, self.max) else {
            return;
        };
        if near > *closest {
            return;
        }

        match &self.kind {
            NodeKind::Leaf(indices) => {
                for &index in indices {
                    if let Some((distance, normal)) = intersect_cube(ray, cubes[index]) {
                        let earlier_equal_hit = result
                            .map(|(current_index, current_distance, _)| {
                                distance == current_distance && index < current_index
                            })
                            .unwrap_or(false);
                        if distance > EPSILON && (distance < *closest || earlier_equal_hit) {
                            *closest = distance;
                            *result = Some((index, distance, normal));
                        }
                    }
                }
            }
            NodeKind::Branch(left, right) => {
                left.nearest(cubes, ray, closest, result);
                right.nearest(cubes, ray, closest, result);
            }
        }
    }

    fn any_hit(&self, cubes: &[Cube], ray: Ray) -> bool {
        if intersect_bounds(ray, self.min, self.max).is_none() {
            return false;
        }
        match &self.kind {
            NodeKind::Leaf(indices) => indices.iter().any(|&index| {
                intersect_cube(ray, cubes[index])
                    .map(|(distance, _)| distance > EPSILON)
                    .unwrap_or(false)
            }),
            NodeKind::Branch(left, right) => left.any_hit(cubes, ray) || right.any_hit(cubes, ray),
        }
    }
}

fn bounds_for(cubes: &[Cube], indices: &[usize]) -> (Vec3, Vec3) {
    let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for &index in indices {
        let cube = cubes[index];
        min.x = min.x.min(cube.min.x);
        min.y = min.y.min(cube.min.y);
        min.z = min.z.min(cube.min.z);
        max.x = max.x.max(cube.max.x);
        max.y = max.y.max(cube.max.y);
        max.z = max.z.max(cube.max.z);
    }
    (min, max)
}

fn center_axis(cube: Cube, axis: usize) -> f32 {
    match axis {
        0 => (cube.min.x + cube.max.x) * 0.5,
        1 => (cube.min.y + cube.max.y) * 0.5,
        _ => (cube.min.z + cube.max.z) * 0.5,
    }
}

fn triangle_bounds_for(triangles: &[Triangle], indices: &[usize]) -> (Vec3, Vec3) {
    let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for &index in indices {
        for vertex in triangles[index].vertices {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }
    }
    let padding = Vec3::new(EPSILON, EPSILON, EPSILON);
    (min - padding, max + padding)
}

fn triangle_center_axis(triangle: Triangle, axis: usize) -> f32 {
    let center = (triangle.vertices[0] + triangle.vertices[1] + triangle.vertices[2]) / 3.0;
    match axis {
        0 => center.x,
        1 => center.y,
        _ => center.z,
    }
}

fn intersect_bounds(ray: Ray, min: Vec3, max: Vec3) -> Option<f32> {
    let mut near = f32::NEG_INFINITY;
    let mut far = f32::INFINITY;
    for (origin, direction, low, high) in [
        (ray.origin.x, ray.direction.x, min.x, max.x),
        (ray.origin.y, ray.direction.y, min.y, max.y),
        (ray.origin.z, ray.direction.z, min.z, max.z),
    ] {
        if direction.abs() < 0.00001 {
            if origin < low || origin > high {
                return None;
            }
            continue;
        }
        let inverse = 1.0 / direction;
        let mut first = (low - origin) * inverse;
        let mut second = (high - origin) * inverse;
        if first > second {
            std::mem::swap(&mut first, &mut second);
        }
        near = near.max(first);
        far = far.min(second);
        if near > far {
            return None;
        }
    }
    if far > EPSILON {
        Some(near.max(0.0))
    } else {
        None
    }
}

pub fn intersect_cube(ray: Ray, cube: Cube) -> Option<(f32, Vec3)> {
    let mut near = f32::NEG_INFINITY;
    let mut far = f32::INFINITY;
    let mut normal = Vec3::default();
    for (origin, direction, low, high, low_normal, high_normal) in [
        (
            ray.origin.x,
            ray.direction.x,
            cube.min.x,
            cube.max.x,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            ray.origin.y,
            ray.direction.y,
            cube.min.y,
            cube.max.y,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            ray.origin.z,
            ray.direction.z,
            cube.min.z,
            cube.max.z,
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
    ] {
        if direction.abs() < 0.00001 {
            if origin < low || origin > high {
                return None;
            }
            continue;
        }
        let inverse = 1.0 / direction;
        let mut first = (low - origin) * inverse;
        let mut second = (high - origin) * inverse;
        let mut face_normal = low_normal;
        if inverse < 0.0 {
            std::mem::swap(&mut first, &mut second);
            face_normal = high_normal;
        }
        if first > near {
            near = first;
            normal = face_normal;
        }
        far = far.min(second);
        if near > far {
            return None;
        }
    }
    if near > EPSILON {
        Some((near, normal))
    } else if far > EPSILON {
        Some((far, -normal))
    } else {
        None
    }
}

pub fn intersect_triangle(ray: Ray, triangle: Triangle) -> Option<(f32, Vec3)> {
    let edge_a = triangle.vertices[1] - triangle.vertices[0];
    let edge_b = triangle.vertices[2] - triangle.vertices[0];
    let cross = ray.direction.cross(edge_b);
    let determinant = edge_a.dot(cross);
    if determinant.abs() < 0.000_001 {
        return None;
    }
    let inverse = 1.0 / determinant;
    let offset = ray.origin - triangle.vertices[0];
    let u = offset.dot(cross) * inverse;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = offset.cross(edge_a);
    let v = ray.direction.dot(q) * inverse;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = edge_b.dot(q) * inverse;
    if distance <= EPSILON {
        return None;
    }
    let w = 1.0 - u - v;
    let normal =
        (triangle.normals[0] * w + triangle.normals[1] * u + triangle.normals[2] * v).normalized();
    Some((distance, normal))
}
