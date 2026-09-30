use crate::math::Vec3;
use crate::scene::{Capsule, Cube, Ellipsoid, Scene, Triangle};

const ROBE: usize = 5;
const CLOUD_RED: usize = 7;
const INK: usize = 10;
const METAL: usize = 11;
const CLOUD_WHITE: usize = 15;
const PAIN_SKIN: usize = 26;
const RINNEGAN: usize = 27;
const PAIN_HAIR: usize = 28;
const CRATER_EARTH: usize = 17;

pub fn add_six_paths(scene: &mut Scene) {
    let paths = [
        pain_position(-8.0, 18.5, 1usize),
        pain_position(-4.8, 17.2, 2usize),
        pain_position(-1.6, 16.4, 0usize),
        pain_position(1.6, 16.4, 3usize),
        pain_position(4.8, 17.2, 4usize),
        pain_position(8.0, 18.5, 5usize),
    ];

    clear_staging_areas(scene, &paths);
    for (base, style) in paths {
        add_block(
            scene,
            base + Vec3::new(0.0, -0.09, 0.0),
            Vec3::new(1.72, 0.18, 1.52),
            CRATER_EARTH,
        );
        add_path(scene, base, style);
    }
}

fn pain_position(x: f32, z: f32, style: usize) -> (Vec3, usize) {
    (Vec3::new(x, crater_surface_y(x, z) + 0.02, z), style)
}

fn crater_surface_y(x: f32, z: f32) -> f32 {
    let radius = (x * x + (z + 2.2) * (z + 2.2)).sqrt();
    if radius <= 16.0 {
        -7.92
    } else if radius <= 21.0 {
        -7.92 + (radius - 16.0) / 5.0 * 1.87
    } else if radius <= 26.0 {
        -6.05 + (radius - 21.0) / 5.0 * 2.05
    } else if radius <= 31.0 {
        -4.00 + (radius - 26.0) / 5.0 * 2.00
    } else if radius <= 35.0 {
        -2.00 + (radius - 31.0) / 4.0 * 1.45
    } else {
        -0.55
    }
}

fn clear_staging_areas(scene: &mut Scene, paths: &[(Vec3, usize); 6]) {
    scene.cubes.retain(|cube| {
        let size = cube.max - cube.min;
        if size.x > 12.0 || size.z > 12.0 {
            return true;
        }
        let center = (cube.min + cube.max) * 0.5;
        !paths.iter().any(|(base, _)| {
            let dx = center.x - base.x;
            let dz = center.z - base.z;
            dx * dx + dz * dz < 1.85 * 1.85 && cube.max.y > base.y - 0.12
        })
    });
}

