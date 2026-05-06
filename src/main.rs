mod lexer;
mod parser;
mod ast;
mod typechecker;
mod ir;
mod codegen;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "shardc")]
#[command(about = "Shard Programming Language Compiler")]
#[command(version = "0.1.0")]
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
    },
    Run {
        #[arg(help = "Input file")]
        input: PathBuf,
        #[arg(short, long, help = "Optimization level (0-3)", default_value = "2")]
        opt_level: u8,
    },
    Check {
        #[arg(help = "Input file")]
        input: PathBuf,
    },
    Version,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output, opt_level } => {
            compile_file(&input, output.as_deref(), opt_level);
        }
        Commands::Run { input, opt_level } => {
            run_file(&input, opt_level);
        }
        Commands::Check { input } => {
            check_file(&input);
        }
        Commands::Version => {
            println!("Shard Compiler 0.1.0");
            println!("Author: MelvinSGjr (MelvinMod)");
            println!("Platform: {}", std::env::consts::OS);
        }
    }
}

fn compile_file(input: &PathBuf, output: Option<&PathBuf>, opt_level: u8) {
    println!("Compiling {}...", input.display());
    
    let source = std::fs::read_to_string(input).expect("Failed to read source file");
    
    let tokens = lexer::lex(&source).expect("Lexing failed");
    println!("  [-] Lexed {} tokens", tokens.len());
    
    let ast = parser::parse(tokens).expect("Parsing failed");
    println!("  [-] Parsed {} nodes", ast.nodes.len());
    
    typechecker::check(&ast).expect("Type checking failed");
    println!("  [-] Type checking passed");
    
    let ir = ir::generate(&ast).expect("IR generation failed");
    println!("  [-] Generated IR");
    
    let output_path = output.unwrap_or_else(|| PathBuf::from("a.out"));
    codegen::compile(&ir, &output_path, opt_level).expect("Code generation failed");
    
    println!("  [+] Compiled successfully: {}", output_path.display());
}

fn run_file(input: &PathBuf, opt_level: u8) {
    println!("Running {}...", input.display());
    compile_file(input, None, opt_level);
    
    let output_path = PathBuf::from("a.out");
    let status = std::process::Command::new(&output_path)
        .status()
        .expect("Failed to execute");
    
    std::fs::remove_file(&output_path).ok();
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
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

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "shardc")]
#[command(about = "Shard Programming Language Compiler")]
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
    },
    Run {
        #[arg(help = "Input file")]
        input: PathBuf,
        #[arg(short, long, help = "Optimization level (0-3)", default_value = "2")]
        opt_level: u8,
    },
    Check {
        #[arg(help = "Input file")]
        input: PathBuf,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { input, output, opt_level } => {
            compile_file(&input, output.as_deref(), opt_level);
        }
        Commands::Run { input, opt_level } => {
            run_file(&input, opt_level);
        }
        Commands::Check { input } => {
            check_file(&input);
        }
    }
}

fn compile_file(input: &PathBuf, output: Option<&PathBuf>, opt_level: u8) {
    println!("Compiling {}...", input.display());
    
    // Read source
    let source = std::fs::read_to_string(input).expect("Failed to read source file");
    
    // Lexing
    let tokens = lexer::lex(&source).expect("Lexing failed");
    println!("  [-] Lexed {} tokens", tokens.len());
    
    // Parsing
    let ast = parser::parse(&tokens).expect("Parsing failed");
    println!("  [-] Parsed {} nodes", ast.nodes.len());
    
    // Type checking
    typechecker::check(&ast).expect("Type checking failed");
    println!("  [-] Type checking passed");
    
    // IR generation
    let ir = ir::generate(&ast).expect("IR generation failed");
    println!("  [-] Generated IR");
    
    // Code generation
    let output_path = output.unwrap_or_else(|| PathBuf::from("a.out"));
    codegen::compile(&ir, &output_path, opt_level).expect("Code generation failed");
    
    println!("  [+] Compiled successfully: {}", output_path.display());
}

fn run_file(input: &PathBuf, opt_level: u8) {
    println!("Running {}...", input.display());
    compile_file(input, None, opt_level);
    
    let output_path = PathBuf::from("a.out");
    let status = std::process::Command::new(&output_path)
        .status()
        .expect("Failed to execute");
    
    std::fs::remove_file(&output_path).ok();
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn check_file(input: &PathBuf) {
    println!("Checking {}...", input.display());
    
    let source = std::fs::read_to_string(input).expect("Failed to read source file");
    let tokens = lexer::lex(&source).expect("Lexing failed");
    let ast = parser::parse(&tokens).expect("Parsing failed");
    typechecker::check(&ast).expect("Type checking failed");
    
    println!("  [+] All checks passed!");
}
