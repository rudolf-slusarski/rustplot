use std::vec;

use svg::{node::element::Line, Document, Node};

use crate::plot::Plot;

pub struct Layout {
    dimensions: (u32, u32),
    elements: Vec<Plot>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            dimensions: (300, 300),
            elements: vec![],
        }
    }

    pub fn add_plot(mut self, plot: Plot) -> Self {
        self.elements.push(plot);
        self
    }

    fn add_axis(&self) -> Line {
        let axis = Line::new()
            .set("x1", 100)
            .set("x2", 100)
            .set("y1", 100)
            .set("y2", 200)
            .set("stroke", "black")
            .set("stroke_width", 1);
        axis
    }

    fn to_svg(&self) -> svg::Document {
        let (width, height) = self.dimensions;
        let mut document = Document::new().set("viewBox", (0, 0, width, height));

        document.append(self.add_axis());

        document
    }

    pub fn save(&self, path: &str) {
        svg::save(path, &self.to_svg());
    }
}
