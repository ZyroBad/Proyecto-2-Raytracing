use std::env;
use std::f32::consts::PI;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::Path;

const EPSILON: f32 = 0.001;
const MAX_DEPTH: u32 = 3;

#[derive(Clone, Copy, Debug, Default)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    fn normalized(self) -> Self {
        let len = self.length();
        if len <= 0.0 {
            self
        } else {
            self / len
        }
    }

    fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    fn refract(self, normal: Self, eta: f32) -> Option<Self> {
        let cos_i = (-self).dot(normal).clamp(-1.0, 1.0);
        let sin2_t = eta * eta * (1.0 - cos_i * cos_i);
        if sin2_t > 1.0 {
            None
        } else {
            let cos_t = (1.0 - sin2_t).sqrt();
            Some(self * eta + normal * (eta * cos_i - cos_t))
        }
    }

    fn clamp01(self) -> Self {
        Self::new(
            self.x.clamp(0.0, 1.0),
            self.y.clamp(0.0, 1.0),
            self.z.clamp(0.0, 1.0),
        )
    }

    fn hadamard(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

type Color = Vec3;

#[derive(Clone, Copy)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy)]
enum MaterialKind {
    StatueStone,
    DarkRock,
    Grass,
    Water,
    Wood,
    Leaf,
    Waterfall,
}

#[derive(Clone, Copy)]
struct Material {
    kind: MaterialKind,
    albedo: Color,
    specular: f32,
    transparency: f32,
    reflectivity: f32,
    refractive_index: f32,
}

impl Material {
    fn texture(self, p: Vec3, normal: Vec3) -> Color {
        let checker = ((p.x.floor() as i32 + p.z.floor() as i32 + p.y.floor() as i32) & 1) as f32;
        match self.kind {
            MaterialKind::StatueStone => {
                let vein = noise(p * 2.7) * 0.18 + stripe(p.y + p.x * 0.15, 0.85) * 0.08;
                self.albedo * (0.75 + vein)
            }
            MaterialKind::DarkRock => {
                let cracks = stripe(p.x * 0.7 + p.z * 1.1, 0.35) * 0.25;
                self.albedo * (0.65 + noise(p * 3.0) * 0.25 - cracks)
            }
            MaterialKind::Grass => {
                let top = if normal.y > 0.6 { 1.0 } else { 0.65 };
                let patch = if checker > 0.0 { 0.12 } else { -0.04 };
                Color::new(0.22, 0.48, 0.16) * top + Color::new(patch, patch * 0.8, patch * 0.3)
            }
            MaterialKind::Water => {
                let ripple = (p.x * 8.0 + p.z * 5.0).sin() * 0.04 + noise(p * 5.0) * 0.05;
                Color::new(0.12 + ripple, 0.42 + ripple, 0.72 + ripple)
            }
            MaterialKind::Wood => {
                let grain = ((p.x * 8.0).sin() * 0.5 + (p.y * 11.0).sin() * 0.5) * 0.10;
                self.albedo * (0.8 + grain + checker * 0.08)
            }
            MaterialKind::Leaf => {
                let speckle = noise(p * 6.0) * 0.18;
                Color::new(0.08, 0.36 + speckle, 0.09)
            }
            MaterialKind::Waterfall => {
                let foam = stripe(p.y * 2.5 + p.x * 0.5, 0.55) * 0.35;
                Color::new(0.45 + foam, 0.75 + foam, 0.95)
            }
        }
        .clamp01()
    }
}

#[derive(Clone, Copy)]
struct Cube {
    min: Vec3,
    max: Vec3,
    material: usize,
}

struct Hit {
    point: Vec3,
    normal: Vec3,
    material: Material,
}

struct Scene {
    cubes: Vec<Cube>,
    materials: Vec<Material>,
    light_dir: Vec3,
    light_color: Color,
}

struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn look_at(origin: Vec3, target: Vec3, fov_deg: f32, aspect: f32) -> Self {
        let forward = (target - origin).normalized();
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(forward).normalized();
        let viewport_h = (fov_deg.to_radians() * 0.5).tan() * 2.0;
        let viewport_w = viewport_h * aspect;
        let horizontal = right * viewport_w;
        let vertical = up * viewport_h;
        let lower_left = origin + forward - horizontal * 0.5 - vertical * 0.5;
        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left + self.horizontal * u + self.vertical * v - self.origin)
                .normalized(),
        }
    }
}

