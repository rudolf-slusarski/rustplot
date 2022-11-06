use std::io;

use svg::{
    node::element::{Group, Line, Rectangle},
    Document, Node,
};

use crate::plot::Plot;

pub struct Layout {
    dimensions: (u32, u32),
    range: (i32, i32),
    plots: Vec<Plot>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            dimensions: (700, 700),
            range: (-350, 350),
            plots: vec![],
        }
    }

    pub fn add_plot(&mut self, plot: Plot) {
        self.plots.push(plot);
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
        document.append(self.draw_axes());

        for p in &self.plots {
            document.append(p.to_svg(width as f64, height as f64));
        }

        document
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        svg::save(path, &self.render())
    }

    pub fn range(&self) -> (i32, i32) {
        self.range
    }

    pub fn set_range(&mut self, range: (i32, i32)) {
        self.range = range;
    }
}
