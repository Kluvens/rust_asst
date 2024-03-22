use core::{f32, panic};

use clap::Parser;
use unsvg::Image;
use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Command {
    PENUP,
    PENDOWN,
    FORWARD(Operation),
    BACK(Operation),
    LEFT(Operation),
    RIGHT(Operation),
    SETPENCOLOR(Operation),
    TURN(Operation),
    SETHEADING(Operation),
    SETX(Operation),
    SETY(Operation),
    MAKE(String, Operation),
    ADDASSIGN(String, Operation),
    IF(Operation, Vec<Command>),
    WHILE(Operation, Vec<Command>),
    PROCEDURE(String, Vec<String>),
}

#[derive(Debug, Clone)]
enum Operation {
    BASE(String),
    ADD(Box<Operation>, Box<Operation>),
    SUBTRACT(Box<Operation>, Box<Operation>),
    MULTIPLY(Box<Operation>, Box<Operation>),
    DIVIDE(Box<Operation>, Box<Operation>),
    EQUAL(Box<Operation>, Box<Operation>),
    NOTEQUAL(Box<Operation>, Box<Operation>),
    GREATERTHAN(Box<Operation>, Box<Operation>),
    LESSTHAN(Box<Operation>, Box<Operation>),
    AND(Box<Operation>, Box<Operation>),
    OR(Box<Operation>, Box<Operation>),
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
        Command::PENUP => {
            queries.is_pen_down = "FALSE".to_string();
        },
        Command::PENDOWN => {
            queries.is_pen_down = "TRUE".to_string();
        },
        Command::FORWARD(numpixels)
        | Command::BACK(numpixels)
        | Command::RIGHT(numpixels)
        | Command::LEFT(numpixels) => {
            let table = if is_in_procedure { procedure_args } else { variable_table };
            let is_pen_down = match queries.is_pen_down.as_str() {
                "TRUE" => true,
                "FALSE" => false,
                _ => false,
            };
            let x = queries.xcor[1..].parse::<f32>().expect("cannot parse as x coordinate");
            let y = queries.ycor[1..].parse::<f32>().expect("cannot parse as y coordinate");
            let direction = queries.heading[1..].parse::<i32>().expect("cannot parse as direction");
            let result = parse_operation(numpixels, table)?;
            let length = result[1..].parse::<f32>().expect("cannot parse as length");
            match command {
                Command::FORWARD(_numpixels) => {
                    if is_pen_down {
                        let color_index: usize = queries.color[1..].parse::<usize>().expect("Invalid color");
                        let (new_x, new_y) = Image::draw_simple_line(&mut image, x, y, direction, length, unsvg::COLORS[color_index]).unwrap();
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    } else {
                        let (new_x, new_y) = unsvg::get_end_coordinates(x, y, direction, length);
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    }
                },
                Command::BACK(_numpixels) => {
                    if is_pen_down {
                        let color_index: usize = queries.color[1..].parse::<usize>().expect("Invalid color");
                        let (new_x, new_y) = Image::draw_simple_line(&mut image, x, y, direction + 180, length, unsvg::COLORS[color_index]).unwrap();
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    } else {
                        let (new_x, new_y) = unsvg::get_end_coordinates(x, y, direction + 180, length);
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    }
                },
                Command::RIGHT(_numpixels) => {
                    if is_pen_down {
                        let color_index: usize = queries.color[1..].parse::<usize>().expect("Invalid color");
                        let (new_x, new_y) = Image::draw_simple_line(&mut image, x, y, direction + 90, length, unsvg::COLORS[color_index]).unwrap();
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    } else {
                        let (new_x, new_y) = unsvg::get_end_coordinates(x, y, direction + 90, length);
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    }
                },
                Command::LEFT(_numpixels) => {
                    if is_pen_down {
                        let color_index: usize = queries.color[1..].parse::<usize>().expect("Invalid color");
                        let (new_x, new_y) = Image::draw_simple_line(&mut image, x, y, direction + 270, length, unsvg::COLORS[color_index]).unwrap();
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    } else {
                        let (new_x, new_y) = unsvg::get_end_coordinates(x, y, direction + 270, length);
                        queries.xcor = format!("{}{}", "\"", new_x);
                        queries.ycor = format!("{}{}", "\"", new_y);
                    }
                },
                _ => {
                    return Err("Invalid Command".to_string());
                }
            }
        },
        Command::SETPENCOLOR(colorcode) => {
            let table = if is_in_procedure { procedure_args } else { variable_table };
            let result = parse_operation(colorcode, table)?;
            queries.color = result;
        },
        Command::TURN(degrees)
        | Command::SETHEADING(degrees) => {
            let table = if is_in_procedure { procedure_args } else { variable_table };
            let result = parse_operation(degrees, table)?;
            match command {
                Command::TURN(_degrees) => {
                    println!("turns by {}", result);
                },
                Command::SETHEADING(_degrees) => {
                    println!("set heading by {}", result);
                },
                _ => {
                    return Err("Invalid Command".to_string());
                }
            }
        },
        Command::SETX(location)
        | Command::SETY(location) => {
            let table = if is_in_procedure { procedure_args } else { variable_table };
            let result = parse_operation(location, table)?;
            match command {
                Command::SETX(_location) => {
                    println!("set x as {}", result);
                },
                Command::SETY(_location) => {
                    println!("set y as {}", result);
                },
                _ => {
                    return Err("Invalid Command".to_string());
                }
            }
        },
        Command::MAKE(variable_name, value) => {
            let table = if is_in_procedure { procedure_args } else { variable_table };
            let variable_value = parse_operation(value, table)?;
            println!("Inserted '{}' with value {} into the hash table", variable_name, variable_value);
            table.insert(variable_name.clone().replace("\"", ":"), variable_value);
        },
        Command::ADDASSIGN(variable_name, value) => {
            let lookup_key = variable_name.replace("\"", ":");
            match variable_table.get(&lookup_key) {
                Some(var) => {
                    let num = var[1..].parse::<f32>().expect("not a number");
                    let operation_result = parse_operation(value, variable_table)?;
                    let add_num = operation_result[1..].parse::<f32>().expect("not a number");

                    let result = num + add_num;
                    variable_table.insert(lookup_key, format!("{}{}", '\"', result));
                },
                None => {
                    return Err("variable not defined".to_string());
                }
            }
        },
        Command::IF(operation, commands) => {
            let table = if is_in_procedure { &procedure_args } else { &variable_table };
            let if_condition = parse_operation(operation, &table)?;
            if if_condition == "TRUE".to_string() {
                for command in commands.iter() {
                    execute_command(command, variable_table, dummy_procedures, is_in_procedure, procedure_args, queries, image)?;
                }
            }
        },
        Command::WHILE(operation, commands) => {
            while {
                // Limit the scope of the immutable borrow
                let table = if is_in_procedure { &procedure_args } else { &variable_table };
                parse_operation(operation, table)? == "TRUE".to_string()
            } {
                for command in commands.iter() {
                    // Now variable_table is not immutably borrowed in this scope
                    execute_command(command, variable_table, dummy_procedures, is_in_procedure, procedure_args, queries, image)?;
                }
            }
        },
        Command::PROCEDURE(procedure_name, params) => {
            // get procedure name
            let procedure = dummy_procedures.get(procedure_name).expect("msg");
            // get procedure defined args list
            let args = procedure.args.clone();
            // get procedure commands
            let commands = procedure.commands.clone();
            // initialise args table
            let mut args_table: HashMap<String, String> = HashMap::new();

            for (arg, param) in args.iter().zip(params.iter()) {
                let table = if is_in_procedure { &procedure_args } else { &variable_table };
                // let mut parameter = "".to_string();
                // if is_in_procedure {
                //     // it is possible that it is a variable, so loop up the tash table first
                //     parameter = procedure_args.get(param).expect("msg").clone();
                // } else {
                //     // it is possible that it is a variable, so loop up the tash table first
                //     parameter = variable_table.get(param).expect("msg").clone();
                // }
                match table.get(param) {
                    Some(parameter) => {
                        args_table.insert(arg.replace("\"", ":"), parameter.clone());
                    },
                    None => {
                        return Err("lookup key not found".to_string());
                    }
                }
            }

            for command in commands {
                if let Err(e) = execute_command(&command, variable_table, dummy_procedures, true, &mut args_table, queries, image) {
                    eprintln!("\x1b[31mError processing commands inside a procedure: {}\x1b[0m", e);
                    std::process::exit(1);
                }
            }
        },
    }
    Ok(())
}