fn add_path(scene: &mut Scene, base: Vec3, style: usize) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let first_triangle = scene.triangles.len();
    let height_scale = match style {
        1 => 0.91,
        4 => 1.05,
        _ => 1.0,
    };

    // Separate feet and a layered cloak preserve the human silhouette at a distance.
    for side in [-1.0, 1.0] {
        add_block(
            scene,
            base + Vec3::new(side * 0.22, 0.10, 0.08),
            Vec3::new(0.30, 0.20, 0.48),
            INK,
        );
    }

    add_cloak_mesh(scene, base, height_scale, ROBE);

    for z in [-0.30, 0.30] {
        add_block(
            scene,
            base + Vec3::new(0.0, 1.12 * height_scale, z),
            Vec3::new(0.055, 1.62 * height_scale, 0.035),
            CLOUD_RED,
        );
    }

    for side in [-1.0, 1.0] {
        add_segment(
            scene,
            base + Vec3::new(side * 0.52, 1.68 * height_scale, 0.0),
            base + Vec3::new(side * 0.68, 0.68 * height_scale, 0.05),
            0.20,
            ROBE,
        );
        add_block(
            scene,
            base + Vec3::new(side * 0.69, 0.59 * height_scale, 0.07),
            Vec3::new(0.22, 0.32, 0.34),
            ROBE,
        );
    }

    add_akatsuki_cloud(
        scene,
        base + Vec3::new(0.0, 0.92 * height_scale, 0.315),
        1.0,
    );
    add_akatsuki_cloud(
        scene,
        base + Vec3::new(0.0, 0.92 * height_scale, -0.315),
        -1.0,
    );

    // Collar, neck and head use progressively smaller voxels than the environment.
    add_block(
        scene,
        base + Vec3::new(0.0, 1.94 * height_scale, 0.02),
        Vec3::new(1.02, 0.56, 0.64),
        ROBE,
    );
    for z in [-0.335, 0.335] {
        add_block(
            scene,
            base + Vec3::new(0.0, 1.93 * height_scale, z),
            Vec3::new(0.82, 0.055, 0.045),
            CLOUD_RED,
        );
    }
    add_ellipsoid(
        scene,
        base + Vec3::new(0.0, 2.14 * height_scale, 0.11),
        Vec3::new(0.18, 0.18, 0.16),
        0.06,
        PAIN_SKIN,
    );
    let head_center = base + Vec3::new(0.0, 2.48 * height_scale, 0.04);
    let head_radii = match style {
        0 => Vec3::new(0.40, 0.51 * height_scale, 0.39),
        1 => Vec3::new(0.38, 0.49 * height_scale, 0.36),
        2 => Vec3::new(0.44, 0.52 * height_scale, 0.38),
        3 => Vec3::new(0.42, 0.50 * height_scale, 0.37),
        4 => Vec3::new(0.36, 0.54 * height_scale, 0.35),
        _ => Vec3::new(0.45, 0.48 * height_scale, 0.39),
    };
    add_ellipsoid(scene, head_center, head_radii, 0.08, PAIN_SKIN);
    add_ellipsoid(
        scene,
        head_center + Vec3::new(0.0, -0.27 * height_scale, 0.10),
        Vec3::new(
            head_radii.x * 0.72,
            0.24 * height_scale,
            head_radii.z * 0.82,
        ),
        0.06,
        PAIN_SKIN,
    );

    add_face(scene, head_center, style, height_scale);
    add_hair(scene, head_center, style, height_scale);

    let presentation_scale = if style == 0 { 1.28 } else { 1.18 };
    scale_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        Vec3::new(
            presentation_scale,
            presentation_scale * 1.04,
            presentation_scale,
        ),
    );

    let toward_toads = Vec3::new(0.0, base.y, -1.4) - base;
    rotate_added_geometry(
        scene,
        first_cube,
        first_ellipsoid,
        first_capsule,
        first_triangle,
        base,
        toward_toads.x.atan2(toward_toads.z),
    );
}

fn scale_added_geometry(
    scene: &mut Scene,
    first_cube: usize,
    first_ellipsoid: usize,
    first_capsule: usize,
    first_triangle: usize,
    origin: Vec3,
    scale: Vec3,
) {
    for cube in &mut scene.cubes[first_cube..] {
        cube.min = origin + (cube.min - origin).hadamard(scale);
        cube.max = origin + (cube.max - origin).hadamard(scale);
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        ellipsoid.center = origin + (ellipsoid.center - origin).hadamard(scale);
        ellipsoid.radii = ellipsoid.radii.hadamard(scale);
    }
    let radius_scale = (scale.x + scale.y + scale.z) / 3.0;
    for capsule in &mut scene.capsules[first_capsule..] {
        capsule.start = origin + (capsule.start - origin).hadamard(scale);
        capsule.end = origin + (capsule.end - origin).hadamard(scale);
        capsule.radius *= radius_scale;
    }
    for triangle in &mut scene.triangles[first_triangle..] {
        for vertex in &mut triangle.vertices {
            *vertex = origin + (*vertex - origin).hadamard(scale);
        }
        for normal in &mut triangle.normals {
            *normal =
                Vec3::new(normal.x / scale.x, normal.y / scale.y, normal.z / scale.z).normalized();
        }
    }
}

