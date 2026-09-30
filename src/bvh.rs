use crate::camera::Ray;
use crate::math::Vec3;
use crate::scene::{Capsule, Cube, Ellipsoid, Triangle};
use std::cmp::Ordering;

const LEAF_SIZE: usize = 6;
const EPSILON: f32 = 0.001;

pub struct Bvh {
    root: Node,
    triangle_root: Option<TriangleNode>,
    ellipsoid_root: Option<PrimitiveNode>,
    capsule_root: Option<PrimitiveNode>,
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
        let ellipsoid_bounds = Vec::new();
        let capsule_bounds = Vec::new();
        Self {
            root: Node::build(cubes, &mut indices),
            triangle_root: if triangle_indices.is_empty() {
                None
            } else {
                Some(TriangleNode::build(triangles, &mut triangle_indices))
            },
            ellipsoid_root: primitive_root(&ellipsoid_bounds),
            capsule_root: primitive_root(&capsule_bounds),
        }
    }

    pub fn build_scene(
        cubes: &[Cube],
        triangles: &[Triangle],
        ellipsoids: &[Ellipsoid],
        capsules: &[Capsule],
    ) -> Self {
        let mut bvh = Self::build(cubes, triangles);
        let ellipsoid_bounds: Vec<_> = ellipsoids
            .iter()
            .map(|shape| (shape.center - shape.radii, shape.center + shape.radii))
            .collect();
        let capsule_bounds: Vec<_> = capsules
            .iter()
            .map(|shape| {
                let radius = Vec3::new(shape.radius, shape.radius, shape.radius);
                (
                    component_min(shape.start, shape.end) - radius,
                    component_max(shape.start, shape.end) + radius,
                )
            })
            .collect();
        bvh.ellipsoid_root = primitive_root(&ellipsoid_bounds);
        bvh.capsule_root = primitive_root(&capsule_bounds);
        bvh
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

    pub fn nearest_ellipsoid(
        &self,
        ellipsoids: &[Ellipsoid],
        ray: Ray,
    ) -> Option<(usize, f32, Vec3)> {
        nearest_primitive(self.ellipsoid_root.as_ref(), ray, |index| {
            intersect_ellipsoid(ray, ellipsoids[index])
        })
    }

    pub fn nearest_capsule(&self, capsules: &[Capsule], ray: Ray) -> Option<(usize, f32, Vec3)> {
        nearest_primitive(self.capsule_root.as_ref(), ray, |index| {
            intersect_capsule(ray, capsules[index])
        })
    }

    pub fn any_ellipsoid_hit(&self, ellipsoids: &[Ellipsoid], ray: Ray) -> bool {
        any_primitive_hit(self.ellipsoid_root.as_ref(), ray, |index| {
            intersect_ellipsoid(ray, ellipsoids[index]).is_some()
        })
    }

    pub fn any_capsule_hit(&self, capsules: &[Capsule], ray: Ray) -> bool {
        any_primitive_hit(self.capsule_root.as_ref(), ray, |index| {
            intersect_capsule(ray, capsules[index]).is_some()
        })
    }
}

struct PrimitiveNode {
    min: Vec3,
    max: Vec3,
    kind: PrimitiveNodeKind,
}

enum PrimitiveNodeKind {
    Leaf(Vec<usize>),
    Branch(Box<PrimitiveNode>, Box<PrimitiveNode>),
}

fn primitive_root(bounds: &[(Vec3, Vec3)]) -> Option<PrimitiveNode> {
    if bounds.is_empty() {
        return None;
    }
    let mut indices: Vec<usize> = (0..bounds.len()).collect();
    Some(PrimitiveNode::build(bounds, &mut indices))
}

impl PrimitiveNode {
    fn build(bounds: &[(Vec3, Vec3)], indices: &mut [usize]) -> Self {
        let (min, max) = primitive_bounds_for(bounds, indices);
        if indices.len() <= LEAF_SIZE {
            return Self {
                min,
                max,
                kind: PrimitiveNodeKind::Leaf(indices.to_vec()),
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
            primitive_center_axis(bounds[*left], axis)
                .partial_cmp(&primitive_center_axis(bounds[*right], axis))
                .unwrap_or(Ordering::Equal)
        });
        let middle = indices.len() / 2;
        let (left, right) = indices.split_at_mut(middle);
        Self {
            min,
            max,
            kind: PrimitiveNodeKind::Branch(
                Box::new(Self::build(bounds, left)),
                Box::new(Self::build(bounds, right)),
            ),
        }
    }

