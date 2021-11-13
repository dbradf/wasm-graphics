use std::ops::{Add, Div, Mul, Sub};

use crate::canvas::{Canvas, Color};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
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

impl Mul<&f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: &f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
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

#[derive(Clone, Debug)]
enum Light {
    Ambient(f64),
    Point(f64, Point),
    Directional(f64, Point),
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
                center: Point {
                    x: 0.0,
                    y: -1.0,
                    z: 3.0,
                },
                color: Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
            },
            Sphere {
                radius: 1.0,
                center: Point {
                    x: 2.0,
                    y: 0.0,
                    z: 4.0,
                },
                color: Color {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
            },
            Sphere {
                radius: 1.0,
                center: Point {
                    x: -2.0,
                    y: 0.0,
                    z: 4.0,
                },
                color: Color {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255,
                },
            },
            Sphere {
                radius: 5000.0,
                center: Point::new(0.0, -5001.0, 0.0),
                color: Color::new(255, 255, 0),
            },
        ],
        lights: vec![
            Light::Ambient(0.2),
            Light::Point(0.6, Point::new(2.0, 1.0, 0.0)),
            Light::Directional(0.2, Point::new(1.0, 4.0, 4.0)),
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
            y: -y as f64 * self.height / canvas.height as f64,
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
    lights: Vec<Light>,
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
            Some(sphere) => {
                let p = origin + &(direction * &closest_t);
                let n = &p - &sphere.center;
                let n = &n / &n.length();
                &sphere.color * self.compete_lighting(&p, &n)
            }
            None => Color::white(),
        }
    }

    fn compete_lighting(&self, point: &Point, normal: &Point) -> f64 {
        let mut i = 0.0;

        for light in &self.lights {
            match light {
                Light::Ambient(intensity) => i += intensity,
                Light::Point(intensity, position) => {
                    i += calc_light(normal, &(position - point), intensity)
                }
                Light::Directional(intensity, direction) => {
                    i += calc_light(normal, direction, intensity)
                }
            }
        }

        i
    }
}

fn calc_light(n: &Point, l: &Point, intensity: &f64) -> f64 {
    let n_dot_l = n.dot(l);
    if n_dot_l > 0.0 {
        intensity * n_dot_l / (n.length() * l.length())
    } else {
        0.0
    }
}
