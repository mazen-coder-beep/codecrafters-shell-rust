#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::env::set_current_dir;
use std::path::Path;
use std::env::args;

fn find_executable_in_path(command: &str) -> Option<PathBuf> {
    let path_env = env::var("PATH").ok()?;
    for dir in path_env.split(':') {
        let full_path = PathBuf::from(dir).join(command);
        if full_path.is_file() {
            let meta = std::fs::metadata(&full_path).ok()?;
            if meta.permissions().mode() & 0o111 != 0 {
                return Some(full_path);
            }
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();

        if command == "exit" {
            break;

        }   else if command == "cd" || command.starts_with("cd "){
            let target = if command == "cd"{
                env::var("HOME").unwrap_or_else(|_| "/".to_string())
            } else {
                command[3..].trim().to_string()
            };

            let path = if target.starts_with("~") {
                let home = env::var("HOME").unwrap_or_else(|_|"/".to_string());
                PathBuf::from(&target.replacen('~', &home , 1))
            } else {
                PathBuf::from(&target)
            };
        }
            else if command == "pwd" {
            match std::env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: {}", e),
            }
        } 
            else if command.starts_with("type ") {
            let arg = &command[5..];
            if arg == "echo" || arg == "exit" || arg == "type" || arg == "pwd" || arg == "cd" || arg = "/" {
                println!("{} is a shell builtin", arg);
            } else if let Some(path) = find_executable_in_path(arg) {
                println!("{} is {}", arg, path.display());
            } else {
                println!("{}: not found", arg);
            }
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);

        } else {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if !parts.is_empty() {
                let cmd_name = parts[0];
                let args = &parts[1..];
                if find_executable_in_path(cmd_name).is_some() {
                    Command::new(cmd_name) 
                    .args(args)
                    .status()
                    .expect("Failed to execute");
                } else {
                    println!("{}: command not found", command);
                }
            }
         
        }
        
            

        }
} 

