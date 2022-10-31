pub struct Plot {
    pub data: Vec<(f64, f64)>,
}

impl Plot {
    pub fn new(data: Vec<(f64, f64)>) -> Self {
        Self { data }
    }
    
    pub fn function<F>(f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let values: Vec<(f64, f64)> = (-100..100)
            .map(|x| x as f64)
            .map(|x| (x, f(x)))
            .collect();
        Plot::new(values)
    }
}
