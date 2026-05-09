mod lexer;
mod parser;
mod ast;
mod typechecker;
mod ir;
mod codegen;
mod error;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "shard")]
#[command(about = "Shard Programming Language Compiler")]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        #[arg(help = "Input file")]
        input: PathBuf,
        #[arg(short, long, help = "Output file")]
        output: Option<PathBuf>,
        #[arg(short, long, help = "Optimization level (0-3)", default_value = "2")]
        opt_level: u8,
        #[arg(short, long, help = "Enable JIT mode")]
        jit: bool,
    },
    Run {
        #[arg(help = "Input file")]
        input: PathBuf,
        #[arg(short, long, help = "Optimization level (0-3)", default_value = "2")]
        opt_level: u8,
        #[arg(short, long, help = "Enable JIT mode")]
        jit: bool,
    },
    Check {
        #[arg(help = "Input file")]
        input: PathBuf,
    },
    Repl,
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output, opt_level, jit } => {
            compile_file(&input, output.as_ref(), opt_level, jit);
        }
        Commands::Run { input, opt_level, jit } => {
            run_file(&input, opt_level, jit);
        }
        Commands::Check { input } => {
            check_file(&input);
        }
        Commands::Repl => {
            start_repl();
        }
        Commands::Version => {
            println!("Shard Compiler 0.2.0");
            println!("Author: MelvinSGjr (MelvinMod)");
            println!("Platform: {}", std::env::consts::OS);
            println!("Features: Native, JIT, Memory Safety, Error Handling");
        }
    }
}

fn compile_file(input: &PathBuf, output: Option<&PathBuf>, opt_level: u8, jit: bool) {
    println!("Compiling {}...", input.display());
    
    let source = std::fs::read_to_string(input).expect("Failed to read source file");
    
    let tokens = lexer::lex(&source).unwrap_or_else(|e| {
        e.print();
        std::process::exit(1);
    });
    println!("  [-] Lexed {} tokens", tokens.len());
    
    let ast = parser::parse(tokens).unwrap_or_else(|e| {
        e.print();
        std::process::exit(1);
    });
    println!("  [-] Parsed {} nodes", ast.nodes.len());
    
    typechecker::check(&ast).unwrap_or_else(|e| {
        e.print();
        std::process::exit(1);
    });
    println!("  [-] Type checking passed");
    
    let ir = ir::generate(&ast).unwrap_or_else(|e| {
        e.print();
        std::process::exit(1);
    });
    println!("  [-] Generated IR");
    
    let output_path = output.cloned().unwrap_or_else(|| PathBuf::from("a.out"));
    
    if jit {
        println!("  [+] JIT compilation enabled");
        // JIT compilation would go here
    } else {
        codegen::compile(&ir, &output_path, opt_level).unwrap_or_else(|e| {
            e.print();
            std::process::exit(1);
        });
    }
    
    println!("  [+] Compiled successfully: {}", output_path.display());
}

fn run_file(input: &PathBuf, opt_level: u8, jit: bool) {
    println!("Running {}...", input.display());
    
    let input_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(input.parent().unwrap_or(PathBuf::from(".").as_path()));
    
    let output_path = input_dir.join("a.out");
    
    compile_file(input, Some(&output_path), opt_level, jit);
    
    if !output_path.exists() {
        eprintln!("Error: Compiled file not found at {}", output_path.display());
        std::process::exit(1);
    }
    
    let status = std::process::Command::new(&output_path)
        .status();
    
    match status {
        Ok(s) => {
            std::fs::remove_file(&output_path).ok();
            if !s.success() {
                std::process::exit(s.code().unwrap_or(1));
            }
        }
        Err(e) => {
            std::fs::remove_file(&output_path).ok();
            eprintln!("Error: Could not run compiled program: {}", e);
            std::process::exit(1);
        }
    }
}

fn check_file(input: &PathBuf) {
    println!("Checking {}...", input.display());
    
    let source = std::fs::read_to_string(input).expect("Failed to read source file");
    let tokens = lexer::lex(&source).expect("Lexing failed");
    let ast = parser::parse(tokens).expect("Parsing failed");
    typechecker::check(&ast).expect("Type checking failed");
    
    println!("  [+] All checks passed!");
}

fn start_repl() {
    println!("Shard REPL - Type 'exit' to quit");
    println!("Try: say \"Hello, World!\"");
    
    loop {
        print!("shard> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        if input == "exit" || input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match lexer::lex(input) {
            Ok(tokens) => {
                match parser::parse(tokens) {
                    Ok(ast) => {
                        match typechecker::check(&ast) {
                            Ok(_) => {
                                println!("  OK");
                            }
                            Err(e) => {
                                e.print();
                            }
                        }
                    }
                    Err(e) => {
                        e.print();
                    }
                }
            }
            Err(e) => {
                e.print();
            }
        }
    }
}