fn add_face(scene: &mut Scene, head: Vec3, style: usize, scale: f32) {
    let (eye_spacing, eye_height, mouth_width, brow_slant) = match style {
        0 => (0.165, 0.055, 0.24, 0.035),
        1 => (0.155, 0.045, 0.21, -0.020),
        2 => (0.175, 0.065, 0.25, 0.015),
        3 => (0.168, 0.040, 0.22, 0.050),
        4 => (0.145, 0.070, 0.19, -0.015),
        _ => (0.180, 0.035, 0.26, 0.025),
    };
    let front = head.z + 0.34;
    for side in [-1.0, 1.0] {
        add_ellipsoid(
            scene,
            head + Vec3::new(side * 0.43, -0.02 * scale, 0.0),
            Vec3::new(0.10, 0.18, 0.11),
            0.055,
            PAIN_SKIN,
        );
    }

    // Protector curvo, placa metalica y remaches apoyados sobre la frente.
    add_ellipsoid(
        scene,
        Vec3::new(head.x, head.y + 0.27 * scale, front - 0.015),
        Vec3::new(0.37, 0.115, 0.075),
        0.045,
        INK,
    );
    add_ellipsoid(
        scene,
        Vec3::new(head.x, head.y + 0.27 * scale, front + 0.045),
        Vec3::new(0.25, 0.085, 0.035),
        0.035,
        METAL,
    );
    for side in [-1.0, 1.0] {
        add_ellipsoid(
            scene,
            Vec3::new(head.x + side * 0.19, head.y + 0.27 * scale, front + 0.08),
            Vec3::new(0.018, 0.018, 0.012),
            0.02,
            INK,
        );
    }

    // Parpados, Rinnegan y cejas tienen profundidad independiente.
    for side in [-1.0, 1.0] {
        let eye_x = head.x + side * eye_spacing;
        add_ellipsoid(
            scene,
            Vec3::new(eye_x, head.y + eye_height * scale, front + 0.055),
            Vec3::new(0.105, 0.060, 0.035),
            0.025,
            RINNEGAN,
        );
        add_ellipsoid(
            scene,
            Vec3::new(eye_x, head.y + eye_height * scale, front + 0.087),
            Vec3::new(0.021, 0.029, 0.014),
            0.018,
            INK,
        );
        add_segment(
            scene,
            Vec3::new(
                eye_x - side * 0.10,
                head.y + (eye_height + 0.105 - brow_slant * side) * scale,
                front + 0.045,
            ),
            Vec3::new(
                eye_x + side * 0.11,
                head.y + (eye_height + 0.12 + brow_slant * side) * scale,
                front + 0.035,
            ),
            0.030,
            PAIN_HAIR,
        );
    }

    add_segment(
        scene,
        Vec3::new(head.x, head.y + 0.04 * scale, front + 0.035),
        Vec3::new(head.x, head.y - 0.11 * scale, front + 0.105),
        0.055,
        PAIN_SKIN,
    );
    add_ellipsoid(
        scene,
        Vec3::new(head.x, head.y - 0.125 * scale, front + 0.10),
        Vec3::new(0.070, 0.045, 0.040),
        0.025,
        PAIN_SKIN,
    );
    add_segment(
        scene,
        Vec3::new(
            head.x - mouth_width * 0.5,
            head.y - 0.255 * scale,
            front + 0.055,
        ),
        Vec3::new(
            head.x + mouth_width * 0.5,
            head.y - 0.255 * scale,
            front + 0.055,
        ),
        0.026,
        INK,
    );
    add_ellipsoid(
        scene,
        Vec3::new(head.x, head.y - 0.34 * scale, front - 0.005),
        Vec3::new(0.16, 0.11, 0.10),
        0.035,
        PAIN_SKIN,
    );

    // Piercings vary per body so the six Paths are not simple duplicates.
    match style {
        0 | 2 => {
            for offset in [-0.13, 0.0, 0.13] {
                add_ellipsoid(
                    scene,
                    Vec3::new(head.x, head.y - 0.08 * scale + offset, front + 0.085),
                    Vec3::new(0.022, 0.022, 0.018),
                    0.018,
                    METAL,
                );
            }
        }
        1 | 5 => {
            for side in [-1.0, 1.0] {
                add_segment(
                    scene,
                    Vec3::new(head.x + side * 0.285, head.y - 0.12 * scale, front + 0.025),
                    Vec3::new(head.x + side * 0.285, head.y + 0.02 * scale, front + 0.035),
                    0.035,
                    METAL,
                );
            }
        }
        _ => {
            add_segment(
                scene,
                Vec3::new(head.x, head.y - 0.08 * scale, front + 0.105),
                Vec3::new(head.x, head.y + 0.10 * scale, front + 0.085),
                0.034,
                METAL,
            );
        }
    }
}

