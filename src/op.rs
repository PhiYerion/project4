/// Any operation that can be performed on two numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

/// Precedence of operations. The first array is the highest precedence, the last is the lowest.
pub const PRECEDENCE: [&[Op]; 3] = [
    &[Op::Pow],
    &[Op::Mod, Op::Div, Op::Mul],
    &[Op::Sub, Op::Add],
];

impl From<Op> for String {
    fn from(op: Op) -> String {
        match op {
            Op::Add => "+".to_string(),
            Op::Sub => "-".to_string(),
            Op::Mul => "*".to_string(),
            Op::Div => "/".to_string(),
            Op::Mod => "%".to_string(),
            Op::Pow => "^".to_string(),
        }
    }
}

impl Op {
    /// Perform the operation on two numbers.
    pub fn operate(&self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            Op::Mod => a % b,
            Op::Pow => a.powf(b),
        }
    }
}
