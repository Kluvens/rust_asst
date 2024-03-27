use crate::commands::Command;
use crate::procedures::DummyProcedure;
use crate::parser::{parse_operation, parse_boolean};
use crate::utils::QueriesStruct;
use unsvg::Image;
use std::collections::HashMap;

pub fn execute_command(
  command: &Command,
  variable_table: &mut HashMap<String, String>,
  dummy_procedures: &mut HashMap<String, DummyProcedure>,
  is_in_procedure: bool,
  procedure_args: &mut HashMap<String, String>,
  queries: &mut QueriesStruct,
  image: &mut Image,
) -> Result<(), String> {
  match command {
      Command::Penup => {
          queries.is_pen_down = "FALSE".to_string();
      }
      Command::Pendown => {
          queries.is_pen_down = "TRUE".to_string();
      }
      Command::Forward(numpixels)
      | Command::Back(numpixels)
      | Command::Right(numpixels)
      | Command::Left(numpixels) => {
          let table = if is_in_procedure {
              procedure_args
          } else {
              variable_table
          };
          let is_pen_down = parse_boolean(&queries.is_pen_down)?;
          let x = queries.xcor[1..]
              .parse::<f32>()
              .expect("cannot parse as x coordinate");
          let y = queries.ycor[1..]
              .parse::<f32>()
              .expect("cannot parse as y coordinate");
          let direction = queries.heading[1..]
              .parse::<i32>()
              .expect("cannot parse as direction");
          let result = parse_operation(numpixels, table, queries)?;
          let length = result[1..].parse::<f32>().expect("cannot parse as length");
          match command {
              Command::Forward(_numpixels) => {
                  if is_pen_down {
                      let color_index: usize =
                          queries.color[1..].parse::<usize>().expect("Invalid color");
                      let (new_x, new_y) = Image::draw_simple_line(
                          image,
                          x,
                          y,
                          direction,
                          length,
                          unsvg::COLORS[color_index],
                      )
                      .unwrap();
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  } else {
                      let (new_x, new_y) = unsvg::get_end_coordinates(x, y, direction, length);
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  }
              }
              Command::Back(_numpixels) => {
                  if is_pen_down {
                      let color_index: usize =
                          queries.color[1..].parse::<usize>().expect("Invalid color");
                      let (new_x, new_y) = Image::draw_simple_line(
                          image,
                          x,
                          y,
                          direction + 180,
                          length,
                          unsvg::COLORS[color_index],
                      )
                      .unwrap();
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  } else {
                      let (new_x, new_y) =
                          unsvg::get_end_coordinates(x, y, direction + 180, length);
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  }
              }
              Command::Right(_numpixels) => {
                  if is_pen_down {
                      let color_index: usize =
                          queries.color[1..].parse::<usize>().expect("Invalid color");
                      let (new_x, new_y) = Image::draw_simple_line(
                          image,
                          x,
                          y,
                          direction + 90,
                          length,
                          unsvg::COLORS[color_index],
                      )
                      .unwrap();
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  } else {
                      let (new_x, new_y) =
                          unsvg::get_end_coordinates(x, y, direction + 90, length);
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  }
              }
              Command::Left(_numpixels) => {
                  if is_pen_down {
                      let color_index: usize =
                          queries.color[1..].parse::<usize>().expect("Invalid color");
                      let (new_x, new_y) = Image::draw_simple_line(
                          image,
                          x,
                          y,
                          direction + 270,
                          length,
                          unsvg::COLORS[color_index],
                      )
                      .unwrap();
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  } else {
                      let (new_x, new_y) =
                          unsvg::get_end_coordinates(x, y, direction + 270, length);
                      queries.xcor = format!("{}{}", "\"", new_x);
                      queries.ycor = format!("{}{}", "\"", new_y);
                  }
              }
              _ => {
                  return Err("Invalid Command".to_string());
              }
          }
      }
      Command::Setpencolor(colorcode) => {
          let table = if is_in_procedure {
              procedure_args
          } else {
              variable_table
          };
          let result = parse_operation(colorcode, table, queries)?;
          queries.color = result;
      }
      Command::Turn(degrees) | Command::Setheading(degrees) => {
          let table = if is_in_procedure {
              procedure_args
          } else {
              variable_table
          };
          let result = parse_operation(degrees, table, queries)?;
          match command {
              Command::Turn(_degrees) => {
                  let new_direction = queries.heading[1..]
                      .parse::<i32>()
                      .expect("cannot parse heading")
                      + result[1..].parse::<i32>().expect("cannot parse result");
                  queries.heading = format!("{}{}", "\"", new_direction);
              }
              Command::Setheading(_degrees) => {
                  queries.heading = result;
              }
              _ => {
                  return Err("Invalid Command".to_string());
              }
          }
      }
      Command::Setx(location) | Command::Sety(location) => {
          let table = if is_in_procedure {
              procedure_args
          } else {
              variable_table
          };
          let result = parse_operation(location, table, queries)?;
          match command {
              Command::Setx(_location) => {
                  queries.xcor = result;
              }
              Command::Sety(_location) => {
                  queries.ycor = result;
              }
              _ => {
                  return Err("Invalid Command".to_string());
              }
          }
      }
      Command::Make(variable_name, value) => {
          let table = if is_in_procedure {
              procedure_args
          } else {
              variable_table
          };
          let variable_value = parse_operation(value, table, queries)?;
          table.insert(variable_name.clone().replace('\"', ":"), variable_value);
      }
      Command::Addassign(variable_name, value) => {
          let lookup_key = variable_name.replace('\"', ":");
          match variable_table.get(&lookup_key) {
              Some(var) => {
                  let num = var[1..].parse::<f32>().expect("not a number");
                  let operation_result = parse_operation(value, variable_table, queries)?;
                  let add_num = operation_result[1..].parse::<f32>().expect("not a number");

                  let result = num + add_num;
                  variable_table.insert(lookup_key, format!("{}{}", '\"', result));
              }
              None => {
                  return Err("variable not defined".to_string());
              }
          }
      }
      Command::If(operation, commands) => {
          let table = if is_in_procedure {
              &procedure_args
          } else {
              &variable_table
          };
          let if_condition = parse_operation(operation, table, queries)?;
          if if_condition == *"TRUE" {
              for command in commands.iter() {
                  execute_command(
                      command,
                      variable_table,
                      dummy_procedures,
                      is_in_procedure,
                      procedure_args,
                      queries,
                      image,
                  )?;
              }
          }
      }
      Command::Whlie(operation, commands) => {
          while {
              // Limit the scope of the immutable borrow
              let table = if is_in_procedure {
                  &procedure_args
              } else {
                  &variable_table
              };
              parse_operation(operation, table, queries)? == *"TRUE"
          } {
              for command in commands.iter() {
                  // Now variable_table is not immutably borrowed in this scope
                  execute_command(
                      command,
                      variable_table,
                      dummy_procedures,
                      is_in_procedure,
                      procedure_args,
                      queries,
                      image,
                  )?;
              }
          }
      }
      Command::Procedure(procedure_name, params) => {
          // get procedure name
          let procedure = dummy_procedures.get(procedure_name).expect("msg");
          // get procedure defined args list
          let args = procedure.args.clone();
          // get procedure commands
          let commands = procedure.commands.clone();
          // initialise args table
          let mut args_table: HashMap<String, String> = HashMap::new();

          for (arg, param) in args.iter().zip(params.iter()) {
              let table = if is_in_procedure {
                  &procedure_args
              } else {
                  &variable_table
              };
              match table.get(param) {
                  Some(parameter) => {
                      args_table.insert(arg.replace('\"', ":"), parameter.clone());
                  }
                  None => {
                      return Err("lookup key not found".to_string());
                  }
              }
          }

          for command in commands {
              if let Err(e) = execute_command(
                  &command,
                  variable_table,
                  dummy_procedures,
                  true,
                  &mut args_table,
                  queries,
                  image,
              ) {
                  eprintln!(
                      "\x1b[31mError processing commands inside a procedure: {}\x1b[0m",
                      e
                  );
                  std::process::exit(1);
              }
          }
      }
  }
  Ok(())
}