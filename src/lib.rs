struct Equation {
    formula: String,
}

impl Equation {
    /// checks validity of parentheses in the equation by counting
    fn check_parentheses(&self) -> bool {
        self.formula.matches('(').count() == self.formula.matches(')').count()
            && self.formula.matches('[').count() == self.formula.matches(']').count()
    }
}
