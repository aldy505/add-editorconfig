use std::{
    env,
    fs::{self, File},
    io::{self, Error, Write},
    path::{Path, PathBuf},
};
extern crate dirs;

#[derive(Debug)]
struct Settings {
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

impl Settings {
    fn new() -> Self {
        Settings {
            end_of_line: String::from("lf"),
            indent_size: 4,
            indent_style: String::from("space"),
            tab_width: 4,
            charset: String::from("utf-8"),
            root: true,
            trim_trailing_whitespace: true,
            insert_final_newline: true,
            max_line_length: 80,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_path: PathBuf = Path::new(".editorconfig").to_path_buf();

    // Check if the user supplied any args
    if args.len() > 1 {
        match args[1].as_str() {
            "default" => {
                if let Some(home_dir) = dirs::home_dir() {
                    let default_path = home_dir.join(".editorconfig");
                    let is_default_cfg_exists = Path::new(default_path.as_os_str()).exists();

                    // Check if there is default editorconfig
                    // already exists on `~/.editorconfig`
                    if !is_default_cfg_exists {
                        println!("Looks like there is no default .editorconfig on your home directory. Creating one..");
                        let settings = get_user_settings();

                        // If not, create one
                        match create_config(&default_path, &settings) {
                            Ok(_) => println!(
                                "Successfully created .editorconfig on your home directory."
                            ),
                            Err(e) => println!("Error: {}", e),
                        }

                        match create_config(&current_path, &settings) {
                            Ok(_) => {
                                println!("Successfully created .editorconfig on current directory.")
                            }
                            Err(e) => println!("Error: {}", e),
                        }
                    }

                    // If there is, print a message
                    println!("Using default .editorconfig...\n");
                    match fs::read_to_string(default_path) {
                        Ok(s) => match create_config(&current_path, &parse_settings(s)) {
                            Ok(_) => println!("Successfully created .editorconfig"),
                            Err(e) => println!("Error: {}", e),
                        },
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Failed to get home directory. Exiting...");
                }
                return;
            }
            "version" | "--version" | "-v" => {
                println!("add-editorconfig 0.1.0");
                return;
            }
            "help" | "--help" | "-h" | _ => {
                println!(
                    "
add-editorconfig
Small and simple CLI app to generate .editorconfig based on a given settings.

Usage:
    add-editorconfig         - Create an .editorconfig in the current directory
    add-editorconfig default - Create an .editorconfig with the default config
                               from .editorconfig that exists on the home directory.
    add-editorconfig help    - Print this help command
    add-editorconfig version - Show current version

If you had encountered with any bugs, please open an issue at:
https://github.com/aldy505/add-editorconfig
                       "
                );
                return;
            }
        }
    }

    let settings = get_user_settings();
    match create_config(&current_path, &settings) {
        Ok(_) => println!("Successfully created .editorconfig"),
        Err(e) => println!("Error: {}", e),
    }
}

fn prompt(text: &str, temp: &mut String) {
    print!("{}", text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(temp).unwrap();
}

fn get_user_settings() -> Settings {
    let mut settings = Settings::new();
    let mut temp = String::new();

    println!("Fill the config with the provided options.");
    println!("Entering nothing will set the parameter to its default value.\n");

    // indent_style
    prompt("Indentation style (space / tabs): ", &mut temp);
    settings.indent_style = match temp.trim() {
        "tabs" => String::from("tabs"),
        _ => String::from("space"),
    };
    temp = String::from("");

    // indent_size
    prompt("Indent size (number): ", &mut temp);
    if let Ok(indent_size) = temp.trim().parse::<i32>() {
        settings.indent_size = indent_size;
    }
    temp = String::from("");

    // tab_width
    prompt("Tab width (number): ", &mut temp);
    settings.tab_width = match temp.trim().parse::<i32>() {
        Ok(tab_width) => tab_width,
        Err(_) => 4,
    };
    temp = String::from("");

    // tab_width
    prompt("End of line (lf / crlf / cr): ", &mut temp);
    settings.end_of_line = match temp.trim() {
        "crlf" => String::from("crlf"),
        "cr" => String::from("cr"),
        "lf" | _ => String::from("lf"),
    };
    temp = String::from("");

    // charset
    prompt(
        "Charset (latin1 / utf-8 / utf-16be / utf-16le / utf-8-bom): ",
        &mut temp,
    );
    settings.charset = match temp.trim() {
        "latin1" => String::from("latin1"),
        "utf-16be" => String::from("utf-16be"),
        "utf-16le" => String::from("utf-16le"),
        "utf-8-bom" => String::from("utf-8-bom"),
        "utf-8" | _ => String::from("utf-8"),
    };
    temp = String::from("");

    // trim_trailing_whitespace
    prompt("Trim trailing whitespace (true / false): ", &mut temp);
    settings.trim_trailing_whitespace = match temp.trim() {
        "true" => true,
        _ => false,
    };
    temp = String::from("");

    // insert_final_newline
    prompt("Insert final newline (true / false): ", &mut temp);
    settings.insert_final_newline = match temp.trim() {
        "true" => true,
        _ => false,
    };
    temp = String::from("");

    // max_line_length
    prompt("Max line length (number): ", &mut temp);
    settings.max_line_length = match temp.trim().parse::<i32>() {
        Ok(max_len) => max_len,
        Err(_) => 80,
    };

    print!("\n");

    settings
}

fn create_config(path: &PathBuf, settings: &Settings) -> Result<(), Error> {
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
        format!(
            "trim_trailing_whitespace = {}",
            settings.trim_trailing_whitespace
        ),
        format!("insert_final_newline = {}", settings.insert_final_newline),
        format!("max_line_length = {}", settings.max_line_length),
    ];

    file.write_all(config.join("\n").as_bytes())
}

fn parse_settings(str: String) -> Settings {
    let s: Vec<&str> = str.split_terminator("\n").collect();
    let mut settings: Settings = Settings::new();

    for x in s.iter() {
        // biar bisa kek destructure doang ribet banget elah
        let (key, value) = match &x.split("=").collect::<Vec<&str>>()[..] {
            &[key, value] => (key.trim(), value.trim()),
            _ => continue, // not a valid key-value pair, we'll skip
        };

        match key {
            "end_of_line" => settings.end_of_line = value.to_string(),
            "indent_size" => settings.indent_size = value.parse::<i32>().unwrap(),
            "indent_style" => settings.indent_style = value.to_string(),
            "tab_width" => settings.tab_width = value.parse::<i32>().unwrap(),
            "charset" => settings.charset = value.to_string(),
            "root" => settings.root = value.to_string().parse::<bool>().unwrap(),
            "trim_trailing_whitespace" => {
                settings.trim_trailing_whitespace = value.parse::<bool>().unwrap()
            }
            "insert_final_newline" => {
                settings.insert_final_newline = value.parse::<bool>().unwrap();
            }
            "max_line_length" => settings.max_line_length = value.parse::<i32>().unwrap(),
            _ => continue,
        }
    }

    settings
}
