use crate::simulations::Boid;
use std::rc::Rc;

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
            && point.1 < self.y
            && self.y - self.height < point.1
    }
}

pub struct Quadtree {
    rectangle: Rectangle,
    capacity: i16,
    points: Option<Vec<(f32, f32, &'static Boid)>>,
    divided: bool,
    nw: Option<Rc<Quadtree>>,
    sw: Option<Rc<Quadtree>>,
    ne: Option<Rc<Quadtree>>,
    se: Option<Rc<Quadtree>>,
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
            points: None::<Vec<(f32, f32, &Boid)>>,
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
        self.nw = Some(Rc::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x,
                y: self.rectangle.y,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.sw = Some(Rc::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x,
                y: self.rectangle.y - self.rectangle.height / 2.0,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.ne = Some(Rc::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x + self.rectangle.width / 2.0,
                y: self.rectangle.y,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.se = Some(Rc::new(Quadtree::new(
            self.capacity,
            Rectangle {
                x: self.rectangle.x + self.rectangle.width / 2.0,
                y: self.rectangle.y - self.rectangle.height / 2.0,
                width: self.rectangle.width / 2.0,
                height: self.rectangle.height / 2.0,
            },
        )));

        self.divided = true;
    }

    pub fn insert() -> bool {}
}
