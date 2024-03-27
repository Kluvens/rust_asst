use crate::commands::Command;
use crate::operations::Operation;
use crate::procedures::DummyProcedure;
use crate::utils::{QueriesStruct, extract_operations};
use std::collections::HashMap;

pub fn parse_command(
  line: &str,
  dummy_procedures: &HashMap<String, DummyProcedure>,
) -> Result<Command, String> {
  let parts: Vec<&str> = line.split_whitespace().collect();

  match parts[0] {
      "PENUP" | "PENDOWN" if parts.len() == 1 => Ok(match parts[0] {
          "PENUP" => Command::Penup,
          "PENDOWN" => Command::Pendown,
          _ => unreachable!(), // We won't get here
      }),
      "FORWARD" | "BACK" | "RIGHT" | "LEFT" | "SETPENCOLOR" | "TURN" | "SETHEADING" | "SETX"
      | "SETY"
          if parts.len() > 1 =>
      {
          let operations = parts[1..].to_vec();
          let extracted = extract_operations(&operations)?; // Make sure this function returns Result as well

          Ok(match parts[0] {
              "FORWARD" => Command::Forward(extracted),
              "BACK" => Command::Back(extracted),
              "RIGHT" => Command::Right(extracted),
              "LEFT" => Command::Left(extracted),
              "SETPENCOLOR" => Command::Setpencolor(extracted),
              "TURN" => Command::Turn(extracted),
              "SETHEADING" => Command::Setheading(extracted),
              "SETX" => Command::Setx(extracted),
              "SETY" => Command::Sety(extracted),
              _ => unreachable!(), // We won't get here
          })
      }
      "MAKE" | "ADDASSIGN" if parts.len() > 2 => {
          let variable_name = parts[1];
          let operations = parts[2..].to_vec();
          let extracted = extract_operations(&operations)?; // Adjust for error handling

          Ok(match parts[0] {
              "MAKE" => Command::Make(variable_name.to_string(), extracted),
              "ADDASSIGN" => Command::Addassign(variable_name.to_string(), extracted),
              _ => unreachable!(), // We won't get here
          })
      }
      _ if dummy_procedures.contains_key(parts[0]) => {
          let args: Vec<String> = parts
              .get(1..)
              .unwrap_or(&[])
              .iter()
              .map(|&s| s.to_string())
              .collect();
          let procedure = dummy_procedures
              .get(parts[0])
              .ok_or_else(|| "Procedure not found".to_string())?;
          if args.len() == procedure.args.len() {
              Ok(Command::Procedure(parts[0].to_string(), args))
          } else {
              Err("Number of parameters does not match".to_string())
          }
      }
      _ => Err("Invalid command or wrong number of arguments".to_string()),
  }
}

