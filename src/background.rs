use crate::material::noise;
use crate::math::Vec3;
use crate::scene::{Cube, Scene};
use std::f32::consts::PI;

pub fn add_destroyed_konoha(scene: &mut Scene) {
    add_konoha_basin(scene);

    add_block(
        scene,
        Vec3::new(0.0, -0.05, -7.2),
        Vec3::new(27.0, 0.32, 13.5),
        17,
    );

    add_crater_ring(scene, 7.2, 0.06, 0.82, 36);
    add_crater_ring(scene, 8.6, 0.11, 0.90, 40);
    add_crater_ring(scene, 10.1, 0.18, 1.05, 44);
    add_crater_ring(scene, 11.7, 0.25, 1.16, 48);
    add_crater_ring(scene, 13.2, 0.32, 1.30, 52);
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
        add_ruined_building(scene, Vec3::new(x, 0.0, z), width, height);
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
            Vec3::new(x, 0.42, z),
            Vec3::new(sx, sy, sz),
            material,
        );
    }

    add_crack_path(
        scene,
        Vec3::new(-0.8, 0.18, -1.8),
        Vec3::new(-7.0, 0.18, -8.5),
    );
    add_crack_path(
        scene,
        Vec3::new(1.2, 0.18, -2.2),
        Vec3::new(7.8, 0.18, -7.4),
    );
    add_crack_path(
        scene,
        Vec3::new(0.2, 0.18, -2.0),
        Vec3::new(-1.3, 0.18, -10.2),
    );
    add_crack_path(
        scene,
        Vec3::new(-1.8, 0.18, 0.4),
        Vec3::new(-8.4, 0.18, 6.8),
    );
    add_crack_path(scene, Vec3::new(2.0, 0.18, 0.2), Vec3::new(8.0, 0.18, 6.2));

    add_dust_plume(scene, Vec3::new(-9.4, 1.2, -9.4), 1.15);
    add_dust_plume(scene, Vec3::new(9.2, 1.0, -9.0), 1.0);
    add_dust_plume(scene, Vec3::new(4.8, 0.8, -11.5), 0.72);
}

