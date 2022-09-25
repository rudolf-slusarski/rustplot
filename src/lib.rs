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

    /// checks validity of parentheses in the equation by counting
    fn check_parentheses(&self) -> bool {
        self.formula.matches('(').count() == self.formula.matches(')').count()
            && self.formula.matches('[').count() == self.formula.matches(']').count()
    }
}

enum SpecialFunction {
    Sine,
    Cosine,
    Tangent,
}
