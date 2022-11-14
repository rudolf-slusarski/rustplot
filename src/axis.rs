use svg::{
    node::element::{Group, Line},
    Node,
};

pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Axis {
    direction: Direction,
    pub range: (f64, f64),
    pub size: (u32, u32),
    offset: u32,
}

impl Axis {
    pub fn new(direction: Direction, range: (f64, f64), size: (u32, u32)) -> Self {
        Axis {
            direction,
            range,
            size,
            offset: 3,
        }
    }

    fn as_svg(&self) -> Group {
        let mut group = Group::new().set("stroke", "black").set("stroke_width", 1);
        let (width, height) = self.size;

        let axis = match self.direction {
            Direction::Vertical => Line::new()
                .set("x1", width / 2)
                .set("y1", self.offset)
                .set("x2", width / 2)
                .set("y2", height - self.offset),
            Direction::Horizontal => Line::new()
                .set("x1", self.offset)
                .set("y1", height / 2)
                .set("x2", width - self.offset)
                .set("y2", height / 2),
        };

        group.append(axis);

        group
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub fn set_range(&mut self, range: (f64, f64)) {
        self.range = range;
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}
