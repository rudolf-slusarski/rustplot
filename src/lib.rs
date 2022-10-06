mod tokenise;

pub struct Equation {
    pub formula: String,
    variable_count: u32,
    special_functions: Vec<SpecialFunction>,
}

impl Equation {
    pub fn new(formula: String) -> Self {
        Self {
            formula,
            variable_count: 0,
            special_functions: vec![],
        }
    }

}

pub enum SpecialFunction {
    Sine,
    Cosine,
    Tangent,
    Absolute,
}
