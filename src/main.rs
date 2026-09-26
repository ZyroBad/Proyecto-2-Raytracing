use std::env;
use std::f32::consts::PI;
use std::fs::{create_dir_all, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

const EPSILON: f32 = 0.001;
const DEFAULT_MAX_DEPTH: u32 = 3;

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
    Rock,
    Cloud,
    GamabuntaSkin,
    GamakenSkin,
    GamahiroSkin,
    Robe,
    OrangeCloth,
    RedCloak,
    NarutoSkin,
    NarutoHair,
    Ink,
    Metal,
    Chakra,
    Wood,
    EyeGold,
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
            MaterialKind::Rock => {
                let cracks = stripe(p.x * 0.55 + p.z * 0.9, 0.45) * 0.16;
                self.albedo * (0.72 + noise(p * 2.8) * 0.28 - cracks)
            }
            MaterialKind::Cloud => {
                let soft = noise(p * 1.7) * 0.10 + if normal.y > 0.5 { 0.08 } else { 0.0 };
                self.albedo * (0.88 + soft)
            }
            MaterialKind::GamabuntaSkin => {
                let spots = stripe(p.x * 0.8 + p.y * 0.35, 1.6) * noise(p * 3.0) * 0.20;
                self.albedo * (0.82 + noise(p * 4.0) * 0.16 - spots)
            }
            MaterialKind::GamakenSkin => {
                let mottled = noise(p * 4.8) * 0.18 + checker * 0.04;
                self.albedo * (0.78 + mottled)
            }
            MaterialKind::GamahiroSkin => {
                let bands = stripe(p.y + p.x * 0.12, 1.8) * 0.08;
                self.albedo * (0.84 + noise(p * 3.2) * 0.12 - bands)
            }
            MaterialKind::Robe => {
                let weave = checker * 0.05 + stripe(p.y, 4.0) * 0.04;
                self.albedo * (0.78 + weave)
            }
            MaterialKind::OrangeCloth => {
                let weave = noise(p * 10.0) * 0.10;
                self.albedo * (0.86 + weave)
            }
            MaterialKind::RedCloak => {
                let fold = stripe(p.x + p.y * 0.2, 1.7) * 0.10;
                self.albedo * (0.78 + noise(p * 5.0) * 0.10 + fold)
            }
            MaterialKind::NarutoSkin => self.albedo * (0.90 + noise(p * 8.0) * 0.08),
            MaterialKind::NarutoHair => {
                let highlight = stripe(p.x - p.y, 3.8) * 0.18;
                self.albedo * (0.82 + highlight)
            }
            MaterialKind::Ink => self.albedo * (0.78 + noise(p * 7.0) * 0.12),
            MaterialKind::Metal => {
                let scratch = stripe(p.y + p.x * 0.3, 7.0) * 0.16;
                self.albedo * (0.74 + scratch)
            }
            MaterialKind::Chakra => {
                let pulse = (p.x * 5.0 + p.y * 3.0 + p.z * 4.0).sin() * 0.08;
                Color::new(0.20 + pulse, 0.58 + pulse, 0.96)
            }
            MaterialKind::Wood => {
                let grain = ((p.x * 8.0).sin() + (p.y * 11.0).sin()) * 0.06;
                self.albedo * (0.80 + grain + checker * 0.06)
            }
            MaterialKind::EyeGold => {
                let glow = if normal.z > 0.5 { 0.18 } else { 0.02 };
                self.albedo * (0.82 + glow + noise(p * 9.0) * 0.05)
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
    interactive: bool,
    summary: bool,
    angle_deg: Option<f32>,
    zoom: f32,
    samples_per_axis: usize,
    max_depth: u32,
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
            interactive: false,
            summary: false,
            angle_deg: None,
            zoom: 1.0,
            samples_per_axis: 2,
            max_depth: DEFAULT_MAX_DEPTH,
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
                "--interactive" => cfg.interactive = true,
                "--summary" => cfg.summary = true,
                "--angle" => {
                    i += 1;
                    cfg.angle_deg = Some(parse_or(&args, i, 45.0));
                }
                "--zoom" => {
                    i += 1;
                    cfg.zoom = parse_or(&args, i, cfg.zoom);
                }
                "--samples" => {
                    i += 1;
                    cfg.samples_per_axis = parse_or(&args, i, cfg.samples_per_axis).clamp(1, 4);
                }
                "--depth" => {
                    i += 1;
                    cfg.max_depth = parse_or(&args, i, cfg.max_depth).clamp(1, 6);
                }
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
    println!("  --interactive   modo consola para ajustar camara y renderizar previews");
    println!("  --summary       imprime resumen de escena sin renderizar");
    println!("  --angle N       angulo manual de camara en grados");
    println!("  --zoom N        zoom manual, mayor acerca la camara");
    println!("  --samples N     muestras por eje, 1 rapido, 2 default, 4 fino");
    println!("  --depth N       rebotes maximos de raytracing, default 3");
    println!("  --output PATH   salida PPM para un frame");
}

fn main() -> std::io::Result<()> {
    let cfg = Config::from_args();
    let scene = build_scene();

    if cfg.summary {
        print_scene_summary(&scene);
    } else if cfg.interactive {
        run_interactive(&scene, cfg)?;
    } else if cfg.animate {
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

fn print_scene_summary(scene: &Scene) {
    println!("Resumen de escena");
    println!("Cubos: {}", scene.cubes.len());
    println!("Materiales: {}", scene.materials.len());
    println!("Efectos: sombras, specular, reflexion, refraccion y skybox procedural");
}

fn run_interactive(scene: &Scene, mut cfg: Config) -> std::io::Result<()> {
    create_dir_all("renders")?;
    cfg.output = "renders/interactive.bmp".to_string();
    cfg.angle_deg = cfg.angle_deg.or(Some(90.0));
    cfg.zoom = cfg.zoom.max(0.35);

    println!("Modo interactivo de camara");
    println!(
        "a/d: rotar | w/s: subir/bajar angulo orbital fino | +/-: zoom | r: render | q: salir"
    );
    println!("Cada render se guarda en renders/interactive.bmp");

    render_to_file(scene, &cfg, 0, &cfg.output)?;
    println!(
        "rendered {} | angle {:.1} | zoom {:.2}",
        cfg.output,
        cfg.angle_deg.unwrap_or(90.0),
        cfg.zoom
    );

    loop {
        print!("comando> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let command = input.trim().to_ascii_lowercase();
        match command.as_str() {
            "a" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) - 12.0),
            "d" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) + 12.0),
            "w" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) + 4.0),
            "s" => cfg.angle_deg = Some(cfg.angle_deg.unwrap_or(90.0) - 4.0),
            "+" | "=" => cfg.zoom = (cfg.zoom + 0.15).min(2.5),
            "-" | "_" => cfg.zoom = (cfg.zoom - 0.15).max(0.45),
            "r" | "" => {}
            "q" | "salir" => break,
            _ => {
                println!("Comando no reconocido. Usa a/d, w/s, +/-, r o q.");
                continue;
            }
        }

        render_to_file(scene, &cfg, 0, &cfg.output)?;
        println!(
            "rendered {} | angle {:.1} | zoom {:.2}",
            cfg.output,
            cfg.angle_deg.unwrap_or(90.0),
            cfg.zoom
        );
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
    let angle = cfg
        .angle_deg
        .map(|a| a.to_radians())
        .unwrap_or(t * 2.0 * PI + PI * 0.5);
    let zoom_wave = (t * 2.0 * PI).sin() * 0.18;
    let radius = (27.0 - zoom_wave * 6.0) / cfg.zoom.max(0.35);
    let camera_pos = Vec3::new(
        angle.cos() * radius,
        8.5 + zoom_wave * 2.5,
        angle.sin() * radius,
    );
    let camera = Camera::look_at(camera_pos, Vec3::new(0.0, 4.7, 0.0), 48.0, aspect);

    let mut pixels = vec![Color::default(); cfg.width * cfg.height];

    for y in 0..cfg.height {
        for x in 0..cfg.width {
            let mut color = Color::default();
            let samples = cfg.samples_per_axis;
            let inv_samples = 1.0 / samples as f32;
            for sy in 0..samples {
                for sx in 0..samples {
                    let u = (x as f32 + (sx as f32 + 0.5) * inv_samples) / (cfg.width - 1) as f32;
                    let v = 1.0
                        - (y as f32 + (sy as f32 + 0.5) * inv_samples) / (cfg.height - 1) as f32;
                    color += trace(scene, camera.ray(u, v), 0, cfg.max_depth);
                }
            }
            color = color / (samples * samples) as f32;
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

fn trace(scene: &Scene, ray: Ray, depth: u32, max_depth: u32) -> Color {
    if depth >= max_depth {
        return skybox(ray.direction);
    }

    if let Some(hit) = intersect_scene(scene, ray) {
        shade(scene, ray, hit, depth, max_depth)
    } else {
        skybox(ray.direction)
    }
}

fn shade(scene: &Scene, ray: Ray, hit: Hit, depth: u32, max_depth: u32) -> Color {
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
            max_depth,
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
            max_depth,
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
    let horizon = Color::new(0.72, 0.88, 1.00);
    let zenith = Color::new(0.10, 0.38, 0.76);
    let mut color = horizon * (1.0 - t) + zenith * t;

    let sun_dir = Vec3::new(-0.45, 0.62, 0.64).normalized();
    let sun = dir.dot(sun_dir).max(0.0).powf(320.0);
    let glow = dir.dot(sun_dir).max(0.0).powf(18.0);
    color += Color::new(1.0, 0.94, 0.70) * sun;
    color += Color::new(0.48, 0.66, 0.92) * glow * 0.22;

    let cloud = ((dir.x * 16.0 + dir.z * 10.0).sin() * 0.5 + 0.5)
        * (1.0 - (dir.y - 0.18).abs() * 5.0).max(0.0);
    color += Color::new(0.82, 0.88, 0.96) * cloud * 0.28;
    color.clamp01()
}

fn build_scene() -> Scene {
    let materials = vec![
        Material {
            kind: MaterialKind::Rock,
            albedo: Color::new(0.43, 0.33, 0.23),
            specular: 0.12,
            transparency: 0.0,
            reflectivity: 0.04,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Cloud,
            albedo: Color::new(0.88, 0.92, 0.98),
            specular: 0.32,
            transparency: 0.06,
            reflectivity: 0.08,
            refractive_index: 1.05,
        },
        Material {
            kind: MaterialKind::GamabuntaSkin,
            albedo: Color::new(0.56, 0.18, 0.13),
            specular: 0.24,
            transparency: 0.0,
            reflectivity: 0.04,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::GamakenSkin,
            albedo: Color::new(0.62, 0.16, 0.40),
            specular: 0.22,
            transparency: 0.0,
            reflectivity: 0.03,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::GamahiroSkin,
            albedo: Color::new(0.42, 0.72, 0.70),
            specular: 0.30,
            transparency: 0.0,
            reflectivity: 0.05,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Robe,
            albedo: Color::new(0.07, 0.10, 0.16),
            specular: 0.16,
            transparency: 0.0,
            reflectivity: 0.05,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::OrangeCloth,
            albedo: Color::new(0.93, 0.30, 0.035),
            specular: 0.10,
            transparency: 0.0,
            reflectivity: 0.02,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::RedCloak,
            albedo: Color::new(0.66, 0.055, 0.04),
            specular: 0.14,
            transparency: 0.0,
            reflectivity: 0.03,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::NarutoSkin,
            albedo: Color::new(0.91, 0.58, 0.38),
            specular: 0.20,
            transparency: 0.0,
            reflectivity: 0.02,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::NarutoHair,
            albedo: Color::new(1.00, 0.72, 0.06),
            specular: 0.34,
            transparency: 0.0,
            reflectivity: 0.05,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Ink,
            albedo: Color::new(0.018, 0.022, 0.030),
            specular: 0.12,
            transparency: 0.0,
            reflectivity: 0.04,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Metal,
            albedo: Color::new(0.62, 0.68, 0.74),
            specular: 0.95,
            transparency: 0.0,
            reflectivity: 0.58,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::Chakra,
            albedo: Color::new(0.18, 0.55, 0.96),
            specular: 0.82,
            transparency: 0.48,
            reflectivity: 0.18,
            refractive_index: 1.18,
        },
        Material {
            kind: MaterialKind::Wood,
            albedo: Color::new(0.30, 0.12, 0.055),
            specular: 0.12,
            transparency: 0.0,
            reflectivity: 0.03,
            refractive_index: 1.0,
        },
        Material {
            kind: MaterialKind::EyeGold,
            albedo: Color::new(0.96, 0.68, 0.08),
            specular: 0.70,
            transparency: 0.0,
            reflectivity: 0.16,
            refractive_index: 1.0,
        },
    ];

    let mut scene = Scene {
        cubes: Vec::new(),
        materials,
        light_dir: Vec3::new(0.42, -0.82, -0.30).normalized(),
        light_color: Color::new(1.0, 0.95, 0.84),
    };

    build_sage_arrival(&mut scene);
    scene
}

fn build_sage_arrival(scene: &mut Scene) {
    add_block(
        scene,
        Vec3::new(0.0, -0.65, 0.0),
        Vec3::new(24.0, 1.3, 12.0),
        0,
    );
    add_block(
        scene,
        Vec3::new(0.0, 0.08, 0.4),
        Vec3::new(16.0, 0.25, 7.5),
        0,
    );

    add_gamabunta(scene, Vec3::new(0.0, 0.0, -0.3));
    add_gamaken(scene, Vec3::new(-7.2, 0.0, -1.3));
    add_gamahiro(scene, Vec3::new(7.2, 0.0, -1.3));
    add_naruto_sage(scene, Vec3::new(0.0, 8.0, 0.25));
    add_summoning_clouds(scene);
}

fn add_gamabunta(scene: &mut Scene, base: Vec3) {
    let skin = 2;
    let robe = 5;
    let pale = 1;
    let ink = 10;
    let eye = 14;

    add_block(
        scene,
        base + Vec3::new(-2.5, 0.8, 1.1),
        Vec3::new(3.2, 1.2, 3.2),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(2.5, 0.8, 1.1),
        Vec3::new(3.2, 1.2, 3.2),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.8, 0.0),
        Vec3::new(6.6, 4.0, 3.8),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.9, 2.0),
        Vec3::new(3.6, 2.8, 0.35),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-2.7, 3.4, 0.9),
        Vec3::new(1.35, 3.8, 2.2),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(2.7, 3.4, 0.9),
        Vec3::new(1.35, 3.8, 2.2),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.8, 0.6),
        Vec3::new(6.2, 2.6, 3.2),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.35, 2.2),
        Vec3::new(4.6, 1.0, 1.1),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.55, 6.35, 2.25),
        Vec3::new(1.05, 0.64, 0.28),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(1.55, 6.35, 2.25),
        Vec3::new(1.05, 0.64, 0.28),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-1.55, 6.34, 2.42),
        Vec3::new(0.24, 0.52, 0.16),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.55, 6.34, 2.42),
        Vec3::new(0.24, 0.52, 0.16),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(-0.72, 5.62, 2.80),
        Vec3::new(0.22, 0.20, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.72, 5.62, 2.80),
        Vec3::new(0.22, 0.20, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.85, 2.77),
        Vec3::new(2.7, 0.18, 0.14),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 6.95, 0.0),
        Vec3::new(1.1, 0.55, 1.2),
        skin,
    );
}

