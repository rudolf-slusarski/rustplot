use svg::{
    node::element::{Group, Line},
    Node,
};

#[derive(Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub struct Axis {
    direction: Direction,
    pub range: (f64, f64),
    pub size: (u32, u32),
    margin: u32,
}

impl Axis {
    pub fn new(direction: Direction, range: (f64, f64), size: (u32, u32)) -> Self {
        Axis {
            direction,
            range,
            size,
            margin: 3,
        }
    }

    pub fn as_svg(&self) -> Group {
        let mut group = Group::new().set("stroke", "black").set("stroke_width", 1);
        let (width, height) = self.size;

        let axis = match self.direction {
            Direction::Vertical => Line::new()
                .set("x1", width / 2)
                .set("y1", self.margin)
                .set("x2", width / 2)
                .set("y2", height - self.margin),
            Direction::Horizontal => Line::new()
                .set("x1", self.margin)
                .set("y1", height / 2)
                .set("x2", width - self.margin)
                .set("y2", height / 2),
        };

        group.append(axis);

        group
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.margin = offset;
    }

    pub fn set_range(&mut self, range: (f64, f64)) {
        self.range = range;
    }

    pub fn place_value(&self, value: f64) -> f64 {
        let size_range = (self.size.1 - self.size.0) as f64;
        (value - self.size.0 as f64) / size_range
    }
}
