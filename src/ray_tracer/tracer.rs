use std::ops::Sub;

use crate::canvas::{Canvas, Color};


#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn dot(&self, rhs: &Point) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub fn render(canvas: &mut Canvas) {
    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let scene = Scene {
        spheres: vec![
            Sphere { 
                radius: 1.0, 
                center: Point {x: 0.0, y: -1.0, z: 3.0},
                color: Color {r: 255, g: 0, b: 0, a: 255},
            },
            Sphere { 
                radius: 1.0, 
                center: Point {x: 2.0, y: 0.0, z: 4.0},
                color: Color {r: 0, g: 0, b: 255, a: 255},
            },
            Sphere { 
                radius: 1.0, 
                center: Point {x: -2.0, y: 0.0, z: 4.0},
                color: Color {r: 0, g: 255, b: 0, a: 255},
            },
        ],
    };

    let viewport = Viewport {
        width: 1.0,
        height: 1.0,
    };

    let half_x = canvas.width / 2;
    let half_y = canvas.height / 2;

    for x in -half_x..half_x {
        for y in -half_y..half_y {
            let d = viewport.canvas_to_viewport(canvas, x, y);
            let color = scene.trace_ray(&origin, &d, 1.0, f64::MAX);
            canvas.put_pixel(x, y, &color);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Viewport {
    width: f64,
    height: f64,
}

impl Viewport {
    fn canvas_to_viewport(&self, canvas: &Canvas, x: i32, y: i32) -> Point {
        Point {
            x: x as f64 * self.width / canvas.width as f64,
            y: y as f64 * self.height / canvas.height as f64, 
            z: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    radius: f64,
    center: Point,
    color: Color,
}

impl Sphere {
    pub fn intersect(&self, origin: &Point, direction: &Point) -> (f64, f64) {
        let r = self.radius;
        let co = origin - &self.center;

        let a = direction.dot(&direction);
        let b = 2.0 * co.dot(&direction);
        let c = co.dot(&co) - r * r;

        let discriminant = (b * b - 4.0 * a * c) as f64;
        if discriminant < 0.0 {
            (f64::MAX, f64::MAX)
        } else {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
            (t1, t2)
        }
    }
}



#[derive(Debug, Clone)]
struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    fn trace_ray(&self, origin: &Point, direction: &Point, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = f64::MAX;
        let mut closest_sphere = None;

        for sphere in &self.spheres {
            let (t1, t2) = sphere.intersect(origin, direction);
            if (t1 >= t_min && t1 <= t_max) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if (t2 >= t_min && t2 <= t_max) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }

        match closest_sphere {
            Some(sphere) => sphere.color,
            None => Color::white(),
        }
    }
}