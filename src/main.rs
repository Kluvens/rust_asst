use core::f32;

use clap::Parser;
use std::collections::HashMap;
use std::fs::read_to_string;
use unsvg::Image;

#[derive(Debug, Clone)]
enum Command {
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

#[derive(Debug, Clone)]
enum Operation {
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

#[derive(Debug)]
struct DummyProcedure {
    args: Vec<String>,
    commands: Vec<Command>,
}

#[derive(Debug)]
struct QueriesStruct {
    xcor: String,
    ycor: String,
    heading: String,
    color: String,
    is_pen_down: String,
}

fn execute_command(
    command: &Command,
    variable_table: &mut HashMap<String, String>,
    dummy_procedures: &mut HashMap<String, DummyProcedure>,
    is_in_procedure: bool,
    procedure_args: &mut HashMap<String, String>,
    queries: &mut QueriesStruct,
    mut image: &mut Image,
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
                            &mut image,
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
                            &mut image,
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
            let if_condition = parse_operation(operation, &table, queries)?;
            if if_condition == "TRUE".to_string() {
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

fn parse_operation(
    operation: &Operation,
    variable_table: &HashMap<String, String>,
    queries: &mut QueriesStruct,
) -> Result<String, String> {
    match operation {
        Operation::Base(value) => match value.chars().next() {
            Some('\"') => value[1..]
                .parse::<f32>()
                .map_err(|_| "BASE operation: Not a number".to_string())
                .and_then(|num| {
                    if num.is_finite() {
                        Ok(value.to_string())
                    } else {
                        Err("BASE operation: Number is not normal".to_string())
                    }
                }),
            Some(':') => {
                let lookup_key = value.clone();
                variable_table
                    .get(&lookup_key)
                    .cloned()
                    .ok_or_else(|| "BASE operation: Key error".to_string())
            }
            _ => match value.as_str() {
                "XCOR" => Ok(queries.xcor.clone()),
                "YCOR" => Ok(queries.ycor.clone()),
                "HEADING" => Ok(queries.heading.clone()),
                "COLOR" => Ok(queries.color.clone()),
                _ => Err(format!("BASE operation: Unexpected value {}", value)),
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
                    if left == right {
                        return Ok("TRUE".to_string());
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
                    if left == right {
                        return Ok("FALSE".to_string());
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

fn parse_boolean(value: &str) -> Result<bool, String> {
    match value {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        _ => Err("Unknown operator".to_string()),
    }
}

fn extract_operations(operations: &Vec<&str>) -> Result<Operation, String> {
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
                    if let Ok(_) = stripped.parse::<f32>() {
                        stack.push(Operation::Base(operation.to_string()));
                    } else {
                        if stripped != "TRUE" && stripped != "FALSE" {
                            return Err(format!("Unexpected value type {}", operation));
                        }
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
        Err("There are sitll some values in the stack".to_string())
    }
}

fn parse_command(
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

/// There are a few possible commands:
/// First is basic commands such as Pendown and Forward
/// Second is IF and WHILE as well as Back bracket "]"
/// Third would be Procedure definition and calling
fn extract_commands(
    lines: &Vec<&str>,
    start: usize,
    dummy_procedures: &mut HashMap<String, DummyProcedure>,
) -> Result<(Vec<Command>, usize), String> {
    let mut commands: Vec<Command> = Vec::new();
    let mut i = start;

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

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let file_content = match read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            return Err(());
        }
    };

    let mut dummy_procedures: HashMap<String, DummyProcedure> = HashMap::new();
    let mut variable_table: HashMap<String, String> = HashMap::new();

    let mut lines: Vec<&str> = Vec::new();

    for line in file_content.lines() {
        let line = line.trim();
        if line.starts_with("// ") || line.is_empty() {
            continue;
        }
        lines.push(line);
    }

    let commands = match extract_commands(&lines, 0, &mut dummy_procedures) {
        Ok((commands, _)) => commands,
        Err(e) => {
            eprintln!("\x1b[31mError processing commands: {}\x1b[0m", e);
            std::process::exit(1);
        }
    };

    let mut image = Image::new(width, height);

    let mut queries_struct = QueriesStruct {
        xcor: format!("{}{}", "\"", (width / 2)),
        ycor: format!("{}{}", "\"", (height / 2)),
        heading: "\"0".to_string(),
        color: "\"7".to_string(),
        is_pen_down: "FALSE".to_string(),
    };

    let mut map: HashMap<String, String> = HashMap::new();
    for command in commands.iter() {
        if let Err(e) = execute_command(
            command,
            &mut variable_table,
            &mut dummy_procedures,
            false,
            &mut map,
            &mut queries_struct,
            &mut image,
        ) {
            eprintln!("\x1b[31mError processing commands: {}\x1b[0m", e);
            std::process::exit(1);
        }
    }

    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
