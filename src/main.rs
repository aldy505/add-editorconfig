use std::{env, fs::{self, File}, io::{self, Error, Write}, path::{Path, PathBuf}};
extern crate dirs;

#[derive(Debug,Clone)]
pub struct Settings {
    end_of_line: String,
    indent_size: i32,
    indent_style: String,
    tab_width: i32,
    charset: String,
    root: bool,
    trim_trailing_whitespace: bool,
    insert_final_newline: bool,
    max_line_length: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_path: PathBuf = Path::new(".editorconfig").to_path_buf();

    // Check if the user supplied any args
    if args.len() > 1 {
        if args[1] == "default" {
            let default_path: PathBuf = dirs::home_dir().unwrap().join(".editorconfig");
            // Check if there is default editorconfig
            // already exists on `~/.editorconfig`
            if !Path::new(default_path.as_os_str()).exists() {
                println!("Looks like there is no default .editorconfig on your home directory. Creating one..");
                let p: Settings = prompt();
                // If not, create one
                match create_config(default_path, p.clone()) {
                    Ok(_) => println!("Successfully created .editorconfig on your home directory."),
                    Err(e) => println!("Error: {}", e)
                }

                match create_config(current_path, p.clone()) {
                    Ok(_) => println!("Successfully created .editorconfig on current directory."),
                    Err(e) => println!("Error: {}", e)
                }

                return;
            }

            // If there is, print a message
            println!("Using default .editorconfig...\n");
            match fs::read_to_string(default_path) {
                Ok(s) => {
                    match create_config(current_path, parse_settings(s)) {
                        Ok(_) => println!("Successfully created .editorconfig"),
                        Err(e) => println!("Error: {}", e)
                    }
                },
                Err(e) => println!("Error: {}", e),
            }
            return;
        } else if args[1] == "version" || args[1] == "--version" || args[1] == "-v" {
          println!("add-editorconfig 0.1.0");
          return;
        } else if args[1] == "help" || args[1] == "--help" || args[1] == "-h" {
          println!("add-editorconfig");
          println!("Small and simple CLI app to generate .editorconfig based on a given settings.\n");
          println!("Usage:");
          println!("    add-editorconfig         - Create an .editorconfig in the current directory");
          println!("    add-editorconfig default - Create an .editorconfig with the default config");
          println!("                               from .editorconfig that exists on the home directory.");
          println!("    add-editorconfig help    - Print this help command");
          println!("    add-editorconfig version - Show current version\n");
          println!("If you had encountered with any bugs, please open an issue at:");
          println!("https://github.com/aldy505/add-editorconfig");
          return;
        }

    }

    let p: Settings = prompt();
    match create_config(current_path, p) {
        Ok(_) => println!("Successfully created .editorconfig"),
        Err(e) => println!("Error: {}", e)
    }
}

pub fn default_settings() -> Settings {
    return Settings {
        end_of_line: String::from("lf"),
        indent_size: 4,
        indent_style: String::from("space"),
        tab_width: 4,
        charset: String::from("utf-8"),
        root: true,
        trim_trailing_whitespace: true,
        insert_final_newline: true,
        max_line_length: 80
    };
}

