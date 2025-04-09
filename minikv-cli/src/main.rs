use minikv_core::{Command, Response};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "minikv-cli")]
#[command(about = "A CLI for MiniKV key-value store", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, default_value = "127.0.0.1:4000")]
    server: String,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
}

fn main() {
    let cli = Cli::parse();

    let mut stream = match TcpStream::connect(&cli.server) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} {}", "[CONNECTION ERROR]".red().bold(), e);
            return;
        }
    };

    if let Some(command) = cli.command {
        let cmd = match command {
            Commands::Set { key, value } => Command::Set { key, value },
            Commands::Get { key } => Command::Get { key },
            Commands::Del { key } => Command::Del { key },
        };

        if let Err(e) = send_command(&mut stream, &cmd) {
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
        }
    } else {
        run_repl(&mut stream);
    }
}

fn run_repl(stream: &mut TcpStream) {
    println!("MiniKV REPL started. Type `exit` or `quit` to leave.");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("minikv> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let input = input.trim();
        if input == "exit" || input == "quit" {
            break;
        }

        match parse_input(input) {
            Some(cmd) => {
                if let Err(e) = send_command(stream, &cmd) {
                    eprintln!("{} {}", "[ERROR]".red().bold(), e);
                }
            }
            None => {
                println!("Invalid command. Use: set <key> <value> | get <key> | del <key>");
            }
        }
    }
}

fn send_command(stream: &mut TcpStream, command: &Command) -> io::Result<()> {
    let serialized = serde_json::to_string(command).expect("Failed to serialize command");
    stream.write_all(serialized.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    println!("{serialized}");

    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone()?);
    reader.read_line(&mut response)?;

    match serde_json::from_str::<Response>(&response) {
        Ok(Response::Ok { value }) => {
            println!("{} {}", "[OK]".green().bold(), value.cyan());
        }
        Ok(Response::Error { message }) => {
            println!("{} {}", "[ERROR]".red().bold(), message.yellow());
        }
        Err(_) => {
            println!("{}", response);
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> Option<Command> {
    let args: Vec<&str> = input.split_whitespace().collect();
    if args.is_empty() {
        return None;
    }

    match args[0] {
        "set" if args.len() >= 3 => Some(Command::Set {
            key: args[1].into(),
            value: args[2..].join(" "),
        }),
        "get" if args.len() == 2 => Some(Command::Get {
            key: args[1].into(),
        }),
        "del" if args.len() == 2 => Some(Command::Del {
            key: args[1].into(),
        }),
        _ => None,
    }
}
