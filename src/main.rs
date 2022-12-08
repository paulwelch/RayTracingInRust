mod ray;

use std::io::{self,Write};
use nalgebra::Vector3;

use crate::ray::Ray;

fn main() -> io::Result<()> {

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = (image_width / aspect_ratio) as f64;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0, 0.0, focal_length);


    //Render
    let mut data = "P3\n".to_string();
    data = data + &image_width.to_string() + &" " + &image_height.to_string() + &"\n255\n";
    handle.write(data.as_bytes())?;

    for j in (0..(image_height as i32)).rev() {
        for i in 0..(image_width as i32) {
            let u = i as f64 / (image_width as i32 - 1) as f64;
            let v = j as f64 / (image_height as i32 - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color = ray_color(&r);
            write_color(&stdout, color)?;
        }
    }

    handle.flush()?;
    Ok(())
}

struct PixelColor {
   r: f64,
   g: f64,
   b: f64,
}

impl PixelColor {
    pub fn new(v: Vector3<f64>) -> Self {
        PixelColor { r: v[0] as f64, g: v[1] as f64, b: v[2] as f64  }
    }
}

fn write_color(stdout: &std::io::Stdout, color: PixelColor) -> Result<(), std::io::Error>  {
    // Write the translated [0,255] value of each color component.
    let mut handle = stdout.lock();
    let data = (((color.r * 255.999) as i32).to_string()) + &" " + &(((color.g * 255.999) as i32).to_string()) + &" " + &(((color.b * 255.999) as i32).to_string()) + &"\n";
    handle.write(data.as_bytes())?;
    Ok(())
}

fn ray_color(ray: &Ray) -> PixelColor {
    let mut t = hit_sphere(Vector3::new(0.0,0.0,-1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vector3::new(0.0,0.0,-1.0)).normalize() as Vector3<f64>;
        return PixelColor::new(0.5 * Vector3::new(n.x+1.0, n.y+1.0, n.z+1.0));
    }
    let unit_direction = ray.direction().normalize();
    t = 0.5 * (unit_direction[1] + 1.0);
    PixelColor::new( (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0) )
}

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-b - discriminant.sqrt()) / (2.0*a)
    }
}