use std::io;

use svg::{
    node::element::{Group, Line, Rectangle},
    Document, Node,
};

use crate::plot::Plot;

pub struct Layout {
    dimensions: (u32, u32),
    elements: Vec<Plot>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            dimensions: (600, 400),
            elements: vec![],
        }
    }

    pub fn add_plot(mut self, plot: Plot) -> Self {
        self.elements.push(plot);
        self
    }

    fn draw_axes(&self) -> Group {
        let mut axes = Group::new().set("stroke", "black").set("stroke_width", 1);
        let (width, height) = self.dimensions;
        let horizontal = Line::new()
            .set("x1", 0)
            .set("y1", height / 2)
            .set("x2", width)
            .set("y2", height / 2);
        axes.append(horizontal);

        let vertical = Line::new()
            .set("x1", width / 2)
            .set("y1", 0)
            .set("x2", width / 2)
            .set("y2", height);
        axes.append(vertical);

        axes
    }

    fn to_svg(&self) -> svg::Document {
        let (width, height) = self.dimensions;
        let mut document = Document::new().set("viewBox", (0, 0, width, height));
        let background = Rectangle::new()
            .set("fill", "white")
            .set("x", 0)
            .set("y", 0)
            .set("width", width)
            .set("height", height);
        document.append(background);
        document.append(self.draw_axes());

        document
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        svg::save(path, &self.to_svg())
    }
}