pub fn prompt() -> Settings {
    let mut settings: Settings = Settings {
        ..default_settings()
    };
    let mut temp = String::new();
    println!("Fill the config with the provided options.");
    println!("Entering nothing will set the parameter to its' default value.\n");

    // indent_style
    print!("Indentation style (space / tabs): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim() == "tabs" {
        settings.indent_style = String::from("tabs");
    } else {
        settings.indent_style = String::from("space");
    }
    temp = String::from("");

    // indent_size
    print!("Indent size (number): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim().parse::<i32>().is_ok() {
        settings.indent_size = temp.trim().parse::<i32>().unwrap();
    } else {
        settings.indent_size = 4;
    }
    temp = String::from("");


    // tab_width
    print!("Tab width (number): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim().parse::<i32>().is_ok() {
        settings.tab_width = temp.trim().parse::<i32>().unwrap();
    } else {
        settings.tab_width = 4;
    }
    temp = String::from("");

    // tab_width
    print!("End of line (lf / crlf / cr): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim() == "lf" {
        settings.end_of_line = String::from("lf");
    } else if temp.trim() == "crlf" {
        settings.end_of_line = String::from("crlf");
    } else if temp.trim() == "cr" {
        settings.end_of_line = String::from("cr");
    } else {
        settings.end_of_line = String::from("lf");
    }
    temp = String::from("");

    // charset
    print!("Charset (latin1 / utf-8 / utf-16be / utf-16le / utf-8-bom): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim() == "latin1" {
        settings.charset = String::from("latin1");
    } else if temp.trim() == "utf-8" {
        settings.charset = String::from("utf-8");
    } else if temp.trim() == "utf-16be" {
        settings.charset = String::from("utf-16be");
    } else if temp.trim() == "utf-16le" {
        settings.charset = String::from("utf-16le");
    } else if temp.trim() == "utf-8-bom" {
        settings.charset = String::from("utf-8-bom");
    } else {
        settings.charset = String::from("utf-8");
    }
    temp = String::from("");

    // trim_trailing_whitespace
    print!("Trim trailing whitespace (true / false): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim() == "true" {
        settings.trim_trailing_whitespace = true;
    } else {
        settings.trim_trailing_whitespace = false;
    }
    temp = String::from("");

    // insert_final_newline
    print!("Insert final newline (true / false): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim() == "true" {
        settings.insert_final_newline = true;
    } else {
        settings.insert_final_newline = false;
    }
    temp = String::from("");

    // max_line_length
    print!("Max line length (number): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut temp).unwrap();
    if temp.trim().parse::<i32>().is_ok() {
        settings.max_line_length = temp.trim().parse::<i32>().unwrap();
    } else {
        settings.max_line_length = 80;
    }
    print!("\n");

    return settings;
}

pub fn create_config(path: PathBuf, settings: Settings) -> Result<(), Error> {
    let mut file = File::create(path)?;
    let config: Vec<String> = vec![
        format!("root = {}", settings.root),
        format!(""),
        format!("[*]"),
        format!("end_of_line = {}", settings.end_of_line),
        format!("indent_size = {}", settings.indent_size),
        format!("indent_style = {}", settings.indent_style),
        format!("tab_width = {}", settings.tab_width),
        format!("charset = {}", settings.charset),
        format!("trim_trailing_whitespace = {}", settings.trim_trailing_whitespace),
        format!("insert_final_newline = {}", settings.insert_final_newline),
        format!("max_line_length = {}", settings.max_line_length),
    ];
    let w = file.write_all(config.join("\n").as_bytes());
    return w;
}

pub fn parse_settings(str: String) -> Settings {
    let s: Vec<&str> = str.split_terminator("\n").collect();
    let mut settings: Settings = Settings {
        ..default_settings()
    };
    for x in s.iter() {
        let kv = x.split("=").collect::<Vec<&str>>();
        match kv[0].trim() {
            "end_of_line" => {
                settings.end_of_line = kv[1].trim().to_string();
            },
            "indent_size" => {
                settings.indent_size = kv[1].trim().parse::<i32>().unwrap();
            },
            "indent_style" => {
                settings.indent_style = kv[1].trim().to_string();
            },
            "tab_width" => {
                settings.tab_width = kv[1].trim().parse::<i32>().unwrap();
            },
            "charset" => {
                settings.charset = kv[1].trim().to_string();
            },
            "root" => {
                settings.root = kv[1].trim().to_string().parse::<bool>().unwrap();
            },
            "trim_trailing_whitespace" => {
                settings.trim_trailing_whitespace = kv[1].trim().parse::<bool>().unwrap();
            },
            "insert_final_newline" => {
                settings.insert_final_newline = kv[1].trim().parse::<bool>().unwrap();
            },
            "max_line_length" => {
                settings.max_line_length = kv[1].trim().parse::<i32>().unwrap();
            },
            _ => {
                continue;
            }
        }
    };

    return settings;
}
