use svg::{
    node::element::{
        path::{Command, Data, Position},
        Group, Path,
    },
    Node,
};

#[derive(Clone)]
pub struct Plot {
    pub data: Vec<(f64, f64)>,
}

impl Plot {
    pub fn new(data: Vec<(f64, f64)>) -> Self {
        Self { data }
    }

    pub fn function<F>(f: F, inf: Option<i32>, sup: Option<i32>, sampling: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let values: Vec<(f64, f64)> = (inf.unwrap_or(-350)..=sup.unwrap_or(350))
            .map(|x| x as f64)
            .map(|x| (x, f(x)))
            .collect();
        Plot::new(values)
    }

    pub fn to_svg(&self, width: f64, height: f64) -> Group {
        let mut d: Vec<Command> = vec![];
        d.push(Command::Move(Position::Absolute, (0, height / 2.).into()));

        for n in &self.data {
            d.push(Command::Line(
                Position::Absolute,
                (n.0 + width / 2., -(n.1 - height / 2.)).into(),
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
