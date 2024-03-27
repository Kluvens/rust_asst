#[derive(Debug, Clone)]
pub enum Operation {
    Base(String),
    Add(Box<Operation>, Box<Operation>),
    Subtract(Box<Operation>, Box<Operation>),
    Multiply(Box<Operation>, Box<Operation>),
    Divide(Box<Operation>, Box<Operation>),
    Equal(Box<Operation>, Box<Operation>),
    Notequal(Box<Operation>, Box<Operation>),
    Greaterthan(Box<Operation>, Box<Operation>),
    Lessthan(Box<Operation>, Box<Operation>),
    And(Box<Operation>, Box<Operation>),
    Or(Box<Operation>, Box<Operation>),
}
