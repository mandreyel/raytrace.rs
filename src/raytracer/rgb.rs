use raytracer::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    pub fn from(v: Vec3) -> Rgb {
        Rgb {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }

    pub fn gamma_correct(&self) -> Rgb {
        Rgb {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
}
