use crate::operations::Operation;
use crate::commands::Command;
use crate::procedures::DummyProcedure;
use crate::parser::parse_command;
use clap::Parser;
use std::collections::HashMap;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
pub struct Args {
    /// Path to a file
    pub file_path: std::path::PathBuf,

    /// Path to an svg or png image
    pub image_path: std::path::PathBuf,

    /// Height
    pub height: u32,

    /// Width
    pub width: u32,
}

#[derive(Debug)]
pub struct QueriesStruct {
    pub xcor: String,
    pub ycor: String,
    pub heading: String,
    pub color: String,
    pub is_pen_down: String,
}

pub fn extract_commands(
  lines: &Vec<&str>,
  start: usize,
  dummy_procedures: &mut HashMap<String, DummyProcedure>,
) -> Result<(Vec<Command>, usize), String> {
  let mut commands: Vec<Command> = Vec::new();
  let mut i = start;

  if !lines.contains(&"]") {
      return Err("Expresion Incomplete: lacking ]".to_string());
  }

  while i < lines.len() {
      let parts: Vec<&str> = lines
          .get(i)
          .map(|line| line.split_whitespace())
          .unwrap()
          .collect();

      match parts.first() {
          Some(&"IF") => {
              if parts[parts.len() - 1] != "[" {
                  return Err("IF expression does not start with [".to_string());
              }

              let raw_operations = parts[1..parts.len() - 1].to_vec();
              let (block_commands, new_index) = extract_commands(lines, i + 1, dummy_procedures)?;
              i = new_index;
              let operations = extract_operations(&raw_operations)?;

              commands.push(Command::If(operations, block_commands));
          }
          Some(&"WHILE") => {
              if parts[parts.len() - 1] != "[" {
                  return Err("WHILE expression does not start with [".to_string());
              }

              let raw_operations = parts[1..parts.len() - 1].to_vec();
              let (block_commands, new_index) = extract_commands(lines, i + 1, dummy_procedures)?;
              i = new_index;
              let operations = extract_operations(&raw_operations)?;

              commands.push(Command::Whlie(operations, block_commands));
          }
          Some(&"]") => {
              return Ok((commands, i));
          }
          Some(&"TO") => {
              if parts.len() < 2 {
                  return Err("TO command has wrong number of arguments".to_string());
              }
              let procedure_name = parts[1].to_string();
              let mut args: Vec<String> = Vec::new();
              let (block_commands, new_index) = extract_commands(lines, i + 1, dummy_procedures)?;
              i = new_index;
              if parts.len() > 2 {
                  args.extend(parts[2..].iter().map(|&s| s.to_string()));
              }
              dummy_procedures.insert(
                  procedure_name,
                  DummyProcedure {
                      args,
                      commands: block_commands,
                  },
              );
          }
          Some(&"END") => {
              return Ok((commands, i));
          }
          _ => match parse_command(lines[i], dummy_procedures) {
              Ok(cmd) => commands.push(cmd),
              Err(e) => return Err(e),
          },
      }
      i += 1;
  }
  Ok((commands, i))
}

pub fn extract_operations(operations: &[&str]) -> Result<Operation, String> {
  let mut stack: Vec<Operation> = Vec::new();

  for operation in operations.iter().rev() {
      match *operation {
          "+" | "-" | "*" | "/" | "EQ" | "NE" | "AND" | "OR" | "GT" | "LT" => {
              let left = stack.pop().expect("Missing left operand");
              let right = stack.pop().expect("Missing right operand");
              let op = match *operation {
                  "+" => Operation::Add(Box::new(left), Box::new(right)),
                  "-" => Operation::Subtract(Box::new(left), Box::new(right)),
                  "*" => Operation::Multiply(Box::new(left), Box::new(right)),
                  "/" => Operation::Divide(Box::new(left), Box::new(right)),
                  "EQ" => Operation::Equal(Box::new(left), Box::new(right)),
                  "NE" => Operation::Notequal(Box::new(left), Box::new(right)),
                  "AND" => Operation::And(Box::new(left), Box::new(right)),
                  "OR" => Operation::Or(Box::new(left), Box::new(right)),
                  "GT" => Operation::Greaterthan(Box::new(left), Box::new(right)),
                  "LT" => Operation::Lessthan(Box::new(left), Box::new(right)),
                  _ => {
                      return Err(format!("Invalid operator {}", operation));
                  }
              };
              stack.push(op);
          }
          _ => {
              if let Some(stripped) = operation.strip_prefix('\"') {
                  if stripped.parse::<f32>().is_ok() {
                      stack.push(Operation::Base(operation.to_string()));
                  } else if stripped == "TRUE" || stripped == "FALSE" {
                      stack.push(Operation::Base(operation.to_string()));
                  } else {
                      return Err(format!("Unexpected value type {}", operation));
                  }
              } else {
                  stack.push(Operation::Base(operation.to_string()));
              }
          }
      }
  }

  if stack.len() == 1 {
      Ok(stack.pop().expect("Invalid expression"))
  } else {
      Err("There are still some values in the stack".to_string())
  }
}