fn add_gamaken(scene: &mut Scene, base: Vec3) {
    let skin = 3;
    let robe = 5;
    let pale = 1;
    let ink = 10;
    let metal = 11;
    let eye = 14;

    add_block(
        scene,
        base + Vec3::new(-1.7, 0.8, 0.8),
        Vec3::new(2.3, 1.1, 2.6),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(1.7, 0.8, 0.8),
        Vec3::new(2.3, 1.1, 2.6),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.7, 0.0),
        Vec3::new(4.8, 3.8, 3.0),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 3.0, 1.65),
        Vec3::new(2.6, 2.5, 0.3),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.9, 3.0, 0.6),
        Vec3::new(1.15, 3.5, 1.8),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(1.9, 3.0, 0.6),
        Vec3::new(1.15, 3.5, 1.8),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.45, 0.55),
        Vec3::new(4.5, 2.4, 2.8),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.0, 1.94),
        Vec3::new(3.35, 0.92, 0.95),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.1, 5.9, 1.98),
        Vec3::new(0.78, 0.52, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(1.1, 5.9, 1.98),
        Vec3::new(0.78, 0.52, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-1.1, 5.9, 2.12),
        Vec3::new(0.18, 0.40, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.1, 5.9, 2.12),
        Vec3::new(0.18, 0.40, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.58, 2.43),
        Vec3::new(2.2, 0.16, 0.12),
        ink,
    );

    add_block(
        scene,
        base + Vec3::new(-3.15, 4.5, 0.3),
        Vec3::new(0.32, 8.8, 0.32),
        metal,
    );
    for x in [-3.75, -3.15, -2.55] {
        add_block(
            scene,
            base + Vec3::new(x, 8.85, 0.3),
            Vec3::new(0.24, 1.6, 0.24),
            metal,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-3.15, 8.15, 0.3),
        Vec3::new(1.45, 0.25, 0.25),
        metal,
    );
}

