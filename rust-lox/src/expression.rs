pub trait Expression {
    fn evaluate(&self) -> f64;
}

pub struct ConstantExpression {
    value: f64,
}

pub struct UnaryExpression<'a> {
    expression: Box<dyn Expression + 'a>,
    operation: fn(f64) -> f64,
}

pub struct BinaryExpression<'a> {
    left: Box<dyn Expression + 'a>,
    right: Box<dyn Expression + 'a>,
    operation: fn(f64, f64) -> f64,
}

impl ConstantExpression {
    pub fn new(value: f64) -> Self {
        ConstantExpression { value }
    }
}

impl<'a> UnaryExpression<'a> {
    pub fn new(expression: Box<dyn Expression + 'a>, operation: fn(f64) -> f64) -> Self {
        UnaryExpression {
            expression,
            operation,
        }
    }
}

impl<'a> BinaryExpression<'a> {
    pub fn new(
        left: Box<dyn Expression + 'a>,
        right: Box<dyn Expression + 'a>,
        operation: fn(f64, f64) -> f64,
    ) -> Self {
        BinaryExpression {
            left,
            right,
            operation,
        }
    }
}

impl<'a> Expression for UnaryExpression<'a> {
    fn evaluate(&self) -> f64 {
        (self.operation)(self.expression.evaluate())
    }
}

impl<'a> Expression for BinaryExpression<'a> {
    fn evaluate(&self) -> f64 {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        (self.operation)(left, right)
    }
}
