use std::io;

use svg::{node::element::Rectangle, Document, Node};

use crate::{
    axis::{Axis, Direction},
    plot::Plot,
};

pub struct Layout {
    dimensions: (u32, u32),
    plots: Vec<Plot>,
    x_axis: Axis,
    y_axis: Axis,
}

impl Layout {
    pub fn new(dimensions: (u32, u32)) -> Self {
        let x_axis = Axis::new(Direction::Horizontal, (0.0, 0.0), dimensions);
        let y_axis = Axis::new(Direction::Vertical, (0.0, 0.0), dimensions);
        Self {
            dimensions,
            plots: vec![],
            x_axis,
            y_axis,
        }
    }

    pub fn add_plot(&mut self, plot: Plot) {
        self.plots.push(plot);
    }

    pub fn render(&self) -> svg::Document {
        let (width, height) = self.dimensions;
        let mut document = Document::new().set("viewBox", (0, 0, width, height));
        let background = Rectangle::new()
            .set("fill", "white")
            .set("x", 0)
            .set("y", 0)
            .set("width", width)
            .set("height", height);

        document.append(background);
        document.append(self.x_axis.as_svg());
        document.append(self.y_axis.as_svg());

        for p in &self.plots {
            document.append(p.as_svg((self.x_axis, self.y_axis)));
        }

        document
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        svg::save(path, &self.render())
    }
}
