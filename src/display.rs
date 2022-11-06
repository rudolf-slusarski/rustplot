use svg::{
    node::element::{
        path::{Command, Data, Position},
        Group, Path,
    },
    Node,
};

use crate::plot::Plot;

pub struct Display {
    plots: Vec<Plot>,
}

impl Display {
    pub fn add_plot(&mut self, plot: Plot) {
        self.plots.push(plot);
    }

    pub fn to_svg(&self) -> Group {
        let mut d: Vec<Command> = vec![];
        d.push(Command::Move(Position::Absolute, (0, 350).into()));

        for n in &self.data {
            d.push(Command::Line(
                Position::Absolute,
                (n.0 + 350.0, -(n.1 - 350.0)).into(),
            ))
        }

        let path = Data::from(d);
        let mut group = Group::new();
        group.append(
            Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-linejoin", "miter")
                .set("d", path),
        );

        group
    }
}