fn parse_operation(operation: &Operation, variable_table: &HashMap<String, String>) -> Result<String, String> {
    match operation {
        Operation::BASE(value) => {
            match value.chars().next() {
                Some('\"') => {
                    value[1..].parse::<f32>()
                        .map_err(|_| "BASE operation: Not a number".to_string())
                        .and_then(|num| {
                            if num.is_finite() {
                                Ok(value.to_string())
                            } else {
                                Err("BASE operation: Number is not normal".to_string())
                            }
                        })
                },
                Some(':') => {
                    let lookup_key = value.clone();
                    variable_table.get(&lookup_key)
                        .cloned()
                        .ok_or_else(|| "BASE operation: Key error".to_string())
                },
                _ => Err("BASE operation: Unexpected value".to_string()),
            }
        },
        Operation::ADD(a, b) | Operation::SUBTRACT(a, b) | Operation::MULTIPLY(a, b) | Operation::DIVIDE(a, b)
        | Operation::EQUAL(a, b) | Operation::NOTEQUAL(a, b) | Operation::LESSTHAN(a, b) | Operation::GREATERTHAN(a, b)
        | Operation::AND(a, b) | Operation::OR(a, b) => {
            let left = parse_operation(&a, variable_table)?;
            let right = parse_operation(&b, variable_table)?;

            match operation {
                Operation::ADD(_a, _b) => {
                    let result = left[1..].parse::<f32>().expect("not a number") + right[1..].parse::<f32>().expect("not a number");

                    Ok(format!("{}{}", '\"', result))
                },
                Operation::SUBTRACT(_a, _b) => {
                    let result = left[1..].parse::<f32>().expect("not a number") - right[1..].parse::<f32>().expect("not a number");
        
                    Ok(format!("{}{}", '\"', result))
                },
                Operation::MULTIPLY(_a, _b) => {
                    let result = left[1..].parse::<f32>().expect("not a number") * right[1..].parse::<f32>().expect("not a number");
        
                    Ok(format!("{}{}", '\"', result))
                },
                Operation::DIVIDE(_a, _b) => {
                    if right[1..].parse::<f32>().expect("not a number") == 0.0 {
                        return Err("divide by 0".to_string());
                    }
        
                    let result = left[1..].parse::<f32>().expect("not a number") / right[1..].parse::<f32>().expect("not a number");
        
                    Ok(format!("{}{}", '\"', result))
                },
                Operation::EQUAL(_a, _b) => {
                    if (left[1..].parse::<f32>().expect("not a number") - right[1..].parse::<f32>().expect("not a number")).abs() < f32::EPSILON {
                        Ok("TRUE".to_string())
                    } else {
                        Ok("FALSE".to_string())
                    }
                },
                Operation::NOTEQUAL(_a, _b) => {
                    if (left[1..].parse::<f32>().expect("not a number") - right[1..].parse::<f32>().expect("not a number")).abs() < f32::EPSILON {
                        Ok("FALSE".to_string())
                    } else {
                        Ok("TRUE".to_string())
                    }
                },
                Operation::LESSTHAN(_a, _b) => {
                    if left[1..].parse::<f32>().expect("not a number") - right[1..].parse::<f32>().expect("not a number") < 0.0 {
                        Ok("TRUE".to_string())
                    } else {
                        Ok("FALSE".to_string())
                    }
                },
                Operation::GREATERTHAN(_a, _b) => {
                    if left[1..].parse::<f32>().expect("not a number") - right[1..].parse::<f32>().expect("not a number") > 0.0 {
                        Ok("TRUE".to_string())
                    } else {
                        Ok("FALSE".to_string())
                    }
                },
                Operation::AND(_a, _b) => {
                    let left_bool = match left.as_str() {
                        "TRUE" => true,
                        "FALSE" => false,
                        _ => return Err("unknown operator".to_string()),
                    };
        
                    let right_bool = match right.as_str() {
                        "TRUE" => true,
                        "FALSE" => false,
                        _ => return Err("unknown operator".to_string()),
                    };
        
                    if left_bool && right_bool {
                        Ok("TRUE".to_string())
                    } else {
                        Ok("FALSE".to_string())
                    }
                },
                Operation::OR(_a, _b) => {
                    let left_bool = match left.as_str() {
                        "TRUE" => true,
                        "FALSE" => false,
                        _ => return Err("unknown operator".to_string()),
                    };
        
                    let right_bool = match right.as_str() {
                        "TRUE" => true,
                        "FALSE" => false,
                        _ => return Err("unknown operator".to_string()),
                    };
        
                    if left_bool || right_bool {
                        Ok("TRUE".to_string())
                    } else {
                        Ok("FALSE".to_string())
                    }
                },
                _ => Err("msg".to_string()),
            }
        },
    }
}