fn add_gamahiro(scene: &mut Scene, base: Vec3) {
    let skin = 4;
    let robe = 5;
    let orange = 6;
    let pale = 1;
    let ink = 10;
    let metal = 11;
    let wood = 13;
    let eye = 14;

    add_block(
        scene,
        base + Vec3::new(-1.8, 0.8, 0.8),
        Vec3::new(2.5, 1.1, 2.7),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(1.8, 0.8, 0.8),
        Vec3::new(2.5, 1.1, 2.7),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.8, 0.0),
        Vec3::new(5.0, 4.0, 3.1),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.5, 1.7),
        Vec3::new(2.8, 2.6, 0.3),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.75, 1.95),
        Vec3::new(3.8, 0.55, 0.42),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(-2.0, 3.1, 0.6),
        Vec3::new(1.15, 3.6, 1.9),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(2.0, 3.1, 0.6),
        Vec3::new(1.15, 3.6, 1.9),
        robe,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.65, 0.5),
        Vec3::new(4.7, 2.5, 2.9),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 5.15, 1.95),
        Vec3::new(3.5, 0.95, 1.0),
        pale,
    );
    add_block(
        scene,
        base + Vec3::new(-1.15, 6.08, 2.0),
        Vec3::new(0.80, 0.54, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(1.15, 6.08, 2.0),
        Vec3::new(0.80, 0.54, 0.24),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-1.15, 6.08, 2.14),
        Vec3::new(0.18, 0.42, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(1.15, 6.08, 2.14),
        Vec3::new(0.18, 0.42, 0.12),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 4.70, 2.45),
        Vec3::new(2.35, 0.16, 0.12),
        ink,
    );

    for x in [-1.75, 1.75] {
        add_block(
            scene,
            base + Vec3::new(x, 7.9, -0.9),
            Vec3::new(0.48, 5.0, 0.48),
            wood,
        );
        add_block(
            scene,
            base + Vec3::new(x, 9.0, -0.9),
            Vec3::new(0.24, 3.3, 0.24),
            metal,
        );
    }
}