fn add_hair(scene: &mut Scene, head: Vec3, style: usize, scale: f32) {
    match style {
        1 => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.38 * scale, -0.04),
                Vec3::new(0.42, 0.22, 0.34),
                0.09,
                PAIN_HAIR,
            );
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.66 * scale, -0.05),
                Vec3::new(0.23, 0.25, 0.23),
                0.075,
                PAIN_HAIR,
            );
        }
        4 => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.29 * scale, -0.12),
                Vec3::new(0.39, 0.31 * scale, 0.31),
                0.07,
                PAIN_HAIR,
            );
            for strand in -3i32..=3 {
                let x = strand as f32 * 0.11;
                add_segment(
                    scene,
                    head + Vec3::new(x, 0.30 * scale, -0.16),
                    head + Vec3::new(x * 1.20, -0.86 * scale, -0.10 + strand.abs() as f32 * 0.008),
                    0.105,
                    PAIN_HAIR,
                );
            }
        }
        3 => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.34 * scale, -0.10),
                Vec3::new(0.43, 0.23, 0.35),
                0.09,
                PAIN_HAIR,
            );
            for side in [-1.0, 1.0] {
                add_segment(
                    scene,
                    head + Vec3::new(side * 0.22, 0.42 * scale, -0.02),
                    head + Vec3::new(side * 0.32, 0.72 * scale, -0.05),
                    0.10,
                    INK,
                );
            }
        }
        _ => {
            add_ellipsoid(
                scene,
                head + Vec3::new(0.0, 0.34 * scale, -0.08),
                Vec3::new(0.44, 0.25, 0.36),
                0.09,
                PAIN_HAIR,
            );
            for index in -3i32..=3 {
                let x = index as f32 * 0.12;
                let tip_y = 0.67 + (3 - index.abs()) as f32 * 0.035;
                add_segment(
                    scene,
                    head + Vec3::new(x * 0.72, 0.41 * scale, -0.03),
                    head + Vec3::new(x, tip_y * scale, -0.05),
                    0.105,
                    PAIN_HAIR,
                );
            }
        }
    }
}

fn add_akatsuki_cloud(scene: &mut Scene, center: Vec3, facing: f32) {
    for offset in [
        Vec3::new(-0.23, 0.0, 0.0),
        Vec3::new(0.02, 0.10, 0.0),
        Vec3::new(0.25, 0.0, 0.0),
        Vec3::new(0.10, -0.13, 0.0),
    ] {
        add_ellipsoid(
            scene,
            center + offset,
            Vec3::new(0.22, 0.18, 0.055),
            0.08,
            CLOUD_WHITE,
        );
    }
    for offset in [
        Vec3::new(-0.21, 0.0, 0.065 * facing),
        Vec3::new(0.02, 0.09, 0.065 * facing),
        Vec3::new(0.23, 0.0, 0.065 * facing),
        Vec3::new(0.10, -0.12, 0.065 * facing),
    ] {
        add_ellipsoid(
            scene,
            center + offset,
            Vec3::new(0.16, 0.12, 0.04),
            0.07,
            CLOUD_RED,
        );
    }
}

