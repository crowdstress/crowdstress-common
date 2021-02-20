use crate::primitives::{Point, Polygon, Section};
use crate::vector::Vector;

pub fn get_section_middle(section: &Section) -> Point {
    Point {
        x: (section.start.x + section.end.x) / 2.0,
        y: (section.start.y + section.end.y) / 2.0,
    }
}

pub fn get_vector_to_line(line: &Section, point: &Point) -> Vector {
    let n = Vector::from_points(&line.start, &line.end).normalize();
    let a = Vector::new(&line.start);
    let p = Vector::new(point);
    let p2a = a.subtract(&p);
    let projection = p2a.dot(&n);
    let projection_vector = n.product(projection);
    if p2a.is_equal_to(&projection_vector) {
        Vector::origin()
    } else {
        p2a.subtract(&projection_vector)
    }
}

pub fn polygon_to_sections(points: &Polygon) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::with_capacity(points.len());
    for i in 0..points.len() - 1 {
        let polygon_point1 = points[i];
        let polygon_point2 = points[i + 1];
        let section = Section {
            start: polygon_point1,
            end: polygon_point2,
        };
        sections.push(section);
    }
    let closing_section = Section {
        start: points[points.len() - 1],
        end: points[0],
    };
    sections.push(closing_section);

    sections
}

pub fn is_lines_intersects(line1: &Section, line2: &Section) -> bool {
    let vector1 = (line2.end.x - line2.start.x) * (line1.start.y - line2.start.y)
        - (line2.end.y - line2.start.y) * (line1.start.x - line2.start.x);
    let vector2 = (line2.end.x - line2.start.x) * (line1.end.y - line2.start.y)
        - (line2.end.y - line2.start.y) * (line1.end.x - line2.start.x);
    let vector3 = (line1.end.x - line1.start.x) * (line2.start.y - line1.start.y)
        - (line1.end.y - line1.start.y) * (line2.start.x - line1.start.x);
    let vector4 = (line1.end.x - line1.start.x) * (line2.end.y - line1.start.y)
        - (line1.end.y - line1.start.y) * (line2.end.x - line1.start.x);
    vector1 * vector2 <= 0.0 && vector3 * vector4 <= 0.0
}

pub fn is_point_belongs_to_line(line: &Section, point: &Point) -> bool {
    let mp1 = Vector::from_points(point, &line.start);
    let mp2 = Vector::from_points(point, &line.end);
    let dot = mp1.dot(&mp2);
    let distance = get_vector_to_line(line, point).get_length();
    dot <= 0.0 && distance <= 1.0
}

pub fn is_point_in_polygon(polygon: &Polygon, point: &Point) -> bool {
    let mut inside = false;
    let sections = polygon_to_sections(polygon);
    for section in sections {
        let is_point_above = point.y < section.start.y && point.y < section.end.y;
        let is_point_below = point.y > section.start.y && point.y > section.end.y;
        if is_point_above || is_point_below {
            continue;
        }

        let sx = section.start.x
            + (section.end.x - section.start.x) * (point.y - section.start.y)
                / (section.end.y - section.start.y);
        if point.x > sx {
            inside = !inside;
        }
    }

    inside
}