fn add_naruto_sage(scene: &mut Scene, base: Vec3) {
    let orange = 6;
    let cloak = 7;
    let skin = 8;
    let hair = 9;
    let ink = 10;
    let metal = 11;
    let chakra = 12;
    let eye = 14;

    add_block(
        scene,
        base + Vec3::new(-0.48, 0.55, 0.0),
        Vec3::new(0.65, 1.25, 0.75),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.48, 0.55, 0.0),
        Vec3::new(0.65, 1.25, 0.75),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.65, 0.0),
        Vec3::new(1.55, 1.25, 0.85),
        orange,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 1.65, -0.55),
        Vec3::new(2.0, 1.65, 0.28),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(-0.82, 1.82, 0.35),
        Vec3::new(0.38, 1.15, 0.42),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(0.82, 1.82, 0.35),
        Vec3::new(0.38, 1.15, 0.42),
        cloak,
    );
    add_block(
        scene,
        base + Vec3::new(-0.38, 1.78, 0.72),
        Vec3::new(0.82, 0.30, 0.34),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.38, 1.78, 0.72),
        Vec3::new(0.82, 0.30, 0.34),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.72, 0.05),
        Vec3::new(1.22, 1.05, 0.92),
        skin,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.94, 0.55),
        Vec3::new(1.28, 0.26, 0.16),
        metal,
    );
    add_block(
        scene,
        base + Vec3::new(-0.33, 2.76, 0.58),
        Vec3::new(0.30, 0.16, 0.10),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(0.33, 2.76, 0.58),
        Vec3::new(0.30, 0.16, 0.10),
        eye,
    );
    add_block(
        scene,
        base + Vec3::new(-0.33, 2.76, 0.65),
        Vec3::new(0.08, 0.13, 0.07),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.33, 2.76, 0.65),
        Vec3::new(0.08, 0.13, 0.07),
        ink,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 2.48, 0.58),
        Vec3::new(0.42, 0.08, 0.07),
        ink,
    );
    for x in [-0.48, -0.24, 0.0, 0.24, 0.48] {
        let height = if x == 0.0 { 0.72 } else { 0.55 };
        add_block(
            scene,
            base + Vec3::new(x, 3.45, -0.02),
            Vec3::new(0.28, height, 0.52),
            hair,
        );
    }
    add_block(
        scene,
        base + Vec3::new(-0.95, 2.0, -0.1),
        Vec3::new(0.16, 2.9, 0.18),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(0.95, 2.0, -0.1),
        Vec3::new(0.16, 2.9, 0.18),
        chakra,
    );
    add_block(
        scene,
        base + Vec3::new(0.0, 3.55, -0.1),
        Vec3::new(1.7, 0.14, 0.20),
        chakra,
    );
}

