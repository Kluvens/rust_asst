use crate::operations::Operation;

#[derive(Debug, Clone)]
pub enum Command {
    Penup,
    Pendown,
    Forward(Operation),
    Back(Operation),
    Left(Operation),
    Right(Operation),
    Setpencolor(Operation),
    Turn(Operation),
    Setheading(Operation),
    Setx(Operation),
    Sety(Operation),
    Make(String, Operation),
    Addassign(String, Operation),
    If(Operation, Vec<Command>),
    Whlie(Operation, Vec<Command>),
    Procedure(String, Vec<String>),
}