fn extract_operations(operations: &Vec<&str>) -> Operation {
    let mut stack: Vec<Operation> = Vec::new();

    for operation in operations.iter().rev() {
        match *operation {
            "+" | "-" | "*" | "/" | "EQ" | "NE" | "AND" | "OR" | "GT" | "LT" => {
                let left = stack.pop().expect("Missing left operand");
                let right = stack.pop().expect("Missing right operand");
                let op = match *operation {
                    "+" => Operation::ADD(Box::new(left), Box::new(right)),
                    "-" => Operation::SUBTRACT(Box::new(left), Box::new(right)),
                    "*" => Operation::MULTIPLY(Box::new(left), Box::new(right)),
                    "/" => Operation::DIVIDE(Box::new(left), Box::new(right)),
                    "EQ" => Operation::EQUAL(Box::new(left), Box::new(right)),
                    "NE" => Operation::NOTEQUAL(Box::new(left), Box::new(right)),
                    "AND" => Operation::AND(Box::new(left), Box::new(right)),
                    "OR" => Operation::OR(Box::new(left), Box::new(right)),
                    "GT" => Operation::GREATERTHAN(Box::new(left), Box::new(right)),
                    "LT" => Operation::LESSTHAN(Box::new(left), Box::new(right)),
                    _ => panic!("unknown operator"),
                };
                stack.push(op);
            },
            _ => {
                if operation.starts_with('\"') {
                    if let Ok(_) = operation[1..].parse::<f32>() {
                    stack.push(Operation::BASE(operation.to_string()));
                    } else {
                        panic!("unexpected value type");
                    }
                } else {
                    stack.push(Operation::BASE(operation.to_string()));
                }
            }
        }
    }

    if stack.len() == 1 {
        stack.pop().expect("Invalid expression")
    } else {
        panic!("wrong number of arguments");
    }
    
}