    fn nearest<F>(
        &self,
        ray: Ray,
        closest: &mut f32,
        result: &mut Option<(usize, f32, Vec3)>,
        intersect: F,
    ) where
        F: Fn(usize) -> Option<(f32, Vec3)> + Copy,
    {
        let Some(near) = intersect_bounds(ray, self.min, self.max) else {
            return;
        };
        if near > *closest {
            return;
        }
        match &self.kind {
            PrimitiveNodeKind::Leaf(indices) => {
                for &index in indices {
                    if let Some((distance, normal)) = intersect(index) {
                        if distance > EPSILON && distance < *closest {
                            *closest = distance;
                            *result = Some((index, distance, normal));
                        }
                    }
                }
            }
            PrimitiveNodeKind::Branch(left, right) => {
                let left_near = intersect_bounds(ray, left.min, left.max);
                let right_near = intersect_bounds(ray, right.min, right.max);
                match (left_near, right_near) {
                    (Some(left_distance), Some(right_distance))
                        if right_distance < left_distance =>
                    {
                        right.nearest(ray, closest, result, intersect);
                        left.nearest(ray, closest, result, intersect);
                    }
                    _ => {
                        left.nearest(ray, closest, result, intersect);
                        right.nearest(ray, closest, result, intersect);
                    }
                }
            }
        }
    }

    fn any_hit<F>(&self, ray: Ray, intersects: F) -> bool
    where
        F: Fn(usize) -> bool + Copy,
    {
        if intersect_bounds(ray, self.min, self.max).is_none() {
            return false;
        }
        match &self.kind {
            PrimitiveNodeKind::Leaf(indices) => indices.iter().any(|&index| intersects(index)),
            PrimitiveNodeKind::Branch(left, right) => {
                left.any_hit(ray, intersects) || right.any_hit(ray, intersects)
            }
        }
    }
}

fn nearest_primitive<F>(
    root: Option<&PrimitiveNode>,
    ray: Ray,
    intersect: F,
) -> Option<(usize, f32, Vec3)>
where
    F: Fn(usize) -> Option<(f32, Vec3)> + Copy,
{
    let mut closest = f32::INFINITY;
    let mut result = None;
    if let Some(root) = root {
        root.nearest(ray, &mut closest, &mut result, intersect);
    }
    result
}

