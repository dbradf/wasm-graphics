use serde::Deserialize;
use crate::canvas::Canvas;
use super::{color::Color, tuple::Tuple};


#[derive(Clone, Debug, Deserialize)]
pub enum Light {
    Ambient(f64),
    Point(f64, Tuple),
    Directional(f64, Tuple),
}

pub fn render(canvas: &mut Canvas, viewport: &Viewport, spheres: Vec<Sphere>) {
    let origin = Tuple::point(0.0, 0.0, 0.0);

    let scene = Scene {
        spheres,
        lights: vec![
            Light::Ambient(0.2),
            Light::Point(0.6, Tuple::point(2.0, 1.0, 0.0)),
            Light::Directional(0.2, Tuple::point(1.0, 4.0, 4.0)),
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
    fn canvas_to_viewport(&self, canvas: &Canvas, x: i32, y: i32) -> Tuple {
        Tuple::vector(
            x as f64 * self.width / canvas.width as f64,
            -y as f64 * self.height / canvas.height as f64,
            1.0,
        )
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Sphere {
    pub radius: f64,
    pub center: Tuple,
    pub color: Color,
    pub specular: f64,
    pub reflective: f64,
}

impl Sphere {
    pub fn intersect(&self, origin: &Tuple, direction: &Tuple) -> (f64, f64) {
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
        origin: &Tuple,
        direction: &Tuple,
        t_min: f64,
        t_max: f64,
        recursive_depth: usize,
    ) -> Color {
        let intersection = self.closest_intersection(origin, direction, t_min, t_max);
        match intersection {
            Some((sphere, closest_t)) => {
                let p = origin + (direction * closest_t);
                let n = p - sphere.center;
                let n = n / n.magnitude();
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
        origin: &Tuple,
        direction: &Tuple,
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

    fn compute_lighting(&self, point: &Tuple, normal: &Tuple, v: &Tuple, s: f64) -> f64 {
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
        p: &Tuple,
        n: &Tuple,
        l: &Tuple,
        intensity: &f64,
        v: &Tuple,
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
            intensity * n_dot_l / (n.magnitude() * l.magnitude())
        } else {
            0.0
        };

        if s != -1.0 {
            let r = reflect_ray(l, n);
            let r_dot_v = r.dot(v);
            if r_dot_v > 0.0 {
                i += intensity * (r_dot_v / (r.magnitude() * v.magnitude())).powf(s);
            }
        }

        i
    }
}

fn reflect_ray(r: &Tuple, n: &Tuple) -> Tuple {
    ((n * 2.0) * n.dot(r)) - r
}
