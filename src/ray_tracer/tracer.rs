use serde::Deserialize;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::canvas::{Canvas, Color};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn dot(&self, rhs: &Point) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
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

impl Sub<&Point> for f64 {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<&f64> for &Point {
    type Output = Point;

    fn div(self, rhs: &f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum Light {
    Ambient(f64),
    Point(f64, Point),
    Directional(f64, Point),
}

pub fn render(canvas: &mut Canvas, viewport: &Viewport, spheres: Vec<Sphere>) {
    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let scene = Scene {
        spheres,
        lights: vec![
            Light::Ambient(0.2),
            Light::Point(0.6, Point::new(2.0, 1.0, 0.0)),
            Light::Directional(0.2, Point::new(1.0, 4.0, 4.0)),
        ],
    };

    let half_x = canvas.width / 2;
    let half_y = canvas.height / 2;

    for x in -half_x..half_x {
        for y in -half_y..half_y {
            let d = viewport.canvas_to_viewport(canvas, x, y);
            let color = scene.trace_ray(&origin, &d, 1.0, f64::MAX, 3);
            canvas.put_pixel(x, y, &color);
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
}

impl Viewport {
    fn canvas_to_viewport(&self, canvas: &Canvas, x: i32, y: i32) -> Point {
        Point {
            x: x as f64 * self.width / canvas.width as f64,
            y: -y as f64 * self.height / canvas.height as f64,
            z: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Sphere {
    pub radius: f64,
    pub center: Point,
    pub color: Color,
    pub specular: f64,
    pub reflective: f64,
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
    lights: Vec<Light>,
}

impl Scene {
    fn trace_ray(
        &self,
        origin: &Point,
        direction: &Point,
        t_min: f64,
        t_max: f64,
        recursive_depth: usize,
    ) -> Color {
        let intersection = self.closest_intersection(origin, direction, t_min, t_max);
        match intersection {
            Some((sphere, closest_t)) => {
                let p = origin + &(direction * closest_t);
                let n = &p - &sphere.center;
                let n = &n / &n.length();
                let local_color =
                    &sphere.color * self.compute_lighting(&p, &n, &(-direction), sphere.specular);

                let r = sphere.reflective;
                if recursive_depth <= 0 || r <= 0.0 {
                    local_color
                } else {
                    let redirected = reflect_ray(&(-direction), &n);
                    let reflected_color =
                        self.trace_ray(&p, &redirected, 0.001, f64::MAX, recursive_depth - 1);
                    local_color * (1.0 - r) + reflected_color * r
                }
            }
            None => Color::black(),
        }
    }

    fn closest_intersection(
        &self,
        origin: &Point,
        direction: &Point,
        t_min: f64,
        t_max: f64,
    ) -> Option<(&Sphere, f64)> {
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
            Some(sphere) => Some((sphere, closest_t)),
            None => None,
        }
    }

    fn compute_lighting(&self, point: &Point, normal: &Point, v: &Point, s: f64) -> f64 {
        let mut i = 0.0;

        for light in &self.lights {
            match light {
                Light::Ambient(intensity) => i += intensity,
                Light::Point(intensity, position) => {
                    i += self.calc_light(point, normal, &(position - point), intensity, v, s, 1.0)
                }
                Light::Directional(intensity, direction) => {
                    i += self.calc_light(point, normal, direction, intensity, v, s, f64::MAX)
                }
            }
        }

        i
    }

    fn calc_light(
        &self,
        p: &Point,
        n: &Point,
        l: &Point,
        intensity: &f64,
        v: &Point,
        s: f64,
        t_max: f64,
    ) -> f64 {
        let intersection = self.closest_intersection(p, l, 0.001, t_max);
        if intersection.is_some() {
            return 0.0;
        }
        let mut i = 0.0;
        let n_dot_l = n.dot(l);
        i += if n_dot_l > 0.0 {
            intensity * n_dot_l / (n.length() * l.length())
        } else {
            0.0
        };

        if s != -1.0 {
            let r = reflect_ray(l, n);
            let r_dot_v = r.dot(v);
            if r_dot_v > 0.0 {
                i += intensity * (r_dot_v / (r.length() * v.length())).powf(s);
            }
        }

        i
    }
}

fn reflect_ray(r: &Point, n: &Point) -> Point {
    &(&(2.0 * n) * n.dot(r)) - r
}