fn parse_command(line: &str, dummy_procedures: &HashMap<String, DummyProcedure>) -> Result<Command, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    match parts[0] {
        "PENUP" | "PENDOWN" if parts.len() == 1 => Ok(match parts[0] {
            "PENUP" => Command::PENUP,
            "PENDOWN" => Command::PENDOWN,
            _ => unreachable!(), // We won't get here
        }),
        "FORWARD" | "BACK" | "RIGHT" | "LEFT" | "SETPENCOLOR" | "TURN" | "SETHEADING" | "SETX" | "SETY" if parts.len() > 1 => {
            let operations = parts[1..].to_vec();
            let extracted = extract_operations(&operations); // Make sure this function returns Result as well

            Ok(match parts[0] {
                "FORWARD" => Command::FORWARD(extracted),
                "BACK" => Command::BACK(extracted),
                "RIGHT" => Command::RIGHT(extracted),
                "LEFT" => Command::LEFT(extracted),
                "SETPENCOLOR" => Command::SETPENCOLOR(extracted),
                "TURN" => Command::TURN(extracted),
                "SETHEADING" => Command::SETHEADING(extracted),
                "SETX" => Command::SETX(extracted),
                "SETY" => Command::SETY(extracted),
                _ => unreachable!(), // We won't get here
            })
        },
        "MAKE" | "ADDASSIGN" if parts.len() > 2 => {
            let variable_name = parts[1];
            let operations = parts[2..].to_vec();
            let extracted = extract_operations(&operations); // Adjust for error handling

            Ok(match parts[0] {
                "MAKE" => Command::MAKE(variable_name.to_string(), extracted),
                "ADDASSIGN" => Command::ADDASSIGN(variable_name.to_string(), extracted),
                _ => unreachable!(), // We won't get here
            })
        },
        _ if dummy_procedures.contains_key(parts[0]) => {
            let args: Vec<String> = parts.get(1..).unwrap_or(&[]).iter().map(|&s| s.to_string()).collect();
            let procedure = dummy_procedures.get(parts[0]).ok_or_else(|| "Procedure not found".to_string())?;
            if args.len() == procedure.args.len() {
                Ok(Command::PROCEDURE(parts[0].to_string(), args))
            } else {
                Err("Number of parameters does not match".to_string())
            }
        },
        _ => Err("Invalid command or wrong number of arguments".to_string()),
    }
}

