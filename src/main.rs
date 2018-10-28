extern crate rand;

use std::io::Write;
use std::fs::OpenOptions;
use rand::Rng;

mod raytracer;
use raytracer::{Vec3, dot, Hit, Ray, Rgb, Sphere};

fn main() {
    let path = "/tmp/raytracer.ppm";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create/*_new*/(true)
        .open(&path);
    let mut file = match file {
        Ok(file) => file,
        Err(msg) => panic!("Could not open file: {}", msg),
    };

    let nx = 200;
    let ny = 100;
    let cam = Camera::axis_aligned();
    let mut rng = rand::thread_rng();

    write!(file, "P3\n");
    write!(file, "{} {}\n", nx, ny);
    write!(file, "255\n");

    let hitables: Vec<Box<Hit>> = vec![
        Box::new(Sphere {
            center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
            radius: 100.0,
        }),
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            // Anti-aliasing.
            let mut col = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..100 {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = cam.ray(u, v);
                let p = ray.point_at(2.0);
                col += Vec3::from(color(&ray, &hitables));
            }

            col /= 100.0;

            let col = Rgb {
                r: 255.99 * col.x,
                g: 255.99 * col.y,
                b: 255.99 * col.z,
            };

            write!(file, "{} {} {}\n", col.r as i32, col.g as i32, col.b as i32);
        }
    }
}

struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    fn axis_aligned() -> Camera {
        Camera {
            lower_left_corner: Vec3 { x: -2.0, y: -1.0, z: -1.0 },
            horizontal: Vec3 { x: 4.0, y: 0.0, z: 0.0 },
            vertical: Vec3 { x: 0.0, y: 2.0, z: 0.0 },
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner
                + u * self.horizontal
                + v * self.vertical
                - self.origin,
        }
    }
}

fn color<T: Hit>(ray: &Ray, world: &T) -> Rgb {
    // See if the ray hits the world, otherwise paint the background.
    if let Some(hit) = world.hit(&ray, 0.0, std::f32::MAX) {
        let normal = 0.5 * Vec3 {
            x: hit.normal.x + 1.0,
            y: hit.normal.y + 1.0,
            z: hit.normal.z + 1.0
        };
        Rgb::from(normal)
    } else {
        // Get unit vector so -1 < y < 1.
        let unit_dir = ray.direction.to_unit();
        // Scale that value to 0 < y < 1.
        let t = 0.5 * (unit_dir.y + 1.0);
        // Linear interpolation: blended_val = (1 - t) * start_val + t * end_val.
        let start_val = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        let end_val = Vec3 { x: 0.5, y: 0.7, z: 1.0 };
        let ler = (1.0 - t) * start_val + t * end_val;
        Rgb::from(ler)
    }
}