fn add_summoning_clouds(scene: &mut Scene) {
    for (x, z, scale) in [
        (-10.0, 2.8, 1.2),
        (-7.0, 3.5, 1.4),
        (-3.8, 3.8, 1.15),
        (0.0, 4.0, 1.45),
        (3.7, 3.7, 1.2),
        (7.0, 3.4, 1.4),
        (10.0, 2.7, 1.2),
        (-9.4, -2.5, 0.9),
        (9.4, -2.5, 0.9),
    ] {
        add_cloud_cluster(scene, Vec3::new(x, 0.65, z), scale);
    }
}

fn add_cloud_cluster(scene: &mut Scene, base: Vec3, scale: f32) {
    for (offset, size) in [
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 1.1, 1.8)),
        (Vec3::new(-1.2, 0.35, 0.1), Vec3::new(1.8, 1.2, 1.5)),
        (Vec3::new(1.2, 0.42, -0.1), Vec3::new(1.9, 1.35, 1.6)),
        (Vec3::new(0.0, 0.65, 0.0), Vec3::new(1.7, 1.2, 1.4)),
    ] {
        add_block(scene, base + offset * scale, size * scale, 1);
    }
}

fn add_block(scene: &mut Scene, center: Vec3, size: Vec3, material: usize) {
    add_cube(scene, center - size * 0.5, center + size * 0.5, material);
}

