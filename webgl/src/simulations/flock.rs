use rand::prelude::*;
use web_sys::WebGlRenderingContext as GL;

use crate::{
    quadtree::Quadtree, quadtree::Rectangle as Rect, rendering::Rectangle, rendering::Triangle,
    utils::ScreenSpaceEncoder,
};

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

impl Boid {
    pub fn update(&mut self, width: i32, height: i32) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        self.velocity = (
            self.velocity.0 + self.acceleration.0,
            self.velocity.1 + self.acceleration.1,
        );

        self.edges(width, height);
    }

    //instead of boids draining off the edges wrap space into a torus lmao
    fn edges(&mut self, width: i32, height: i32) {
        if self.position.0 > (width + 11) as f32 {
            self.position.0 = -11.0;
        } else if self.position.0 < -11.0 {
            self.position.0 = (width + 11) as f32;
        } // add some arbitrary amount so the triangles are popping in and out of existancel

        if self.position.1 > (height + 11) as f32 {
            self.position.1 = -11.0;
        } else if self.position.1 < -11.0 {
            self.position.1 = (height + 11) as f32;
        }
    }
}

pub struct Flock {
    dimensions: (u32, u32),
    aspect: f32,
    boids: Vec<Boid>,
    triangle: Triangle,
    quadtree: Quadtree,
    line: Rectangle,
    encoder: ScreenSpaceEncoder,
    count: u32,
}

impl Flock {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let encoder = ScreenSpaceEncoder {
            dimensions: (width, height),
        };

        let boidshape = [0.0, 0.5, 0.34, -0.5, -0.34, -0.5];
        let mut rng = rand::thread_rng();
        let mut boids = Vec::<Boid>::new();
        let mut qt = Quadtree::new(
            2,
            Rect {
                x: 0.0,
                y: 0.0,
                width: width as f32,
                height: height as f32,
            },
        );

        for index in 0..1000 {
            boids.push(Boid {
                position: (
                    (rng.gen::<f32>() * encoder.dimensions.0 as f32),
                    (rng.gen::<f32>() * encoder.dimensions.1 as f32),
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
            aspect: width as f32 / height as f32,
            triangle: Triangle::new(&gl, boidshape),
            boids,
            quadtree: qt,
            line: Rectangle::new(&gl),
            encoder,
            count: 0,
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

    pub fn update(&mut self, width: i32, height: i32) {
        self.encoder.updateDimensions(width as u32, height as u32);
        self.count = (self.count + 1) % 101;
        self.aspect = width as f32 / height as f32;
        self.dimensions = (width as u32, height as u32);
        self.quadtree.reset();

        for (pos, boid) in self.boids.iter_mut().enumerate() {
            boid.update(width, height);

            self.quadtree.insert(boid.position.0, boid.position.1, pos);
        }
    }

    pub fn render(&self, gl: &GL) {
        let selected = self.quadtree.query((
            self.dimensions.0 as f32 / 2.0,
            self.dimensions.1 as f32 / 2.0,
            100.0,
        ));
        self.quadtree.renderroot(&gl, &self.line, self.encoder);

        let mut color = [1.0, 1.0, 1.0, 1.0];
        for (index, boid) in self.boids.iter().enumerate() {
            let ang = boid.velocity.1.atan2(boid.velocity.0);

            if selected.iter().any(|&i| i == index) {
                color = [0.0, 1.0, 0.0, 1.0];
            } else {
                color = [0.37, 0.22, 0.40, 1.0]
            }
            let test = self.encoder.encode(boid.position.0, boid.position.1);
            self.triangle.render(
                &gl,
                test.0,
                test.1,
                0.05,
                0.05, //* self.aspect,
                ang - std::f32::consts::FRAC_PI_2,
                color,
            );
        }
    }
}
