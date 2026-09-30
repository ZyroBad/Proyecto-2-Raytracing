use crate::material::noise;
use crate::math::Vec3;
use crate::scene::{Capsule, Cube, Ellipsoid, Scene};
use std::f32::consts::PI;

pub fn add_destroyed_konoha(scene: &mut Scene) {
    add_konoha_basin(scene);
    add_distant_forest(scene);
    add_mountain_horizon(scene);

    add_block(
        scene,
        Vec3::new(0.0, -8.30, -2.0),
        Vec3::new(76.0, 0.52, 76.0),
        17,
    );
    for x in [-37.65, 37.65] {
        add_block(
            scene,
            Vec3::new(x, -4.28, -2.0),
            Vec3::new(0.72, 7.55, 76.0),
            17,
        );
    }
    add_block(
        scene,
        Vec3::new(0.0, -4.28, -39.65),
        Vec3::new(76.0, 7.55, 0.72),
        17,
    );
    add_block(
        scene,
        Vec3::new(0.0, -6.82, 35.65),
        Vec3::new(76.0, 2.48, 0.72),
        17,
    );
    add_block(
        scene,
        Vec3::new(0.0, -8.12, -7.2),
        Vec3::new(27.0, 0.32, 13.5),
        17,
    );

    add_crater_ring(scene, 16.0, -7.95, 2.05, 1.05, 72);
    add_crater_ring(scene, 21.0, -6.05, 2.20, 1.15, 88);
    add_crater_ring(scene, 26.0, -4.00, 2.20, 1.25, 104);
    add_crater_ring(scene, 31.0, -2.00, 1.80, 1.35, 120);
    add_crater_ring(scene, 35.0, -0.65, 1.15, 1.45, 136);
    add_crater_wall(scene);
    add_hokage_mountain(scene);
    add_debris_field(scene);
    add_destroyed_districts(scene);

    for (x, z, width, height) in [
        (-10.4, -4.8, 2.7, 4.6),
        (-7.8, -8.6, 3.4, 3.0),
        (8.1, -8.4, 3.2, 3.4),
        (10.6, -4.5, 2.8, 4.9),
        (-11.3, 2.8, 2.5, 2.6),
        (11.2, 2.5, 2.6, 2.8),
    ] {
        add_ruined_building(
            scene,
            Vec3::new(x, crater_surface_y(x, z), z),
            width,
            height,
        );
    }

    for (x, z, sx, sy, sz, material) in [
        (-8.5, -2.2, 1.8, 0.55, 0.9, 18),
        (-6.3, -4.0, 1.1, 0.42, 1.4, 20),
        (-4.5, -6.0, 1.5, 0.50, 0.75, 18),
        (-2.4, -4.6, 0.8, 0.36, 1.5, 19),
        (2.8, -5.2, 1.2, 0.48, 0.8, 18),
        (4.8, -7.1, 1.6, 0.45, 0.7, 20),
        (6.7, -3.8, 0.9, 0.38, 1.5, 19),
        (8.9, -2.4, 1.7, 0.52, 0.85, 18),
    ] {
        add_block(
            scene,
            Vec3::new(x, crater_surface_y(x, z) + sy * 0.5, z),
            Vec3::new(sx, sy, sz),
            material,
        );
    }

    add_crack_path(
        scene,
        Vec3::new(-0.8, -7.93, -1.8),
        Vec3::new(-27.0, -0.55, -18.5),
    );
    add_crack_path(
        scene,
        Vec3::new(1.2, -7.93, -2.2),
        Vec3::new(28.0, -0.55, -16.4),
    );
    add_crack_path(
        scene,
        Vec3::new(0.2, -7.93, -2.0),
        Vec3::new(-3.3, -0.55, -34.0),
    );
    add_crack_path(
        scene,
        Vec3::new(-1.8, -7.93, 0.4),
        Vec3::new(-27.4, -0.55, 16.8),
    );
    add_crack_path(
        scene,
        Vec3::new(2.0, -7.93, 0.2),
        Vec3::new(27.0, -0.55, 17.2),
    );

    add_dust_plume(scene, Vec3::new(-18.4, -3.0, -16.4), 1.15);
    add_dust_plume(scene, Vec3::new(19.2, -2.8, -15.0), 1.0);
    add_dust_plume(scene, Vec3::new(8.8, -5.4, -18.5), 0.72);
}