fn any_primitive_hit<F>(root: Option<&PrimitiveNode>, ray: Ray, intersects: F) -> bool
where
    F: Fn(usize) -> bool + Copy,
{
    root.map(|root| root.any_hit(ray, intersects))
        .unwrap_or(false)
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
                let left_near = intersect_bounds(ray, left.min, left.max);
                let right_near = intersect_bounds(ray, right.min, right.max);
                match (left_near, right_near) {
                    (Some(left_distance), Some(right_distance))
                        if right_distance < left_distance =>
                    {
                        right.nearest(triangles, ray, closest, result);
                        left.nearest(triangles, ray, closest, result);
                    }
                    _ => {
                        left.nearest(triangles, ray, closest, result);
                        right.nearest(triangles, ray, closest, result);
                    }
                }
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
                let left_near = intersect_bounds(ray, left.min, left.max);
                let right_near = intersect_bounds(ray, right.min, right.max);
                match (left_near, right_near) {
                    (Some(left_distance), Some(right_distance))
                        if right_distance < left_distance =>
                    {
                        right.nearest(cubes, ray, closest, result);
                        left.nearest(cubes, ray, closest, result);
                    }
                    _ => {
                        left.nearest(cubes, ray, closest, result);
                        right.nearest(cubes, ray, closest, result);
                    }
                }
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

fn primitive_bounds_for(bounds: &[(Vec3, Vec3)], indices: &[usize]) -> (Vec3, Vec3) {
    let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for &index in indices {
        min = component_min(min, bounds[index].0);
        max = component_max(max, bounds[index].1);
    }
    (min, max)
}

fn primitive_center_axis(bounds: (Vec3, Vec3), axis: usize) -> f32 {
    let center = (bounds.0 + bounds.1) * 0.5;
    match axis {
        0 => center.x,
        1 => center.y,
        _ => center.z,
    }
}

fn component_min(left: Vec3, right: Vec3) -> Vec3 {
    Vec3::new(
        left.x.min(right.x),
        left.y.min(right.y),
        left.z.min(right.z),
    )
}

fn component_max(left: Vec3, right: Vec3) -> Vec3 {
    Vec3::new(
        left.x.max(right.x),
        left.y.max(right.y),
        left.z.max(right.z),
    )
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

pub fn intersect_ellipsoid(ray: Ray, ellipsoid: Ellipsoid) -> Option<(f32, Vec3)> {
    let offset = ray.origin - ellipsoid.center;
    let origin = Vec3::new(
        offset.x / ellipsoid.radii.x,
        offset.y / ellipsoid.radii.y,
        offset.z / ellipsoid.radii.z,
    );
    let direction = Vec3::new(
        ray.direction.x / ellipsoid.radii.x,
        ray.direction.y / ellipsoid.radii.y,
        ray.direction.z / ellipsoid.radii.z,
    );
    let a = direction.dot(direction);
    let half_b = origin.dot(direction);
    let c = origin.dot(origin) - 1.0;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    }
    let root = discriminant.sqrt();
    let near = (-half_b - root) / a;
    let far = (-half_b + root) / a;
    let distance = if near > EPSILON {
        near
    } else if far > EPSILON {
        far
    } else {
        return None;
    };
    let point = ray.at(distance) - ellipsoid.center;
    let normal = Vec3::new(
        point.x / (ellipsoid.radii.x * ellipsoid.radii.x),
        point.y / (ellipsoid.radii.y * ellipsoid.radii.y),
        point.z / (ellipsoid.radii.z * ellipsoid.radii.z),
    )
    .normalized();
    Some((distance, normal))
}

pub fn intersect_capsule(ray: Ray, capsule: Capsule) -> Option<(f32, Vec3)> {
    let axis = capsule.end - capsule.start;
    let axis_length_squared = axis.dot(axis);
    if axis_length_squared <= EPSILON * EPSILON {
        return intersect_sphere(ray, capsule.start, capsule.radius);
    }

    let offset = ray.origin - capsule.start;
    let axis_ray = axis.dot(ray.direction);
    let axis_offset = axis.dot(offset);
    let ray_offset = ray.direction.dot(offset);
    let offset_squared = offset.dot(offset);
    let a = axis_length_squared - axis_ray * axis_ray;
    let b = axis_length_squared * ray_offset - axis_offset * axis_ray;
    let c = axis_length_squared * offset_squared
        - axis_offset * axis_offset
        - capsule.radius * capsule.radius * axis_length_squared;
    let mut closest: Option<(f32, Vec3)> = None;

    if a.abs() > EPSILON {
        let discriminant = b * b - a * c;
        if discriminant >= 0.0 {
            let root = discriminant.sqrt();
            for distance in [(-b - root) / a, (-b + root) / a] {
                let height = axis_offset + distance * axis_ray;
                if distance > EPSILON && height >= 0.0 && height <= axis_length_squared {
                    let point = ray.at(distance);
                    let center = capsule.start + axis * (height / axis_length_squared);
                    if closest.as_ref().map(|hit| distance < hit.0).unwrap_or(true) {
                        closest = Some((distance, (point - center).normalized()));
                    }
                }
            }
        }
    }

    for (center, is_start) in [(capsule.start, true), (capsule.end, false)] {
        if let Some((distance, normal)) = intersect_sphere(ray, center, capsule.radius) {
            let point = ray.at(distance);
            let on_outer_hemisphere = if is_start {
                (point - capsule.start).dot(axis) <= 0.0
            } else {
                (point - capsule.end).dot(axis) >= 0.0
            };
            if on_outer_hemisphere && closest.as_ref().map(|hit| distance < hit.0).unwrap_or(true) {
                closest = Some((distance, normal));
            }
        }
    }
    closest
}

fn intersect_sphere(ray: Ray, center: Vec3, radius: f32) -> Option<(f32, Vec3)> {
    let offset = ray.origin - center;
    let half_b = offset.dot(ray.direction);
    let c = offset.dot(offset) - radius * radius;
    let discriminant = half_b * half_b - c;
    if discriminant < 0.0 {
        return None;
    }
    let root = discriminant.sqrt();
    let near = -half_b - root;
    let far = -half_b + root;
    let distance = if near > EPSILON {
        near
    } else if far > EPSILON {
        far
    } else {
        return None;
    };
    Some((distance, (ray.at(distance) - center).normalized()))
}