pub fn parse_operation(
  operation: &Operation,
  variable_table: &HashMap<String, String>,
  queries: &mut QueriesStruct,
) -> Result<String, String> {
  match operation {
      Operation::Base(raw_value) => match raw_value.chars().next() {
          Some('\"') => {
              if raw_value == "\"TRUE" || raw_value == "\"FALSE" {
                  Ok(raw_value.clone())
              } else {
                  raw_value[1..]
                      .parse::<f32>()
                      .map_err(|_| "BASE operation: Not a number".to_string())
                      .and_then(|_| {
                          Ok(raw_value.to_string())
                      })
              }
          },
          Some(':') => {
              let lookup_key = raw_value.clone();
              variable_table
                  .get(&lookup_key)
                  .cloned()
                  .ok_or_else(|| "BASE operation: Key error".to_string())
          }
          _ => match raw_value.as_str() {
              "XCOR" => Ok(queries.xcor.clone()),
              "YCOR" => Ok(queries.ycor.clone()),
              "HEADING" => Ok(queries.heading.clone()),
              "COLOR" => Ok(queries.color.clone()),
              _ => Err(format!("BASE operation: Unexpected value {}", raw_value)),
          },
      },
      Operation::Add(a, b)
      | Operation::Subtract(a, b)
      | Operation::Multiply(a, b)
      | Operation::Divide(a, b)
      | Operation::Equal(a, b)
      | Operation::Notequal(a, b)
      | Operation::Lessthan(a, b)
      | Operation::Greaterthan(a, b)
      | Operation::And(a, b)
      | Operation::Or(a, b) => {
          let left = parse_operation(a, variable_table, queries)?;
          let right = parse_operation(b, variable_table, queries)?;

          match operation {
              Operation::Add(_a, _b) => {
                  let result = left[1..].parse::<f32>().expect("not a number")
                      + right[1..].parse::<f32>().expect("not a number");

                  Ok(format!("{}{}", '\"', result))
              }
              Operation::Subtract(_a, _b) => {
                  let result = left[1..].parse::<f32>().expect("not a number")
                      - right[1..].parse::<f32>().expect("not a number");

                  Ok(format!("{}{}", '\"', result))
              }
              Operation::Multiply(_a, _b) => {
                  let result = left[1..].parse::<f32>().expect("not a number")
                      * right[1..].parse::<f32>().expect("not a number");

                  Ok(format!("{}{}", '\"', result))
              }
              Operation::Divide(_a, _b) => {
                  if right[1..].parse::<f32>().expect("not a number") == 0.0 {
                      return Err("divide by 0".to_string());
                  }

                  let result = left[1..].parse::<f32>().expect("not a number")
                      / right[1..].parse::<f32>().expect("not a number");

                  Ok(format!("{}{}", '\"', result))
              }
              Operation::Equal(_a, _b) => {
                  if (left == "\"TRUE" || left == "\"FALSE") && (right == "\"TRUE" || right == "\"FALSE") {
                      if left == right {
                          return Ok("TRUE".to_string());
                      } else {
                          return Ok("FALSE".to_string());
                      }
                  }

                  if (left[1..].parse::<f32>().expect("not a number")
                      - right[1..].parse::<f32>().expect("not a number"))
                  .abs()
                      < f32::EPSILON
                  {
                      Ok("TRUE".to_string())
                  } else {
                      Ok("FALSE".to_string())
                  }
              }
              Operation::Notequal(_a, _b) => {
                  if (left == "\"TRUE" || left == "\"FALSE") && (right == "\"TRUE" || right == "\"FALSE") {
                      if left == right {
                          return Ok("TRUE".to_string());
                      } else {
                          return Ok("FALSE".to_string());
                      }
                  }

                  if (left[1..].parse::<f32>().expect("not a number")
                      - right[1..].parse::<f32>().expect("not a number"))
                  .abs()
                      < f32::EPSILON
                  {
                      Ok("FALSE".to_string())
                  } else {
                      Ok("TRUE".to_string())
                  }
              }
              Operation::Lessthan(_a, _b) => {
                  if left[1..].parse::<f32>().expect("not a number")
                      - right[1..].parse::<f32>().expect("not a number")
                      < 0.0
                  {
                      Ok("TRUE".to_string())
                  } else {
                      Ok("FALSE".to_string())
                  }
              }
              Operation::Greaterthan(_a, _b) => {
                  if left[1..].parse::<f32>().expect("not a number")
                      - right[1..].parse::<f32>().expect("not a number")
                      > 0.0
                  {
                      Ok("TRUE".to_string())
                  } else {
                      Ok("FALSE".to_string())
                  }
              }
              Operation::And(_a, _b) => {
                  let left_bool = parse_boolean(&left)?;
                  let right_bool = parse_boolean(&right)?;

                  Ok(if left_bool && right_bool {
                      "TRUE".to_string()
                  } else {
                      "FALSE".to_string()
                  })
              }
              Operation::Or(_a, _b) => {
                  let left_bool = parse_boolean(&left)?;
                  let right_bool = parse_boolean(&right)?;

                  Ok(if left_bool || right_bool {
                      "TRUE".to_string()
                  } else {
                      "FALSE".to_string()
                  })
              }
              _ => Err("msg".to_string()),
          }
      }
  }
}

pub fn parse_boolean(value: &str) -> Result<bool, String> {
  match value {
      "TRUE" => Ok(true),
      "FALSE" => Ok(false),
      _ => Err("Unknown operator".to_string()),
  }
}