fn add_konoha_basin(scene: &mut Scene) {
    // Cuatro losas rodean una abertura real; el terreno no atraviesa el socavon.
    for (center, size) in [
        (Vec3::new(-56.5, -1.18, -2.0), Vec3::new(37.0, 1.42, 150.0)),
        (Vec3::new(56.5, -1.18, -2.0), Vec3::new(37.0, 1.42, 150.0)),
        (Vec3::new(0.0, -1.18, 55.5), Vec3::new(76.0, 1.42, 39.0)),
        (Vec3::new(0.0, -1.18, -58.5), Vec3::new(76.0, 1.42, 33.0)),
    ] {
        add_block(scene, center, size, 17);
    }

    for index in 0..36 {
        let angle = index as f32 * 2.399_963;
        let variation = noise(Vec3::new(index as f32 * 0.47, 2.7, 6.2));
        let radius = 13.0 + variation * 22.0;
        let width = 2.0 + variation * 5.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.0;
        add_block(
            scene,
            Vec3::new(x, crater_surface_y(x, z) + 0.05, z),
            Vec3::new(width, 0.10, width * (0.52 + variation * 0.38)),
            if index % 4 == 0 { 18 } else { 0 },
        );
    }

    // Colinas bajas y restos urbanos cierran el horizonte desde cualquier angulo.
    for index in 0..72 {
        let angle = 2.0 * PI * index as f32 / 72.0;
        let variation = noise(Vec3::new(index as f32 * 0.71, 9.0, 3.5));
        let radius = 37.0 + variation * 3.0;
        let height = 1.4 + variation * 3.8;
        let width = 2.8 + variation * 2.2;
        add_block(
            scene,
            Vec3::new(
                angle.cos() * radius,
                height * 0.5 - 0.35,
                angle.sin() * radius - 2.0,
            ),
            Vec3::new(width, height, 3.4 + variation * 1.8),
            if index % 5 == 0 { 18 } else { 0 },
        );
    }

    for index in 0..18 {
        let angle = 2.0 * PI * index as f32 / 18.0 + 0.17;
        let radius = 27.0 + noise(Vec3::new(index as f32, 4.2, 8.7)) * 5.0;
        let height = 1.5 + (index % 5) as f32 * 0.62;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.0;
        add_ruined_building(
            scene,
            Vec3::new(x, crater_surface_y(x, z), z),
            1.8 + (index % 3) as f32 * 0.48,
            height,
        );
    }

    for index in 0..140 {
        let angle = index as f32 * 2.399_963;
        let variation = noise(Vec3::new(index as f32 * 0.83, 6.1, 2.4));
        let radius = 16.0 + variation * 19.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.0;
        let width = 0.24 + noise(Vec3::new(x, 1.4, z)) * 1.15;
        let height = 0.14 + noise(Vec3::new(z, 3.7, x)) * 0.52;
        let depth = 0.30 + noise(Vec3::new(x * 0.4, z * 0.7, 5.0)) * 1.30;
        add_block(
            scene,
            Vec3::new(x, crater_surface_y(x, z) + height * 0.5, z),
            Vec3::new(width, height, depth),
            match index % 6 {
                0 => 13,
                1 | 2 => 18,
                3 => 19,
                _ => 17,
            },
        );
    }

    for index in 0..14 {
        let angle = 2.0 * PI * index as f32 / 14.0 + 0.31;
        let radius = 24.0 + (index % 4) as f32 * 1.8;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.0;
        let trunk_height = 1.4 + (index % 3) as f32 * 0.55;
        let ground_y = crater_surface_y(x, z);
        add_block(
            scene,
            Vec3::new(x, ground_y + trunk_height * 0.5, z),
            Vec3::new(0.24, trunk_height, 0.28),
            13,
        );
        add_block(
            scene,
            Vec3::new(x + 0.32, ground_y + trunk_height * 0.72, z),
            Vec3::new(0.78, 0.18, 0.20),
            13,
        );
    }
}

