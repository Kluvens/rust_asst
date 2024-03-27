use rslogo::utils::{Args, QueriesStruct, extract_commands};
use std::collections::HashMap;
use rslogo::procedures::DummyProcedure;
use rslogo::executer::execute_command;
use unsvg::Image;
use clap::Parser;
use std::fs::read_to_string;

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

    match image_path.extension().and_then(|s| s.to_str()) {
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