#[derive(Clone)]
struct Config {
    width: usize,
    height: usize,
    frame: usize,
    frames: usize,
    animate: bool,
    output: String,
}

impl Config {
    fn from_args() -> Self {
        let mut cfg = Self {
            width: 480,
            height: 270,
            frame: 0,
            frames: 120,
            animate: false,
            output: "renders/valle_del_fin.ppm".to_string(),
        };

        let args: Vec<String> = env::args().collect();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--width" | "-w" => {
                    i += 1;
                    cfg.width = parse_or(&args, i, cfg.width);
                }
                "--height" | "-h" => {
                    i += 1;
                    cfg.height = parse_or(&args, i, cfg.height);
                }
                "--frame" => {
                    i += 1;
                    cfg.frame = parse_or(&args, i, cfg.frame);
                }
                "--frames" => {
                    i += 1;
                    cfg.frames = parse_or(&args, i, cfg.frames);
                }
                "--animate" => cfg.animate = true,
                "--output" | "-o" => {
                    i += 1;
                    if let Some(value) = args.get(i) {
                        cfg.output = value.clone();
                    }
                }
                "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }
        cfg
    }
}

fn parse_or<T: std::str::FromStr>(args: &[String], index: usize, fallback: T) -> T {
    args.get(index)
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(fallback)
}

fn print_help() {
    println!("Valle del Fin Raytracing");
    println!("cargo run --release -- [opciones]");
    println!("  --width N       ancho del render, default 480");
    println!("  --height N      alto del render, default 270");
    println!("  --frame N       frame individual para camara orbital");
    println!("  --frames N      cantidad de frames para animacion");
    println!("  --animate       renderiza todos los frames en frames/");
    println!("  --output PATH   salida PPM para un frame");
}

fn main() -> std::io::Result<()> {
    let cfg = Config::from_args();
    let scene = build_scene();

    if cfg.animate {
        create_dir_all("frames")?;
        for frame in 0..cfg.frames {
            let path = format!("frames/frame_{:04}.ppm", frame);
            render_to_file(&scene, &cfg, frame, &path)?;
            println!("rendered {}", path);
        }
    } else {
        render_to_file(&scene, &cfg, cfg.frame, &cfg.output)?;
        println!("rendered {}", cfg.output);
    }

    Ok(())
}

fn render_to_file(scene: &Scene, cfg: &Config, frame: usize, path: &str) -> std::io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    let aspect = cfg.width as f32 / cfg.height as f32;
    let t = frame as f32 / cfg.frames.max(1) as f32;
    let angle = t * 2.0 * PI;
    let zoom_wave = (t * 2.0 * PI).sin() * 0.18;
    let radius = 24.0 - zoom_wave * 7.0;
    let camera_pos = Vec3::new(
        angle.cos() * radius,
        10.0 + zoom_wave * 3.0,
        angle.sin() * radius,
    );
    let camera = Camera::look_at(camera_pos, Vec3::new(0.0, 3.0, 0.0), 48.0, aspect);

    let mut pixels = vec![Color::default(); cfg.width * cfg.height];

    for y in 0..cfg.height {
        for x in 0..cfg.width {
            let mut color = Color::default();
            for sy in 0..2 {
                for sx in 0..2 {
                    let u = (x as f32 + (sx as f32 + 0.5) * 0.5) / (cfg.width - 1) as f32;
                    let v = 1.0 - (y as f32 + (sy as f32 + 0.5) * 0.5) / (cfg.height - 1) as f32;
                    color += trace(scene, camera.ray(u, v), 0);
                }
            }
            color = color / 4.0;
            color = Color::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt()).clamp01();
            pixels[y * cfg.width + x] = color;
        }
    }

    if path.to_ascii_lowercase().ends_with(".bmp") {
        save_bmp(path, cfg.width, cfg.height, &pixels)?;
    } else {
        save_ppm(path, cfg.width, cfg.height, &pixels)?;
    }

    Ok(())
}