fn crater_surface_y(x: f32, z: f32) -> f32 {
    let dx = x;
    let dz = z + 2.2;
    let radius = (dx * dx + dz * dz).sqrt();
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

fn add_distant_forest(scene: &mut Scene) {
    let leaves = 24;
    for index in 0..190 {
        let angle = index as f32 * 2.399_963;
        let radial_noise = noise(Vec3::new(index as f32 * 0.63, 12.0, 4.8));
        let radius = 43.0 + radial_noise * 16.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.0;
        let height = 2.5 + noise(Vec3::new(x * 0.18, z * 0.15, 7.0)) * 3.4;
        let crown = 1.35 + radial_noise * 1.20;

        add_block(
            scene,
            Vec3::new(x, -0.42 + height * 0.5, z),
            Vec3::new(
                0.32 + radial_noise * 0.18,
                height,
                0.36 + radial_noise * 0.16,
            ),
            13,
        );
        for offset in [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(-0.62, -0.20, 0.12),
            Vec3::new(0.58, -0.12, -0.18),
            Vec3::new(0.08, 0.56, 0.06),
        ] {
            add_block(
                scene,
                Vec3::new(x, height - 0.30, z) + offset * crown,
                Vec3::new(crown * 1.15, crown * 0.92, crown * 1.10),
                leaves,
            );
        }
    }
}

fn add_mountain_horizon(scene: &mut Scene) {
    let mountain = 25;
    for index in 0..52 {
        let angle = 2.0 * PI * index as f32 / 52.0;
        let variation = noise(Vec3::new(index as f32 * 0.54, 15.0, 1.7));
        let radius = 67.0 + variation * 7.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.0;
        let peak_height = 8.0 + variation * 11.0;
        let base_width = 8.0 + variation * 5.0;

        let layer_count = 8;
        let layer_height = peak_height / layer_count as f32;
        for layer in 0..layer_count {
            let progress = layer as f32 / layer_count as f32;
            let taper = (1.0 - progress).powf(0.78).max(0.16);
            let drift = (index as f32 * 1.7 + layer as f32 * 0.8).sin() * base_width * 0.045;
            add_block(
                scene,
                Vec3::new(
                    x + drift,
                    -0.38 + layer_height * (layer as f32 + 0.5),
                    z + drift * 0.35,
                ),
                Vec3::new(
                    base_width * taper,
                    layer_height + 0.12,
                    base_width * (0.82 * taper).max(1.1),
                ),
                mountain,
            );
        }

        if index % 3 == 0 {
            let side = if index % 2 == 0 { -1.0 } else { 1.0 };
            add_block(
                scene,
                Vec3::new(x + side * base_width * 0.56, 1.55, z + 0.4),
                Vec3::new(base_width * 0.52, 3.8, base_width * 0.48),
                mountain,
            );
        }
    }
}

fn add_destroyed_districts(scene: &mut Scene) {
    // Siluetas urbanas en tres planos para que Konoha continue detras del crater.
    for (x, z, width, height) in [
        (-15.8, -5.8, 3.2, 5.8),
        (-13.8, -10.2, 2.8, 4.2),
        (-17.5, -13.5, 3.8, 6.4),
        (-11.8, -15.0, 2.5, 3.5),
        (15.6, -5.5, 3.1, 5.4),
        (13.5, -10.4, 2.7, 4.0),
        (17.2, -13.2, 3.6, 6.1),
        (11.7, -15.2, 2.6, 3.7),
    ] {
        add_ruined_building(
            scene,
            Vec3::new(x, crater_surface_y(x, z), z),
            width,
            height,
        );
    }

    for (x, z, radius, height, segments) in [
        (-14.8, -9.0, 1.65, 6.2, 14),
        (14.7, -8.7, 1.55, 5.5, 12),
        (-9.8, -13.8, 1.15, 3.8, 10),
        (9.7, -14.0, 1.25, 4.1, 10),
    ] {
        add_ruined_tower(
            scene,
            Vec3::new(x, crater_surface_y(x, z), z),
            radius,
            height,
            segments,
        );
    }

    add_broken_gate(
        scene,
        Vec3::new(-6.7, crater_surface_y(-6.7, -12.4), -12.4),
        0.85,
    );
    add_broken_gate(
        scene,
        Vec3::new(6.8, crater_surface_y(6.8, -12.6), -12.6),
        0.78,
    );

    // Fragmentos de las avenidas radiales de Konoha.
    for side in [-1.0, 1.0] {
        for step in 0..7 {
            let z = -5.4 - step as f32 * 1.55;
            let drift = (step as f32 * 1.7).sin() * 0.32;
            let x = side * (11.8 + drift);
            add_block(
                scene,
                Vec3::new(x, crater_surface_y(x, z) + 0.16, z),
                Vec3::new(1.85, 0.16, 1.05),
                if step % 3 == 0 { 18 } else { 17 },
            );
        }
    }
}

fn add_ruined_tower(scene: &mut Scene, base: Vec3, radius: f32, height: f32, segments: usize) {
    let levels = (height / 0.72).ceil() as usize;
    for level in 0..levels {
        let y = 0.40 + level as f32 * 0.72;
        let broken_side = (level * 3 + segments / 4) % segments;
        for segment in 0..segments {
            if level > levels / 2
                && (segment == broken_side || segment == (broken_side + 1) % segments)
            {
                continue;
            }
            let angle = 2.0 * PI * segment as f32 / segments as f32;
            let center = base + Vec3::new(angle.cos() * radius, y, angle.sin() * radius);
            add_block(
                scene,
                center,
                Vec3::new(0.58, 0.66, 0.58),
                if (segment + level) % 7 == 0 { 0 } else { 18 },
            );
        }
    }

    for segment in 0..segments {
        if segment % 4 == 1 {
            continue;
        }
        let angle = 2.0 * PI * segment as f32 / segments as f32;
        add_block(
            scene,
            base + Vec3::new(
                angle.cos() * (radius + 0.18),
                height + 0.18,
                angle.sin() * (radius + 0.18),
            ),
            Vec3::new(0.72, 0.28, 0.72),
            19,
        );
    }
}

fn add_broken_gate(scene: &mut Scene, base: Vec3, scale: f32) {
    for side in [-1.0, 1.0] {
        let pillar_height = if side < 0.0 { 3.8 } else { 2.9 };
        add_block(
            scene,
            base + Vec3::new(side * 1.45 * scale, pillar_height * scale * 0.5, 0.0),
            Vec3::new(0.55, pillar_height * scale, 0.65),
            18,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-0.18, 2.75 * scale, 0.0),
        Vec3::new(2.35 * scale, 0.36, 0.72),
        19,
    );
    add_block(
        scene,
        base + Vec3::new(0.65, 1.35 * scale, 0.20),
        Vec3::new(0.38, 2.2 * scale, 0.34),
        13,
    );
}

fn add_crater_ring(
    scene: &mut Scene,
    radius: f32,
    floor_y: f32,
    wall_height: f32,
    block_size: f32,
    segments: usize,
) {
    for index in 0..=segments {
        let angle = 2.0 * PI * index as f32 / segments as f32;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.2;
        let uneven = noise(Vec3::new(x, floor_y, z));
        let backness = (-angle.sin() + 1.0) * 0.5;
        let front_cutaway = 1.0 - angle.sin().max(0.0) * 0.58;
        let block_height = wall_height * (0.82 + backness * 0.10 + uneven * 0.10) * front_cutaway;
        add_block(
            scene,
            Vec3::new(x, floor_y + block_height * 0.5, z),
            Vec3::new(block_size, block_height, block_size),
            if index % 5 == 0 { 18 } else { 17 },
        );
    }
}

fn add_crater_wall(scene: &mut Scene) {
    for index in -10i32..=10 {
        let x = index as f32 * 3.18;
        let edge = (index.abs() as f32 / 10.0).powf(1.5);
        let z = -34.0 + edge * 4.8;
        let variation = noise(Vec3::new(x, 2.0, z));
        let rim_height = 1.7 + edge * 1.9 + variation * 0.65;
        let wall_height = rim_height + 8.05;
        add_block(
            scene,
            Vec3::new(x, -8.05 + wall_height * 0.5, z),
            Vec3::new(3.38, wall_height, 2.80),
            if index % 6 == 0 { 18 } else { 17 },
        );
        if index % 4 == 0 {
            add_block(
                scene,
                Vec3::new(x + 0.28, rim_height + 0.22, z - 0.15),
                Vec3::new(1.42, 0.42, 1.75),
                18,
            );
        }
    }
}

fn add_hokage_mountain(scene: &mut Scene) {
    // Macizo distante integrado con el bosque y la cordillera exterior.
    for index in -11i32..=11 {
        let x = index as f32 * 2.90;
        let center_falloff = (1.0 - (x.abs() / 35.0).powf(1.65)).max(0.0);
        let variation = noise(Vec3::new(x * 0.19, 5.0, -44.0));
        let height = 8.0 + center_falloff * 7.0 + variation * 2.2;
        let depth = 8.5 + noise(Vec3::new(x * 0.31, 8.0, -9.0)) * 3.5;
        let z = -51.0 - noise(Vec3::new(x * 0.15, 3.0, 2.0)) * 1.2;
        add_block(
            scene,
            Vec3::new(x, height * 0.5 - 0.45, z),
            Vec3::new(3.55, height, depth),
            if index % 4 == 0 { 25 } else { 0 },
        );

        if index % 2 == 0 {
            let ledge_y = 3.0 + variation * 5.0;
            add_block(
                scene,
                Vec3::new(x + 0.42, ledge_y, z + depth * 0.49),
                Vec3::new(4.10, 0.58, 1.25),
                25,
            );
        }
    }

    for index in -9i32..=9 {
        let x = index as f32 * 3.45;
        let variation = noise(Vec3::new(x * 0.25, 11.0, -50.0));
        let peak = 2.0 + variation * 5.0;
        add_block(
            scene,
            Vec3::new(x, 13.0 + peak * 0.5, -55.0),
            Vec3::new(4.4, peak, 5.5),
            if index % 3 == 0 { 25 } else { 0 },
        );
    }

    for (index, x) in [-17.0, -8.5, 0.0, 8.5, 17.0].into_iter().enumerate() {
        add_hokage_face(
            scene,
            Vec3::new(x, 10.0 + (index % 2) as f32 * 0.48, -45.45),
            index,
        );
    }
}

fn add_hokage_face(scene: &mut Scene, center: Vec3, style: usize) {
    let first_cube = scene.cubes.len();
    let first_ellipsoid = scene.ellipsoids.len();
    let first_capsule = scene.capsules.len();
    let stone = 21;
    let shadow = 0;

    let (head_width, jaw_width, eye_y, brow_slant) = match style {
        0 => (1.02, 0.76, 0.24, 0.08),
        1 => (0.94, 0.70, 0.29, 0.15),
        2 => (1.08, 0.84, 0.20, 0.02),
        3 => (0.92, 0.68, 0.31, 0.18),
        _ => (0.90, 0.64, 0.30, 0.11),
    };

    // Volumen craneal, sienes, mejillas y mandibula forman un relieve continuo.
    add_relief_ellipsoid(
        scene,
        center + Vec3::new(0.0, 0.28, 0.0),
        Vec3::new(head_width, 1.02, 0.34),
        stone,
    );
    add_relief_ellipsoid(
        scene,
        center + Vec3::new(0.0, -0.68, 0.08),
        Vec3::new(jaw_width, 0.48, 0.31),
        stone,
    );
    add_relief_ellipsoid(
        scene,
        center + Vec3::new(0.0, -1.02, 0.05),
        Vec3::new(jaw_width * 0.56, 0.24, 0.25),
        stone,
    );
    for side in [-1.0, 1.0] {
        add_relief_ellipsoid(
            scene,
            center + Vec3::new(side * head_width * 0.67, -0.24, 0.24),
            Vec3::new(0.38, 0.43, 0.25),
            stone,
        );
        add_relief_ellipsoid(
            scene,
            center + Vec3::new(side * (head_width + 0.08), 0.02, -0.02),
            Vec3::new(0.17, 0.40, 0.22),
            stone,
        );

        // Cuenca hundida, parpados y ceja tallada con inclinacion individual.
        let eye_x = side * head_width * 0.48;
        add_relief_ellipsoid(
            scene,
            center + Vec3::new(eye_x, eye_y, 0.37),
            Vec3::new(0.31, 0.12, 0.075),
            shadow,
        );
        add_relief_ellipsoid(
            scene,
            center + Vec3::new(eye_x, eye_y, 0.43),
            Vec3::new(0.19, 0.075, 0.045),
            stone,
        );
        add_relief_segment(
            scene,
            center + Vec3::new(side * 0.10, eye_y + 0.22, 0.37),
            center
                + Vec3::new(
                    side * (head_width * 0.88),
                    eye_y + 0.22 + brow_slant * side,
                    0.31,
                ),
            0.095,
            stone,
        );
        add_relief_segment(
            scene,
            center + Vec3::new(side * 0.28, -0.22, 0.39),
            center + Vec3::new(side * 0.58, -0.48, 0.34),
            0.055,
            shadow,
        );
    }

    // Nariz proyectada, aletas nasales, labios y pliegues de expresion.
    add_relief_segment(
        scene,
        center + Vec3::new(0.0, 0.42, 0.37),
        center + Vec3::new(0.0, -0.28, 0.47),
        0.14,
        stone,
    );
    add_relief_ellipsoid(
        scene,
        center + Vec3::new(0.0, -0.34, 0.49),
        Vec3::new(0.30, 0.17, 0.18),
        stone,
    );
    for side in [-1.0, 1.0] {
        add_relief_ellipsoid(
            scene,
            center + Vec3::new(side * 0.13, -0.36, 0.60),
            Vec3::new(0.055, 0.035, 0.025),
            shadow,
        );
    }
    add_relief_segment(
        scene,
        center + Vec3::new(-0.42, -0.69, 0.43),
        center + Vec3::new(0.0, -0.76, 0.48),
        0.052,
        shadow,
    );
    add_relief_segment(
        scene,
        center + Vec3::new(0.0, -0.76, 0.48),
        center + Vec3::new(0.42, -0.69, 0.43),
        0.052,
        shadow,
    );
    add_relief_segment(
        scene,
        center + Vec3::new(-0.29, -0.88, 0.34),
        center + Vec3::new(0.29, -0.88, 0.34),
        0.035,
        shadow,
    );

    match style {
        // Hashirama: protector amplio y mechones largos.
        0 => {
            add_relief_block(
                scene,
                center + Vec3::new(0.0, 0.91, 0.30),
                Vec3::new(1.78, 0.20, 0.18),
                shadow,
            );
            add_relief_block(
                scene,
                center + Vec3::new(0.0, 0.91, 0.42),
                Vec3::new(0.62, 0.22, 0.10),
                stone,
            );
            for side in [-1.0, 1.0] {
                add_relief_segment(
                    scene,
                    center + Vec3::new(side * 0.86, 0.82, 0.02),
                    center + Vec3::new(side * 1.02, -0.92, 0.05),
                    0.18,
                    shadow,
                );
            }
        }
        // Tobirama: casco, placa frontal y guardas laterales.
        1 => {
            add_relief_ellipsoid(
                scene,
                center + Vec3::new(0.0, 1.06, -0.04),
                Vec3::new(0.82, 0.42, 0.30),
                stone,
            );
            add_relief_block(
                scene,
                center + Vec3::new(0.0, 0.88, 0.39),
                Vec3::new(0.82, 0.26, 0.12),
                shadow,
            );
            for side in [-1.0, 1.0] {
                add_relief_segment(
                    scene,
                    center + Vec3::new(side * 0.90, 0.62, 0.02),
                    center + Vec3::new(side * 1.02, -0.54, 0.03),
                    0.15,
                    shadow,
                );
            }
        }
        // Hiruzen: cejas pobladas, bigote y barba escalonada.
        2 => {
            for side in [-1.0, 1.0] {
                add_relief_segment(
                    scene,
                    center + Vec3::new(side * 0.08, -0.55, 0.46),
                    center + Vec3::new(side * 0.48, -0.63, 0.39),
                    0.075,
                    shadow,
                );
            }
            for strand in -2i32..=2 {
                add_relief_segment(
                    scene,
                    center + Vec3::new(strand as f32 * 0.18, -0.84, 0.20),
                    center
                        + Vec3::new(
                            strand as f32 * 0.24,
                            -1.42 - (2 - strand.abs()) as f32 * 0.10,
                            0.04,
                        ),
                    0.11,
                    shadow,
                );
            }
        }
        // Minato: corona de puntas y rostro anguloso.
        3 => {
            for spike in -3i32..=3 {
                let x = spike as f32 * 0.24;
                add_relief_segment(
                    scene,
                    center + Vec3::new(x * 0.72, 1.00, -0.02),
                    center + Vec3::new(x, 1.52 + (3 - spike.abs()) as f32 * 0.10, -0.08),
                    0.13,
                    shadow,
                );
            }
            for side in [-1.0, 1.0] {
                add_relief_segment(
                    scene,
                    center + Vec3::new(side * 0.48, -0.46, 0.38),
                    center + Vec3::new(side * 0.69, -0.74, 0.28),
                    0.045,
                    shadow,
                );
            }
        }
        // Tsunade: cabello largo, raya central y sello frontal.
        _ => {
            for side in [-1.0, 1.0] {
                add_relief_segment(
                    scene,
                    center + Vec3::new(side * 0.56, 1.02, 0.00),
                    center + Vec3::new(side * 0.98, -1.10, 0.02),
                    0.21,
                    shadow,
                );
            }
            add_relief_segment(
                scene,
                center + Vec3::new(0.0, 1.08, 0.22),
                center + Vec3::new(0.0, 0.60, 0.34),
                0.045,
                shadow,
            );
            add_relief_ellipsoid(
                scene,
                center + Vec3::new(0.0, 0.63, 0.48),
                Vec3::new(0.075, 0.13, 0.045),
                shadow,
            );
        }
    }

    const FACE_SCALE: f32 = 1.95;
    for cube in &mut scene.cubes[first_cube..] {
        cube.min = center + (cube.min - center) * FACE_SCALE;
        cube.max = center + (cube.max - center) * FACE_SCALE;
    }
    for ellipsoid in &mut scene.ellipsoids[first_ellipsoid..] {
        ellipsoid.center = center + (ellipsoid.center - center) * FACE_SCALE;
        ellipsoid.radii = ellipsoid.radii * FACE_SCALE;
    }
    for capsule in &mut scene.capsules[first_capsule..] {
        capsule.start = center + (capsule.start - center) * FACE_SCALE;
        capsule.end = center + (capsule.end - center) * FACE_SCALE;
        capsule.radius *= FACE_SCALE;
    }
}

fn add_relief_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    add_block(scene, center, size, material);
}

