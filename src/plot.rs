use svg::{
    node::element::{
        path::{Command, Data, Position},
        Group, Path,
    },
    Node,
};

use crate::axis::Axis;

#[derive(Clone)]
pub struct Plot {
    pub data: Vec<(f64, f64)>,
}

impl Plot {
    pub fn new(data: Vec<(f64, f64)>) -> Self {
        Self { data }
    }

    pub fn function<F>(f: F, inf: f64, sup: f64, sampling: f64) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let samples_count = (sup - inf / sampling) as u32;

        let mut samples = vec![];
        for n in 0..samples_count {
            samples.push(inf + sampling * n as f64);
        }

        let values: Vec<(f64, f64)> = samples.into_iter().map(|x| (x, f(x))).collect();

        Plot::new(values)
    }

    fn get_min_value(&self) -> f64 {
        self.data
            .iter()
            .map(|(_, v)| *v)
            .fold(f64::MAX, |acc: f64, x: f64| acc.min(x))
    }

    fn get_max_value(&self) -> f64 {
        self.data
            .iter()
            .map(|(_, v)| *v)
            .fold(f64::MIN, |acc, x| acc.max(x))
    }

    pub fn as_svg(&self, axes: (Axis, Axis)) -> Group {
        let (mut horizontal, mut vertical) = axes;

        horizontal.set_range((self.data[0].0, self.data.last().unwrap().0));
        vertical.set_range((self.get_min_value(), self.get_max_value()));

        let mut d: Vec<Command> = vec![];
        d.push(Command::Move(
            Position::Absolute,
            (
                horizontal.place_value(self.data[0].0),
                vertical.place_value(self.data[0].1),
            )
                .into(),
        ));

        for n in &self.data {
            d.push(Command::Line(
                Position::Absolute,
                (horizontal.place_value(n.0), vertical.place_value(n.1)).into(),
            ))
        }

        let path = Data::from(d);
        let mut group = Group::new();
        group.append(
            Path::new()
                .set("fill", "none")
                .set("stroke", "blue")
                .set("stroke-linejoin", "miter")
                .set("d", path),
        );

        group
    }
}

#[cfg(test)]
mod tests {
    use super::Plot;

    #[test]
    fn test_min_value() {
        let plot = Plot::function(|x| x * x + 30., -20.0, 10.0, 0.1);
        assert_eq!(30., plot.get_min_value());
    }
}