fn save_ppm(path: &str, width: usize, height: usize, pixels: &[Color]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;

    for y in 0..height {
        for x in 0..width {
            let color = pixels[y * width + x];
            write!(
                writer,
                "{} {} {} ",
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8
            )?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

fn save_bmp(path: &str, width: usize, height: usize, pixels: &[Color]) -> std::io::Result<()> {
    let row_stride = (width * 3 + 3) & !3;
    let pixel_data_size = row_stride * height;
    let file_size = 54 + pixel_data_size;
    let mut file = BufWriter::new(File::create(path)?);

    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&(54u32).to_le_bytes())?;
    file.write_all(&(40u32).to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&(1u16).to_le_bytes())?;
    file.write_all(&(24u16).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;

    let padding = vec![0u8; row_stride - width * 3];
    for y in (0..height).rev() {
        for x in 0..width {
            let color = pixels[y * width + x];
            file.write_all(&[
                (color.z * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.x * 255.0) as u8,
            ])?;
        }
        file.write_all(&padding)?;
    }

    Ok(())
}

fn trace(scene: &Scene, ray: Ray, depth: u32) -> Color {
    if depth >= MAX_DEPTH {
        return skybox(ray.direction);
    }

    if let Some(hit) = intersect_scene(scene, ray) {
        shade(scene, ray, hit, depth)
    } else {
        skybox(ray.direction)
    }
}

fn shade(scene: &Scene, ray: Ray, hit: Hit, depth: u32) -> Color {
    let view_dir = -ray.direction;
    let light_dir = -scene.light_dir.normalized();
    let base = hit.material.texture(hit.point, hit.normal);

    let shadow_ray = Ray {
        origin: hit.point + hit.normal * EPSILON,
        direction: light_dir,
    };
    let in_shadow = intersect_scene(scene, shadow_ray).is_some();
    let ndotl = hit.normal.dot(light_dir).max(0.0);
    let diffuse_strength = if in_shadow { 0.18 } else { ndotl };
    let diffuse = base.hadamard(scene.light_color) * diffuse_strength;

    let half_vec = (light_dir + view_dir).normalized();
    let spec = hit.normal.dot(half_vec).max(0.0).powf(48.0) * hit.material.specular;
    let specular = scene.light_color * spec;
    let ambient = base * 0.18;

    let mut color = ambient + diffuse + specular;

    if hit.material.reflectivity > 0.0 {
        let reflected = ray.direction.reflect(hit.normal).normalized();
        let reflected_color = trace(
            scene,
            Ray {
                origin: hit.point + hit.normal * EPSILON,
                direction: reflected,
            },
            depth + 1,
        );
        color =
            color * (1.0 - hit.material.reflectivity) + reflected_color * hit.material.reflectivity;
    }

    if hit.material.transparency > 0.0 {
        let entering = ray.direction.dot(hit.normal) < 0.0;
        let normal = if entering { hit.normal } else { -hit.normal };
        let eta = if entering {
            1.0 / hit.material.refractive_index
        } else {
            hit.material.refractive_index
        };
        let refracted = ray
            .direction
            .refract(normal, eta)
            .unwrap_or_else(|| ray.direction.reflect(normal));
        let refracted_color = trace(
            scene,
            Ray {
                origin: hit.point - normal * EPSILON * 2.0,
                direction: refracted.normalized(),
            },
            depth + 1,
        );
        color =
            color * (1.0 - hit.material.transparency) + refracted_color * hit.material.transparency;
    }

    color.clamp01()
}

fn intersect_scene(scene: &Scene, ray: Ray) -> Option<Hit> {
    let mut closest: Option<Hit> = None;
    let mut closest_t = f32::INFINITY;

    for cube in &scene.cubes {
        if let Some((t, normal)) = intersect_cube(ray, *cube) {
            if t > EPSILON && t < closest_t {
                closest_t = t;
                closest = Some(Hit {
                    point: ray.at(t),
                    normal,
                    material: scene.materials[cube.material],
                });
            }
        }
    }

    closest
}

fn intersect_cube(ray: Ray, cube: Cube) -> Option<(f32, Vec3)> {
    let mut t_min = -f32::INFINITY;
    let mut t_max = f32::INFINITY;
    let mut hit_normal = Vec3::default();

    let axes = [
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
    ];

    for (origin, direction, min_v, max_v, n_min, n_max) in axes {
        if direction.abs() < 0.00001 {
            if origin < min_v || origin > max_v {
                return None;
            }
            continue;
        }

        let inv = 1.0 / direction;
        let mut t0 = (min_v - origin) * inv;
        let mut t1 = (max_v - origin) * inv;
        let mut normal = n_min;
        if inv < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
            normal = n_max;
        }

        if t0 > t_min {
            t_min = t0;
            hit_normal = normal;
        }
        t_max = t_max.min(t1);
        if t_min > t_max {
            return None;
        }
    }

    if t_min > EPSILON {
        Some((t_min, hit_normal))
    } else if t_max > EPSILON {
        Some((t_max, -hit_normal))
    } else {
        None
    }
}

fn skybox(dir: Vec3) -> Color {
    let t = (dir.y * 0.5 + 0.5).clamp(0.0, 1.0);
    let horizon = Color::new(0.95, 0.55, 0.32);
    let zenith = Color::new(0.12, 0.23, 0.52);
    let mut color = horizon * (1.0 - t) + zenith * t;

    let sun_dir = Vec3::new(-0.45, 0.58, -0.68).normalized();
    let sun = dir.dot(sun_dir).max(0.0).powf(320.0);
    color += Color::new(1.0, 0.72, 0.35) * sun;

    let cloud = ((dir.x * 18.0 + dir.z * 11.0).sin() * 0.5 + 0.5)
        * (1.0 - (dir.y - 0.25).abs() * 4.0).max(0.0);
    color += Color::new(0.30, 0.24, 0.18) * cloud * 0.25;
    color.clamp01()
}

fn build_scene() -> Scene {
    let materials = vec![
        Material {
            kind: MaterialKind::StatueStone,
            albedo: Color::new(0.58, 0.57, 0.53),
            specular: 0.18,
            transparency: 0.0,
            reflectivity: 0.05,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::DarkRock,
            albedo: Color::new(0.22, 0.21, 0.20),
            specular: 0.22,
            transparency: 0.0,
            reflectivity: 0.12,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Grass,
            albedo: Color::new(0.18, 0.42, 0.13),
            specular: 0.04,
            transparency: 0.0,
            reflectivity: 0.0,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Water,
            albedo: Color::new(0.12, 0.42, 0.72),
            specular: 0.9,
            transparency: 0.55,
            reflectivity: 0.35,
            refractive_index: 1.33,
        },
        Material {
            kind: MaterialKind::Wood,
            albedo: Color::new(0.45, 0.25, 0.10),
            specular: 0.12,
            transparency: 0.0,
            reflectivity: 0.03,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Leaf,
            albedo: Color::new(0.08, 0.32, 0.08),
            specular: 0.05,
            transparency: 0.0,
            reflectivity: 0.0,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Waterfall,
            albedo: Color::new(0.55, 0.82, 0.95),
            specular: 0.7,
            transparency: 0.35,
            reflectivity: 0.22,
            refractive_index: 1.33,
        },
    ];

    let mut scene = Scene {
        cubes: Vec::new(),
        materials,
        light_dir: Vec3::new(0.35, -0.85, 0.25).normalized(),
        light_color: Color::new(1.0, 0.86, 0.68),
    };

    build_valley(&mut scene);
    scene
}

fn build_valley(scene: &mut Scene) {
    add_cube(
        scene,
        Vec3::new(-12.0, -1.0, -8.0),
        Vec3::new(12.0, 0.0, 8.0),
        1,
    );
    add_cube(
        scene,
        Vec3::new(-11.0, 0.0, -7.0),
        Vec3::new(11.0, 0.35, 7.0),
        2,
    );
    add_cube(
        scene,
        Vec3::new(-2.4, 0.05, -8.5),
        Vec3::new(2.4, 0.25, 8.5),
        3,
    );
    add_cube(
        scene,
        Vec3::new(-1.0, 0.25, -8.8),
        Vec3::new(1.0, 4.5, -7.9),
        6,
    );

    add_cliff(scene, -8.0);
    add_cliff(scene, 8.0);
    add_statue(scene, Vec3::new(-6.0, 0.35, -1.8), 1.0);
    add_statue(scene, Vec3::new(6.0, 0.35, 1.8), -1.0);

    for x in [-10.0, -7.0, 8.0, 10.0, -4.0, 4.0] {
        for z in [-6.0, 6.0] {
            add_tree(scene, Vec3::new(x, 0.35, z));
        }
    }

    add_cube(
        scene,
        Vec3::new(-2.1, 0.35, -1.0),
        Vec3::new(2.1, 0.58, 1.0),
        4,
    );
}

fn add_cliff(scene: &mut Scene, x_center: f32) {
    for y in 0..5 {
        let h = y as f32;
        let width = 4.5 - h * 0.35;
        add_cube(
            scene,
            Vec3::new(x_center - width, 0.0 + h, -6.0 + h * 0.3),
            Vec3::new(x_center + width, 1.0 + h, 6.0 - h * 0.3),
            1,
        );
    }
}

fn add_statue(scene: &mut Scene, base: Vec3, facing: f32) {
    let stone = 0;
    add_cube(
        scene,
        base + Vec3::new(-1.0, 0.0, -0.8),
        base + Vec3::new(1.0, 1.2, 0.8),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.65, 1.2, -0.45),
        base + Vec3::new(0.65, 3.9, 0.45),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.8, 3.9, -0.65),
        base + Vec3::new(0.8, 5.1, 0.65),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.95, 5.1, -0.75),
        base + Vec3::new(0.95, 6.1, 0.75),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.55, 6.1, -0.45),
        base + Vec3::new(0.55, 7.0, 0.45),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.8, 3.2, -0.35),
        base + Vec3::new(-0.65, 3.9, 0.35),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(0.65, 3.2, -0.35),
        base + Vec3::new(1.8, 3.9, 0.35),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.95, 2.0, facing * 0.15 - 0.25),
        base + Vec3::new(-1.2, 3.3, facing * 0.15 + 0.25),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(1.2, 2.0, facing * -0.15 - 0.25),
        base + Vec3::new(1.95, 3.3, facing * -0.15 + 0.25),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.7, 7.0, -0.5),
        base + Vec3::new(0.7, 7.65, 0.5),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.35, 7.65, -0.3),
        base + Vec3::new(0.35, 8.1, 0.3),
        stone,
    );
}

fn add_tree(scene: &mut Scene, base: Vec3) {
    add_cube(
        scene,
        base + Vec3::new(-0.18, 0.0, -0.18),
        base + Vec3::new(0.18, 1.1, 0.18),
        4,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.75, 0.9, -0.75),
        base + Vec3::new(0.75, 1.75, 0.75),
        5,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.55, 1.55, -0.55),
        base + Vec3::new(0.55, 2.25, 0.55),
        5,
    );
}

fn add_cube(scene: &mut Scene, min: Vec3, max: Vec3, material: usize) {
    scene.cubes.push(Cube { min, max, material });
}

fn noise(p: Vec3) -> f32 {
    let n = (p.x * 12.9898 + p.y * 78.233 + p.z * 37.719).sin() * 43_758.547;
    n.fract().abs()
}

fn stripe(v: f32, frequency: f32) -> f32 {
    if (v * frequency).sin() > 0.72 {
        1.0
    } else {
        0.0
    }
}
