use crate::drawing_object::DrawingObject;
use crate::vector::Vector;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Section {
    pub start: Point,
    pub end: Point,
}

impl Section {
    pub fn new(point1: Point, point2: Point) -> Section {
        Section {
            start: point1,
            end: point2,
        }
    }

    pub fn move_to(&self, vector: &Vector) -> Section {
        let point1 = Point {
            x: self.start.x + vector.x,
            y: self.start.y + vector.y,
        };
        let point2 = Point {
            x: self.end.x + vector.x,
            y: self.end.y + vector.y,
        };
        Section::new(point1, point2)
    }

    pub fn from_object(object: &DrawingObject) -> Vec<Section> {
        let mut walls: Vec<Section> = Vec::new();

        if object.object_type == 0 {
            walls.push(Section {
                start: object.points[0],
                end: object.points[1],
            });
        } else if object.object_type == 1 {
            let mut rect_walls: Vec<Section> = Vec::with_capacity(4);
            rect_walls.push(Section {
                start: object.points[0],
                end: Point::new(object.points[1].y, object.points[0].y),
            });
            rect_walls.push(Section {
                start: Point::new(object.points[1].x, object.points[0].y),
                end: Point::new(object.points[1].x, object.points[1].y),
            });
            rect_walls.push(Section {
                start: Point::new(object.points[1].x, object.points[1].y),
                end: Point::new(object.points[0].x, object.points[1].y),
            });
            rect_walls.push(Section {
                start: Point::new(object.points[0].x, object.points[1].y),
                end: object.points[0],
            });
            for rect_wall in rect_walls {
                walls.push(rect_wall);
            }
        }

        walls
    }
}

pub type Polygon = Vec<Point>;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn to_i32(&self) -> Point32 {
        Point32 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Point32 {
    pub x: i32,
    pub y: i32,
}

impl Point32 {
    pub fn new(x: i32, y: i32) -> Point32 {
        Point32 { x, y }
    }
}
