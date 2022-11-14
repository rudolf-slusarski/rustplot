use svg::{
    node::element::{Group, Line},
    Node,
};

enum Direction {
    Vertical,
    Horizontal,
}

struct Axis {
    direction: Direction,
    range: (f64, f64),
    size: (u32, u32),
    offset: u32,
}

impl Axis {
    fn new(direction: Direction, range: (f64, f64), size: (u32, u32)) -> Self {
        Axis {
            direction,
            range,
            size,
            offset: 3,
        }
    }

    fn to_svg(&self) -> Group {
        let mut group = Group::new().set("stroke", "black").set("stroke_width", 0.5);
        let (width, height) = self.size;

        let x_axis = Line::new()
            .set("x1", self.offset)
            .set("y1", height / 2)
            .set("x2", width - self.offset)
            .set("y2", height / 2);

        let y_axis = Line::new()
            .set("x1", width / 2)
            .set("y1", self.offset)
            .set("x2", width / 2)
            .set("y2", height - self.offset);

        group.append(x_axis);
        group.append(y_axis);

        group
    }
}