fn rotate_added_geometry(
    scene: &mut Scene,
    first_cube: usize,
    first_ellipsoid: usize,
    first_capsule: usize,
    first_triangle: usize,
    origin: Vec3,
    yaw: f32,
) {
    let cosine = yaw.cos();
    let sine = yaw.sin();
    for cube in &mut scene.cubes[first_cube..] {
        let center = (cube.min + cube.max) * 0.5;
        let half_size = (cube.max - cube.min) * 0.5;
        let local = center - origin;
        let rotated_center = origin
            + Vec3::new(
                local.x * cosine + local.z * sine,
                local.y,
                -local.x * sine + local.z * cosine,
            );
        cube.min = rotated_center - half_size;
        cube.max = rotated_center + half_size;
        if let Some(normal) = &mut cube.smooth_normal {
            let old_x = normal.x;
            let old_z = normal.z;
            normal.x = old_x * cosine + old_z * sine;
            normal.z = -old_x * sine + old_z * cosine;
        }
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        let local = ellipsoid.center - origin;
        ellipsoid.center = origin
            + Vec3::new(
                local.x * cosine + local.z * sine,
                local.y,
                -local.x * sine + local.z * cosine,
            );
    }
    for capsule in &mut scene.capsules[first_capsule..] {
        for point in [&mut capsule.start, &mut capsule.end] {
            let local = *point - origin;
            *point = origin
                + Vec3::new(
                    local.x * cosine + local.z * sine,
                    local.y,
                    -local.x * sine + local.z * cosine,
                );
        }
    }
    for triangle in &mut scene.triangles[first_triangle..] {
        for vertex in &mut triangle.vertices {
            let local = *vertex - origin;
            *vertex = origin
                + Vec3::new(
                    local.x * cosine + local.z * sine,
                    local.y,
                    -local.x * sine + local.z * cosine,
                );
        }
        for normal in &mut triangle.normals {
            let old_x = normal.x;
            let old_z = normal.z;
            normal.x = old_x * cosine + old_z * sine;
            normal.z = -old_x * sine + old_z * cosine;
        }
    }
}

fn add_cloak_mesh(scene: &mut Scene, base: Vec3, height_scale: f32, material: usize) {
    const SIDES: usize = 18;
    let rings = [
        (0.26, 0.72, 0.37),
        (0.78, 0.68, 0.36),
        (1.34, 0.60, 0.34),
        (1.84, 0.49, 0.32),
    ];
    let mut vertices = Vec::with_capacity(rings.len() * SIDES);
    let mut normals = Vec::with_capacity(rings.len() * SIDES);
    for (y, radius_x, radius_z) in rings {
        for side in 0..SIDES {
            let angle = std::f32::consts::PI * 2.0 * side as f32 / SIDES as f32;
            vertices.push(
                base + Vec3::new(
                    angle.cos() * radius_x,
                    y * height_scale,
                    angle.sin() * radius_z,
                ),
            );
            normals
                .push(Vec3::new(angle.cos() / radius_x, 0.08, angle.sin() / radius_z).normalized());
        }
    }
    for ring in 0..rings.len() - 1 {
        for side in 0..SIDES {
            let next = (side + 1) % SIDES;
            let a = ring * SIDES + side;
            let b = ring * SIDES + next;
            let c = (ring + 1) * SIDES + side;
            let d = (ring + 1) * SIDES + next;
            scene.triangles.push(Triangle {
                vertices: [vertices[a], vertices[c], vertices[b]],
                normals: [normals[a], normals[c], normals[b]],
                material,
            });
            scene.triangles.push(Triangle {
                vertices: [vertices[b], vertices[c], vertices[d]],
                normals: [normals[b], normals[c], normals[d]],
                material,
            });
        }
    }
}

fn add_ellipsoid(scene: &mut Scene, center: Vec3, radii: Vec3, cell: f32, material: usize) {
    let _detail_hint = cell;
    scene.ellipsoids.push(Ellipsoid {
        center,
        radii,
        material,
    });
}

fn add_segment(scene: &mut Scene, start: Vec3, end: Vec3, thickness: f32, material: usize) {
    scene.capsules.push(Capsule {
        start,
        end,
        radius: thickness * 0.5,
        material,
    });
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
        smooth_normal: None,
    });
}
