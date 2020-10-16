use rand::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::quadtree::Quadtree;
use crate::quadtree::Rectangle as Rect;
use crate::rendering::Rectangle;
use crate::rendering::Triangle;

#[derive(Debug)]
pub struct Boid {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub acceleration: (f32, f32),
    pub alignment_force: f32,
    pub cohesion_force: f32,
    pub seperation_force: f32,
    pub perception_size: f32,
    pub max_speed: f32,
}

pub struct Flock {
    dimensions: (u32, u32),
    boids: Vec<Boid>,
    triangle: Triangle,
    quadtree: Quadtree,
    line: Rectangle,
}

impl Flock {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let boidshape = [0.0, 1.0, 0.34, -1.0, -0.34, -1.0];
        let mut rng = rand::thread_rng();
        let mut boids = Vec::<Boid>::new();
        let mut qt = Quadtree::new(
            2,
            Rect {
                x: -1.0,
                y: 1.0,
                width: 2.0,
                height: 2.0,
            },
        );

        for index in 0..100 {
            boids.push(Boid {
                position: (
                    (rng.gen::<f32>() * 2.0) - 1.0,
                    (rng.gen::<f32>() * 2.0) - 1.0,
                ),
                velocity: (
                    (rng.gen::<f32>() * 2.0) - 1.0,
                    (rng.gen::<f32>() * 2.0) - 1.0,
                ),
                acceleration: (0.0, 0.0),
                alignment_force: 0.2,
                cohesion_force: 0.2,
                seperation_force: 0.6,
                perception_size: 100.0,
                max_speed: 7.0 / 2.0,
            });

            let _ = qt.insert(boids[index].position.0, boids[index].position.1, index);
        }

        Self {
            dimensions: (width, height),
            triangle: Triangle::new(&gl, boidshape),
            boids,
            quadtree: qt,
            line: Rectangle::new(&gl),
        }
    }

    fn wrappedDistance(&self, vec1: (f32, f32), vec2: (f32, f32)) -> f32 {
        let mut dx = (vec1.0 - vec2.0).abs();
        let mut dy = (vec1.1 - vec2.1).abs();

        if dx > 1.0 {
            dx -= 2.0;
        }
        if dy > 1.0 {
            dy -= 2.0;
        }

        return (dx.powi(2) + dy.powi(2)).sqrt();
    }

    fn getLocalBoids(&self, circle: (f32, f32, f32)) {
        let mut boid_indexs: Vec<usize> = Vec::new();

        if circle.0 + circle.2 > 2.0 {
            boid_indexs.extend(self.quadtree.query(circle));
        }
    }

    fn distance(vec1: (f32, f32), vec2: (f32, f32)) -> f32 {
        return ((vec2.0 - vec1.0).powi(2) + (vec2.1 - vec1.1).powi(2)).sqrt();
    }

    pub fn update(&self) {}

    pub fn render(&self, gl: &GL) {
        let selected = self.quadtree.query((0.0, 0.0, 1.0));
        let mut color = [1.0, 1.0, 1.0, 1.0];
        for (index, boid) in self.boids.iter().enumerate() {
            if selected.iter().any(|&i| i == index) {
                color = [0.0, 1.0, 0.0, 1.0];
            } else {
                color = [1.0, 1.0, 1.0, 1.0]
            }
            self.triangle
                .render(&gl, boid.position.0, boid.position.1, 0.05, 0.05, color)
        }
        self.quadtree.renderroot(&gl, &self.line);
    }
}