/// There are a few possible commands:
/// First is basic commands such as PENDOWN and FORWARD
/// Second is IF and WHILE as well as back bracket "]"
/// Third would be Procedure definition and calling
fn extract_commands(lines: &Vec<&str>, start: usize, dummy_procedures: &mut HashMap<String, DummyProcedure>) -> Result<(Vec<Command>, usize), String> {

    let mut commands: Vec<Command> = Vec::new();
    let mut i = start;

    while i < lines.len() {
        
        let parts: Vec<&str> = lines.get(i)
            .map(|line| line.split_whitespace())
            .unwrap()
            .collect();

        match parts.get(0) {
            Some(&"IF") => {
                if parts[parts.len() - 1] != "[" {
                    return Err("IF expression does not start with [".to_string());
                }

                let raw_operations = parts[1..parts.len()-1].to_vec();
                let (block_commands, new_index) = extract_commands(lines, i + 1, dummy_procedures)?;
                i = new_index;
                let operations = extract_operations(&raw_operations);

                commands.push(Command::IF(operations, block_commands));
            },
            Some(&"WHILE") => {
                if parts[parts.len() - 1] != "[" {
                    return Err("WHILE expression does not start with [".to_string());
                }

                let raw_operations = parts[1..parts.len()-1].to_vec();
                let (block_commands, new_index) = extract_commands(lines, i + 1, dummy_procedures)?;
                i = new_index;
                let operations = extract_operations(&raw_operations);

                commands.push(Command::WHILE(operations, block_commands));
            },
            Some(&"]") => {
                return Ok((commands, i));
            },
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
                dummy_procedures.insert(procedure_name, DummyProcedure {args, commands: block_commands});
            },
            Some(&"END") => {
                return Ok((commands, i));
            },
            _ => {
                match parse_command(&lines[i], dummy_procedures) {
                    Ok(cmd) => commands.push(cmd),
                    Err(e) => return Err(e),
                }
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

    let file_content = match read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => {
            return Err(());
        },
    };

    let mut dummy_procedures: HashMap<String, DummyProcedure> = HashMap::new();
    let mut variable_table: HashMap<String, String> = HashMap::new();

    let mut lines: Vec<&str> = Vec::new();
    
    for line in file_content.lines() {
        let line = line.trim();
        if line.starts_with("// ") || line == "" {
            continue;
        }
        lines.push(line);
    }

    let commands = match extract_commands(&lines, 0, &mut dummy_procedures) {
        Ok((commands, _)) => {
            commands
        },
        Err(e) => {
            eprintln!("Error processing commands: {}", e);
            std::process::exit(1);
        }
    };

    let mut image = Image::new(width, height);

    let mut queries_struct = QueriesStruct { 
        xcor: format!("{}{}", "\"", (width/2).to_string()),
        ycor: format!("{}{}", "\"", (height/2).to_string()),
        heading: "\"0".to_string(),
        color: "\"7".to_string(),
        is_pen_down: "FALSE".to_string(),
    };

    let mut map:HashMap<String, String> = HashMap::new();
    for command in commands.iter() {
        if let Err(e) = execute_command(command, &mut variable_table, &mut dummy_procedures, false, &mut map, &mut queries_struct, &mut image) {
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
