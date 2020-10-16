use crate::rendering::Rectangle as drawableRect;

use crate::simulations::Boid;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn contains(&self, point: (f32, f32)) -> bool {
        point.0 >= self.x
            && self.x + self.width > point.0
            && point.1 <= self.y
            && self.y - self.height < point.1
    }

    pub fn intersectCircle(&self, circle: (f32, f32, f32)) -> bool {
        //x,y,radius
        let x = (circle.0 - self.x).abs();
        let y = (circle.1 - self.y).abs();

        if x > self.width / 2.0 + circle.2 {
            return false;
        };
        if y > self.height / 2.0 + circle.2 {
            return false;
        };

        if x <= self.width / 2.0 {
            return true;
        };
        if y <= self.height / 2.0 {
            return true;
        };

        let corner_dist = (x - self.width / 2.0).powi(2) + (y - self.height / 2.0).powi(2);

        return corner_dist <= circle.2.powi(2);
    }
}
pub struct Quadtree {
    rectangle: Rectangle,
    capacity: i16,
    points: Option<Vec<(f32, f32, usize)>>,
    divided: bool,
    nw: Option<Box<Quadtree>>, //boxes?
    sw: Option<Box<Quadtree>>,
    ne: Option<Box<Quadtree>>,
    se: Option<Box<Quadtree>>,
}

impl Quadtree {
    pub fn new(capacity: i16, rectangle: Rectangle) -> Self {
        Self {
            rectangle,
            capacity,
            nw: None,
            sw: None,
            ne: None,
            se: None,
            divided: false,
            points: None::<Vec<(f32, f32, usize)>>,
        }
    }

    pub fn reset(&mut self) {
        self.divided = false;
        self.nw = None;
        self.sw = None;
        self.ne = None;
        self.se = None;
    }

    fn subdivide(&mut self) {
        self.nw = Some(Box::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x,
                y: self.rectangle.y,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.sw = Some(Box::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x,
                y: self.rectangle.y - self.rectangle.height / 2.0,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.ne = Some(Box::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x + self.rectangle.width / 2.0,
                y: self.rectangle.y,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.se = Some(Box::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x + self.rectangle.width / 2.0,
                y: self.rectangle.y - self.rectangle.height / 2.0,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        // offload points from this qt to the children and turn points to none
        for point in self.points.as_ref().unwrap() {
            self.ne.as_mut().unwrap().insert(point.0, point.1, point.2);
            self.se.as_mut().unwrap().insert(point.0, point.1, point.2);
            self.nw.as_mut().unwrap().insert(point.0, point.1, point.2);
            self.sw.as_mut().unwrap().insert(point.0, point.1, point.2);
        }

        self.points = None;

        self.divided = true;
    }

    pub fn insert(&mut self, x: f32, y: f32, boid_index: usize) -> bool {
        if !self.rectangle.contains((x, y)) {
            false
        } else if self.divided {
            if self.nw.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else if self.ne.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else if self.sw.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else if self.se.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else {
                false
            }
        } else if self.points.is_none() {
            //if self.points.as_ref().unwrap().len() < self.capacity as usize {
            self.points = Some(Vec::new());
            let _ = self.points.as_mut().unwrap().push((x, y, boid_index));

            true
        } else if self.points.as_ref().unwrap().len() < self.capacity as usize {
            let _ = self.points.as_mut().unwrap().push((x, y, boid_index));

            true
        } else {
            if !self.divided {
                self.subdivide();
            }
            if self.nw.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else if self.ne.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else if self.sw.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else if self.se.as_mut().unwrap().insert(x, y, boid_index) {
                true
            } else {
                false
            }
        }
    }

    pub fn query(&self, circle: (f32, f32, f32)) -> Vec<usize> {
        let mut found: Vec<usize> = Vec::new();
        if self.divided {
            found.extend(self.ne.as_ref().unwrap().query(circle));
            found.extend(self.nw.as_ref().unwrap().query(circle));
            found.extend(self.se.as_ref().unwrap().query(circle));
            found.extend(self.sw.as_ref().unwrap().query(circle));
        }

        if self.points.is_none() || !self.rectangle.intersectCircle(circle) {
            return found;
        } else {
            for point in self.points.as_ref().unwrap() {
                if ((point.0 - circle.0).powi(2) + (point.1 - circle.1).powi(2)).sqrt() <= circle.2
                {
                    found.push(point.2);
                }
            }
        }
        return found;
    }

    pub fn renderroot(&self, gl: &GL, line: &drawableRect) {
        let color = [0.5, 0.5, 0.5, 1.0];

        line.render(
            &gl,
            self.rectangle.x,
            self.rectangle.y,
            self.rectangle.width,
            -0.007,
            color,
        );

        line.render(
            &gl,
            self.rectangle.x,
            self.rectangle.y - self.rectangle.width,
            0.004,
            self.rectangle.height,
            color,
        );

        line.render(
            &gl,
            self.rectangle.x,
            self.rectangle.y - self.rectangle.width,
            self.rectangle.width,
            0.007,
            color,
        );

        line.render(
            &gl,
            self.rectangle.x + self.rectangle.width,
            self.rectangle.y - self.rectangle.height,
            -0.004,
            self.rectangle.height,
            color,
        );

        if self.divided {
            self.nw
                .as_ref()
                .unwrap()
                .renderchild(&gl, (true, true), &line);
            self.ne
                .as_ref()
                .unwrap()
                .renderchild(&gl, (true, false), &line);
            self.se
                .as_ref()
                .unwrap()
                .renderchild(&gl, (false, false), &line);
            self.sw
                .as_ref()
                .unwrap()
                .renderchild(&gl, (false, true), &line);
        }
    }

    pub fn renderchild(&self, gl: &GL, dir: (bool, bool), line: &drawableRect) {
        let color = [0.5, 0.5, 0.5, 1.0];

        if dir.0 && dir.1 {
            //nw
            line.render(
                &gl,
                self.rectangle.x,
                self.rectangle.y - self.rectangle.height,
                self.rectangle.width,
                0.004,
                color,
            );
            line.render(
                &gl,
                self.rectangle.x + self.rectangle.width + 0.002, //0.002 is half the width of a line
                self.rectangle.y - self.rectangle.height,
                0.004,
                self.rectangle.height,
                color,
            )
        } else if dir.0 && !dir.1 {
            //ne
            line.render(
                &gl,
                self.rectangle.x,
                self.rectangle.y - self.rectangle.height,
                self.rectangle.width,
                0.004,
                color,
            )
        } else if !dir.0 && dir.1 {
            //se
            line.render(
                &gl,
                self.rectangle.x + self.rectangle.width + 0.002,
                self.rectangle.y - self.rectangle.height,
                0.004,
                self.rectangle.height,
                color,
            )
        }

        if self.divided {
            self.nw
                .as_ref()
                .unwrap()
                .renderchild(&gl, (true, true), &line);
            self.ne
                .as_ref()
                .unwrap()
                .renderchild(&gl, (true, false), &line);
            self.se
                .as_ref()
                .unwrap()
                .renderchild(&gl, (false, false), &line);
            self.sw
                .as_ref()
                .unwrap()
                .renderchild(&gl, (false, true), &line);
        }
    }
}
