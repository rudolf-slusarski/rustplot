pub struct Equation {
    pub formula: String,
    variable_count: i32,
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

enum SpecialFunction {
    Sine,
    Cosine,
    Tangent,
}