fn add_konoha_basin(scene: &mut Scene) {
    // Terreno continuo bajo toda la orbita para evitar el efecto de isla flotante.
    add_block(
        scene,
        Vec3::new(0.0, -1.18, -2.0),
        Vec3::new(92.0, 1.42, 92.0),
        17,
    );

    for index in 0..36 {
        let angle = index as f32 * 2.399_963;
        let variation = noise(Vec3::new(index as f32 * 0.47, 2.7, 6.2));
        let radius = 13.0 + variation * 22.0;
        let width = 2.0 + variation * 5.0;
        add_block(
            scene,
            Vec3::new(
                angle.cos() * radius,
                -0.43 + variation * 0.035,
                angle.sin() * radius - 2.0,
            ),
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
        add_ruined_building(
            scene,
            Vec3::new(angle.cos() * radius, -0.38, angle.sin() * radius - 2.0),
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
            Vec3::new(x, -0.34 + height * 0.5, z),
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
        add_block(
            scene,
            Vec3::new(x, -0.38 + trunk_height * 0.5, z),
            Vec3::new(0.24, trunk_height, 0.28),
            13,
        );
        add_block(
            scene,
            Vec3::new(x + 0.32, -0.38 + trunk_height * 0.72, z),
            Vec3::new(0.78, 0.18, 0.20),
            13,
        );
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
        add_ruined_building(scene, Vec3::new(x, 0.0, z), width, height);
    }

    add_ruined_tower(scene, Vec3::new(-14.8, 0.0, -9.0), 1.65, 6.2, 14);
    add_ruined_tower(scene, Vec3::new(14.7, 0.0, -8.7), 1.55, 5.5, 12);
    add_ruined_tower(scene, Vec3::new(-9.8, 0.0, -13.8), 1.15, 3.8, 10);
    add_ruined_tower(scene, Vec3::new(9.7, 0.0, -14.0), 1.25, 4.1, 10);

    add_broken_gate(scene, Vec3::new(-6.7, 0.0, -12.4), 0.85);
    add_broken_gate(scene, Vec3::new(6.8, 0.0, -12.6), 0.78);

    // Fragmentos de las avenidas radiales de Konoha.
    for side in [-1.0, 1.0] {
        for step in 0..7 {
            let z = -5.4 - step as f32 * 1.55;
            let drift = (step as f32 * 1.7).sin() * 0.32;
            add_block(
                scene,
                Vec3::new(side * (11.8 + drift), 0.16, z),
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

fn add_crater_ring(scene: &mut Scene, radius: f32, height: f32, block_size: f32, segments: usize) {
    for index in 0..=segments {
        let angle = 2.0 * PI * index as f32 / segments as f32;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius - 2.2;
        let uneven = noise(Vec3::new(x, height, z));
        let backness = (-angle.sin() + 1.0) * 0.5;
        let vertical_scale = 0.35 + backness * 0.55 + uneven * 0.25;
        add_block(
            scene,
            Vec3::new(x, height * (0.35 + backness * 0.65) + uneven * 0.25, z),
            Vec3::new(block_size, block_size * vertical_scale, block_size),
            if index % 5 == 0 { 18 } else { 17 },
        );
    }
}

fn add_crater_wall(scene: &mut Scene) {
    for index in -10i32..=10 {
        let x = index as f32 * 1.28;
        let edge = (index.abs() as f32 / 10.0).powf(1.5);
        let z = -12.2 + edge * 2.6;
        let variation = noise(Vec3::new(x, 2.0, z));
        let height = 1.7 + edge * 1.9 + variation * 0.65;
        add_block(
            scene,
            Vec3::new(x, height * 0.5 - 0.15, z),
            Vec3::new(1.42, height, 2.25),
            if index % 6 == 0 { 18 } else { 17 },
        );
        if index % 4 == 0 {
            add_block(
                scene,
                Vec3::new(x + 0.28, height + 0.22, z - 0.15),
                Vec3::new(0.72, 0.42, 1.35),
                18,
            );
        }
    }
}

fn add_hokage_mountain(scene: &mut Scene) {
    // Columnas solapadas forman una pared erosionada sin una silueta rectangular.
    for index in -12i32..=12 {
        let x = index as f32 * 1.55;
        let center_falloff = (1.0 - (x.abs() / 21.0).powf(1.7)).max(0.0);
        let variation = noise(Vec3::new(x * 0.27, 5.0, -16.0));
        let height = 5.4 + center_falloff * 4.8 + variation * 1.7;
        let depth = 3.8 + noise(Vec3::new(x * 0.41, 8.0, -4.0)) * 2.1;
        let z = -17.0 - noise(Vec3::new(x * 0.19, 3.0, 2.0)) * 0.8;
        add_block(
            scene,
            Vec3::new(x, height * 0.5 - 0.18, z),
            Vec3::new(1.85, height, depth),
            if index % 4 == 0 { 18 } else { 0 },
        );

        if index % 2 == 0 {
            let ledge_y = 3.1 + variation * 2.8;
            add_block(
                scene,
                Vec3::new(x + 0.28, ledge_y, z + depth * 0.48),
                Vec3::new(2.15, 0.42, 0.90),
                18,
            );
        }
    }

    // Crestas traseras y hombros laterales dan profundidad desde la orbita completa.
    for index in -9i32..=9 {
        let x = index as f32 * 2.05;
        let variation = noise(Vec3::new(x * 0.36, 11.0, -18.0));
        let peak = 1.5 + variation * 3.4;
        add_block(
            scene,
            Vec3::new(x, 8.0 + peak * 0.5, -19.2),
            Vec3::new(2.45, peak, 3.0),
            if index % 3 == 0 { 18 } else { 0 },
        );
    }

    for (index, x) in [-10.0, -5.0, 0.0, 5.0, 10.0].into_iter().enumerate() {
        add_hokage_face(
            scene,
            Vec3::new(x, 7.55 + (index % 2) as f32 * 0.28, -14.42),
            index,
        );
    }
}

fn add_hokage_face(scene: &mut Scene, center: Vec3, style: usize) {
    let stone = 21;
    let shadow = 0;

    // El relieve se construye por planos: craneo, mejillas, mandibula y rasgos salientes.
    add_ellipsoid_surface(
        scene,
        center + Vec3::new(0.0, 0.30, 0.0),
        Vec3::new(1.05, 0.94, 0.25),
        0.11,
        stone,
    );
    add_ellipsoid_surface(
        scene,
        center + Vec3::new(0.0, -0.72, 0.04),
        Vec3::new(0.78, 0.42, 0.23),
        0.10,
        stone,
    );
    add_ellipsoid_surface(
        scene,
        center + Vec3::new(0.0, -1.08, 0.02),
        Vec3::new(0.42, 0.20, 0.21),
        0.09,
        stone,
    );

    for side in [-1.0, 1.0] {
        add_ellipsoid_surface(
            scene,
            center + Vec3::new(side * 0.68, -0.28, 0.22),
            Vec3::new(0.34, 0.36, 0.18),
            0.085,
            stone,
        );
        add_block(
            scene,
            center + Vec3::new(side * 0.55, 0.33, 0.25),
            Vec3::new(0.62, 0.16, 0.22),
            shadow,
        );
        add_block(
            scene,
            center + Vec3::new(side * 0.55, 0.12, 0.30),
            Vec3::new(0.20, 0.13, 0.16),
            shadow,
        );
        add_ellipsoid_surface(
            scene,
            center + Vec3::new(side * 1.10, 0.12, -0.02),
            Vec3::new(0.16, 0.38, 0.20),
            0.075,
            stone,
        );
    }

    // Nariz en dos niveles y boca tallada.
    add_block(
        scene,
        center + Vec3::new(0.0, -0.02, 0.30),
        Vec3::new(0.26, 0.62, 0.28),
        stone,
    );
    add_block(
        scene,
        center + Vec3::new(0.0, -0.36, 0.37),
        Vec3::new(0.46, 0.18, 0.25),
        stone,
    );
    add_block(
        scene,
        center + Vec3::new(0.0, -0.70, 0.27),
        Vec3::new(0.68, 0.13, 0.20),
        shadow,
    );

    let hair_width = if style == 1 { 2.50 } else { 2.16 };
    add_block(
        scene,
        center + Vec3::new(0.0, 1.17, -0.03),
        Vec3::new(hair_width, 0.36, 0.40),
        shadow,
    );

    if style == 0 || style == 3 {
        for spike in -2i32..=2 {
            let spike_height = 0.28 + (2 - spike.abs()) as f32 * 0.10;
            add_block(
                scene,
                center + Vec3::new(spike as f32 * 0.38, 1.42 + spike_height * 0.5, -0.04),
                Vec3::new(0.30, spike_height, 0.34),
                shadow,
            );
        }
    } else if style == 1 {
        add_block(
            scene,
            center + Vec3::new(0.0, 1.48, -0.04),
            Vec3::new(1.42, 0.34, 0.36),
            shadow,
        );
    } else if style == 2 {
        add_block(
            scene,
            center + Vec3::new(0.0, 1.42, -0.04),
            Vec3::new(1.70, 0.25, 0.34),
            shadow,
        );
    } else {
        add_block(
            scene,
            center + Vec3::new(0.0, 1.48, -0.04),
            Vec3::new(1.02, 0.42, 0.34),
            shadow,
        );
    }
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
    for step in 0..8 {
        let t = step as f32 / 7.0;
        let bend = (step as f32 * 2.1).sin() * 0.35;
        let point = start * (1.0 - t) + end * t + Vec3::new(bend, 0.0, 0.0);
        add_block(scene, point, Vec3::new(0.20, 0.06, 1.15), 0);
    }
}

fn add_debris_field(scene: &mut Scene) {
    for index in 0..52 {
        let angle = index as f32 * 2.399_963;
        let radial_noise = noise(Vec3::new(index as f32 * 0.73, 1.7, 4.2));
        let radius = 8.0 + radial_noise * 5.2;
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
        add_block(
            scene,
            Vec3::new(x, 0.14 + height * 0.5, z),
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
        add_block(
            scene,
            Vec3::new(x, height * 0.5, z),
            Vec3::new(0.62, height, 0.72),
            18,
        );
        add_block(
            scene,
            Vec3::new(x + 0.34, height + 0.16, z - 0.15),
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

fn add_ellipsoid_surface(scene: &mut Scene, center: Vec3, radii: Vec3, cell: f32, material: usize) {
    let cells_x = (radii.x / cell).ceil() as i32;
    let cells_y = (radii.y / cell).ceil() as i32;
    let cells_z = (radii.z / cell).ceil() as i32;
    let inner = Vec3::new(
        (radii.x - cell * 1.35).max(0.0),
        (radii.y - cell * 1.35).max(0.0),
        (radii.z - cell * 1.35).max(0.0),
    );

    for y in -cells_y..=cells_y {
        for z in -cells_z..=cells_z {
            for x in -cells_x..=cells_x {
                let offset = Vec3::new(x as f32 * cell, y as f32 * cell, z as f32 * cell);
                let normalized = (offset.x / radii.x).powi(2)
                    + (offset.y / radii.y).powi(2)
                    + (offset.z / radii.z).powi(2);
                let inside_inner = inner.x > 0.0
                    && inner.y > 0.0
                    && inner.z > 0.0
                    && (offset.x / inner.x).powi(2)
                        + (offset.y / inner.y).powi(2)
                        + (offset.z / inner.z).powi(2)
                        < 1.0;
                if normalized > 1.0 || inside_inner {
                    continue;
                }

                let normal = Vec3::new(
                    offset.x / (radii.x * radii.x),
                    offset.y / (radii.y * radii.y),
                    offset.z / (radii.z * radii.z),
                );
                let voxel_center = center + offset;
                let size = Vec3::new(cell, cell, cell);
                scene.cubes.push(Cube {
                    min: voxel_center - size * 0.5,
                    max: voxel_center + size * 0.5,
                    material,
                    smooth_normal: (normal.length() > 0.0001).then(|| normal.normalized()),
                });
            }
        }
    }
}