#[allow(dead_code)]
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
        Vec3::new(-3.6, 0.08, -8.7),
        Vec3::new(3.6, 0.28, 8.7),
        3,
    );
    add_trail_stones(scene);
    add_cube(
        scene,
        Vec3::new(-1.0, 0.25, -8.8),
        Vec3::new(1.0, 4.5, -7.9),
        6,
    );
    add_water_details(scene);
    add_back_wall(scene);

    add_cliff(scene, -8.0);
    add_cliff(scene, 8.0);
    add_statue(scene, Vec3::new(-6.4, 0.35, -1.9), 1.0);
    add_statue(scene, Vec3::new(6.4, 0.35, 1.9), -1.0);

    for x in [-10.0, -7.0, 8.0, 10.0, -4.0, 4.0] {
        for z in [-6.0, 6.0] {
            add_tree(scene, Vec3::new(x, 0.35, z));
        }
    }
    for x in [-9.0, -7.4, 7.4, 9.0] {
        add_moss_patch(scene, Vec3::new(x, 5.03, -2.2));
        add_moss_patch(scene, Vec3::new(x, 5.03, 2.2));
    }
    add_rock_cluster(scene, Vec3::new(-4.0, 0.35, -4.2));
    add_rock_cluster(scene, Vec3::new(4.5, 0.35, 4.4));
    add_rock_cluster(scene, Vec3::new(-0.5, 0.35, 5.5));

    add_cube(
        scene,
        Vec3::new(-2.1, 0.35, -1.0),
        Vec3::new(2.1, 0.58, 1.0),
        4,
    );
}

