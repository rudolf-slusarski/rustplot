enum Direction {
    Vertical,
    Horizontal,
}

struct Axis {
    direction: Direction,
    range: (f64, f64),
    size: (u32, u32),
}

impl Axis {
    fn new(direction: Direction, range: (f64, f64), size: (u32, u32)) -> Self {
        Axis {
            direction,
            range,
            size,
        }
    }
}