fn add_relief_ellipsoid(scene: &mut Scene, center: Vec3, radii: Vec3, material: usize) {
    scene.ellipsoids.push(Ellipsoid {
        center,
        radii,
        material,
    });
}

fn add_relief_segment(scene: &mut Scene, start: Vec3, end: Vec3, radius: f32, material: usize) {
    scene.capsules.push(Capsule {
        start,
        end,
        radius,
        material,
    });
}

fn add_ruined_building(scene: &mut Scene, base: Vec3, width: f32, height: f32) {
    let levels = height.ceil() as usize;
    for level in 0..levels {
        let y = 0.5 + level as f32;
        let taper = level as f32 * 0.12;
        if level % 3 != 1 {
            add_block(
                scene,
                base + Vec3::new(-width * 0.43 + taper, y, 0.0),
                Vec3::new(0.42, 0.92, 2.0 - taper * 0.3),
                18,
            );
        }
        if level % 4 != 2 {
            add_block(
                scene,
                base + Vec3::new(width * 0.43 - taper, y, 0.0),
                Vec3::new(0.42, 0.92, 2.0 - taper * 0.3),
                18,
            );
        }
        if level % 2 == 0 {
            add_block(
                scene,
                base + Vec3::new(0.0, y, -0.85),
                Vec3::new((width - taper).max(0.8), 0.38, 0.34),
                20,
            );
        }
    }
    add_block(
        scene,
        base + Vec3::new(width * 0.18, height + 0.18, 0.0),
        Vec3::new(width * 0.72, 0.30, 2.15),
        19,
    );
}