fn add_water_details(scene: &mut Scene) {
    for z in [-6.9, -5.8, -4.6, 4.6, 5.8, 6.9] {
        add_cube(
            scene,
            Vec3::new(-2.8, 0.30, z),
            Vec3::new(2.8, 0.36, z + 0.22),
            7,
        );
    }
    for y in 0..5 {
        let fy = 0.5 + y as f32 * 0.75;
        add_cube(
            scene,
            Vec3::new(-1.15, fy, -7.78),
            Vec3::new(1.15, fy + 0.08, -7.64),
            7,
        );
    }
    add_cube(
        scene,
        Vec3::new(-1.8, 0.29, -8.05),
        Vec3::new(1.8, 0.38, -7.45),
        7,
    );
}

fn add_back_wall(scene: &mut Scene) {
    add_cube(
        scene,
        Vec3::new(-4.4, 0.0, -9.45),
        Vec3::new(4.4, 5.0, -8.95),
        1,
    );
    add_cube(
        scene,
        Vec3::new(-6.0, 0.0, -9.20),
        Vec3::new(-3.6, 3.5, -8.70),
        1,
    );
    add_cube(
        scene,
        Vec3::new(3.6, 0.0, -9.20),
        Vec3::new(6.0, 3.5, -8.70),
        1,
    );
    add_cube(
        scene,
        Vec3::new(-2.4, 4.45, -9.60),
        Vec3::new(2.4, 5.15, -8.85),
        0,
    );
    for x in [-3.6, -2.4, 2.4, 3.6] {
        add_moss_patch(scene, Vec3::new(x, 5.17, -9.10));
    }
}

fn add_trail_stones(scene: &mut Scene) {
    for z in [-5.2, -3.6, 3.6, 5.2] {
        add_cube(
            scene,
            Vec3::new(-1.05, 0.34, z),
            Vec3::new(1.05, 0.43, z + 0.72),
            9,
        );
    }
    for z in [-1.6, 1.35] {
        add_cube(
            scene,
            Vec3::new(-1.55, 0.34, z),
            Vec3::new(1.55, 0.45, z + 0.42),
            7,
        );
    }
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
    let detail = 1;
    add_cube(
        scene,
        base + Vec3::new(-1.85, -0.08, -1.35),
        base + Vec3::new(1.85, 0.20, 1.35),
        detail,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.45, 0.20, -1.05),
        base + Vec3::new(1.45, 0.52, 1.05),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.15, 0.40, -0.85),
        base + Vec3::new(1.15, 1.55, 0.85),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.78, 1.55, -0.52),
        base + Vec3::new(0.78, 4.45, 0.52),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.15, 3.85, -0.72),
        base + Vec3::new(1.15, 5.25, 0.72),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.02, 5.25, -0.78),
        base + Vec3::new(1.02, 6.25, 0.78),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.62, 6.25, -0.50),
        base + Vec3::new(0.62, 7.25, 0.50),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-1.65, 4.20, -0.55),
        base + Vec3::new(-1.00, 5.15, 0.55),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(1.00, 4.20, -0.55),
        base + Vec3::new(1.65, 5.15, 0.55),
        stone,
    );
    let face_x = facing * 0.66;
    add_cube(
        scene,
        base + Vec3::new(face_x - 0.08, 6.68, -0.30),
        base + Vec3::new(face_x + 0.08, 6.83, -0.10),
        detail,
    );
    add_cube(
        scene,
        base + Vec3::new(face_x - 0.08, 6.68, 0.10),
        base + Vec3::new(face_x + 0.08, 6.83, 0.30),
        detail,
    );
    add_cube(
        scene,
        base + Vec3::new(face_x - 0.10, 6.38, -0.08),
        base + Vec3::new(face_x + 0.10, 6.58, 0.08),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(face_x - 0.09, 7.15, -0.42),
        base + Vec3::new(face_x + 0.09, 7.32, 0.42),
        detail,
    );
    add_cube(
        scene,
        base + Vec3::new(-2.05, 3.25, -0.38),
        base + Vec3::new(-0.80, 4.05, 0.38),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(0.80, 3.25, -0.38),
        base + Vec3::new(2.05, 4.05, 0.38),
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
        base + Vec3::new(-0.78, 7.25, -0.56),
        base + Vec3::new(0.78, 7.95, 0.56),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.42, 7.95, -0.34),
        base + Vec3::new(0.42, 8.45, 0.34),
        stone,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.22, 8.45, -0.20),
        base + Vec3::new(0.22, 8.85, 0.20),
        detail,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.95, 7.75, -0.70),
        base + Vec3::new(0.95, 8.05, -0.48),
        detail,
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

