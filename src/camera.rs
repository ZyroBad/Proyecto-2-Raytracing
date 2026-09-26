use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn look_at(origin: Vec3, target: Vec3, fov_deg: f32, aspect: f32) -> Self {
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

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left + self.horizontal * u + self.vertical * v - self.origin)
                .normalized(),
        }
    }
}