fn add_crack_path(scene: &mut Scene, start: Vec3, end: Vec3) {
    for step in 0..18 {
        let t = step as f32 / 17.0;
        let bend = (step as f32 * 2.1).sin() * 0.35;
        let mut point = start * (1.0 - t) + end * t + Vec3::new(bend, 0.0, 0.0);
        point.y = crater_surface_y(point.x, point.z) + 0.03;
        add_block(scene, point, Vec3::new(0.20, 0.06, 1.15), 0);
    }
}

fn add_debris_field(scene: &mut Scene) {
    for index in 0..52 {
        let angle = index as f32 * 2.399_963;
        let radial_noise = noise(Vec3::new(index as f32 * 0.73, 1.7, 4.2));
        let radius = 12.0 + radial_noise * 20.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.2;
        let width = 0.28 + noise(Vec3::new(x, 2.1, z)) * 0.82;
        let height = 0.18 + noise(Vec3::new(z, 4.3, x)) * 0.48;
        let depth = 0.32 + noise(Vec3::new(x * 0.5, z * 0.8, 8.1)) * 0.90;
        let material = match index % 5 {
            0 => 19,
            1 | 2 => 18,
            _ => 17,
        };
        if z > 4.8 && z < 14.5 && x.abs() < 13.5 {
            continue;
        }
        let floor_y = crater_surface_y(x, z);
        add_block(
            scene,
            Vec3::new(x, floor_y + height * 0.5, z),
            Vec3::new(width, height, depth),
            material,
        );
    }

    for (x, z, height) in [
        (-12.8, -1.8, 3.2),
        (-11.7, 4.5, 2.4),
        (12.7, -2.4, 3.5),
        (11.4, 4.2, 2.2),
    ] {
        let ground_y = crater_surface_y(x, z);
        add_block(
            scene,
            Vec3::new(x, ground_y + height * 0.5, z),
            Vec3::new(0.62, height, 0.72),
            18,
        );
        add_block(
            scene,
            Vec3::new(x + 0.34, ground_y + height + 0.16, z - 0.15),
            Vec3::new(1.25, 0.30, 0.82),
            19,
        );
    }
}

fn add_dust_plume(scene: &mut Scene, base: Vec3, scale: f32) {
    for (offset, size) in [
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.5, 1.2, 1.8)),
        (Vec3::new(-0.5, 1.0, 0.0), Vec3::new(2.0, 1.4, 1.5)),
        (Vec3::new(0.35, 2.0, -0.1), Vec3::new(1.7, 1.6, 1.4)),
        (Vec3::new(-0.2, 3.1, 0.0), Vec3::new(1.25, 1.7, 1.1)),
    ] {
        add_block(scene, base + offset * scale, size * scale, 20);
    }
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    scene.cubes.push(Cube {
        min: center - size * 0.5,
        max: center + size * 0.5,
        material,
        smooth_normal: None,
    });
}