fn add_moss_patch(scene: &mut Scene, base: Vec3) {
    add_cube(
        scene,
        base + Vec3::new(-0.65, 0.0, -0.28),
        base + Vec3::new(0.65, 0.08, 0.28),
        8,
    );
}

fn add_rock_cluster(scene: &mut Scene, base: Vec3) {
    add_cube(
        scene,
        base + Vec3::new(-0.45, 0.0, -0.30),
        base + Vec3::new(0.20, 0.38, 0.25),
        1,
    );
    add_cube(
        scene,
        base + Vec3::new(0.18, 0.0, -0.18),
        base + Vec3::new(0.60, 0.28, 0.30),
        1,
    );
    add_cube(
        scene,
        base + Vec3::new(-0.18, 0.28, -0.12),
        base + Vec3::new(0.22, 0.58, 0.18),
        0,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.0001,
            "expected {expected}, got {actual}"
        );
    }

    fn test_cube() -> Cube {
        Cube {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
            material: 0,
        }
    }

    #[test]
    fn reflection_preserves_the_incident_angle() {
        let reflected = Vec3::new(1.0, -1.0, 0.0).reflect(Vec3::new(0.0, 1.0, 0.0));

        assert_close(reflected.x, 1.0);
        assert_close(reflected.y, 1.0);
        assert_close(reflected.z, 0.0);
    }

    #[test]
    fn refraction_keeps_a_perpendicular_ray_straight() {
        let refracted = Vec3::new(0.0, -1.0, 0.0)
            .refract(Vec3::new(0.0, 1.0, 0.0), 1.0 / 1.33)
            .expect("a perpendicular ray should refract");

        assert_close(refracted.x, 0.0);
        assert_close(refracted.y, -1.0);
        assert_close(refracted.z, 0.0);
    }

    #[test]
    fn refraction_detects_total_internal_reflection() {
        let direction = Vec3::new(0.9, -0.435_889_9, 0.0).normalized();

        assert!(direction.refract(Vec3::new(0.0, 1.0, 0.0), 1.5).is_none());
    }

    #[test]
    fn cube_intersection_returns_distance_and_face_normal() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, -3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        };
        let (distance, normal) = intersect_cube(ray, test_cube()).expect("ray should hit cube");

        assert_close(distance, 2.0);
        assert_close(normal.z, -1.0);
    }

    #[test]
    fn cube_intersection_uses_exit_face_when_ray_starts_inside() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        };
        let (distance, normal) = intersect_cube(ray, test_cube()).expect("ray should exit cube");

        assert_close(distance, 1.0);
        assert_close(normal.x, 1.0);
    }

    #[test]
    fn center_camera_ray_points_at_target() {
        let origin = Vec3::new(0.0, 2.0, -5.0);
        let target = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera::look_at(origin, target, 46.0, 16.0 / 9.0);
        let expected = (target - origin).normalized();
        let ray = camera.ray(0.5, 0.5);

        assert_close(ray.direction.x, expected.x);
        assert_close(ray.direction.y, expected.y);
        assert_close(ray.direction.z, expected.z);
    }

    #[test]
    fn scene_contains_required_raytracing_materials() {
        let scene = build_scene();

        assert!(scene.materials.len() >= 5);
        assert!(scene.materials.iter().any(|m| m.reflectivity > 0.0));
        assert!(scene.materials.iter().any(|m| m.transparency > 0.0));
        assert!(scene.materials.iter().any(|m| m.refractive_index > 1.0));
        assert!(!scene.cubes.is_empty());
    }
}
