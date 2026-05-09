use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub hint: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Syntax,
    TypeMismatch,
    Memory,
    Runtime,
    Parse,
}

impl Error {
    pub fn new(kind: ErrorKind, file: &str, line: usize, column: usize, message: &str) -> Self {
        Self {
            kind,
            file: file.to_string(),
            line,
            column,
            message: message.to_string(),
            hint: None,
            context: None,
        }
    }

    pub fn with_hint(mut self, hint: &str) -> Self {
        self.hint = Some(hint.to_string());
        self
    }

    pub fn with_context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }

    pub fn print(&self) {
        self.print_colored();
    }

    fn print_colored(&self) {
        let red = "\x1b[31m";
        let yellow = "\x1b[33m";
        let cyan = "\x1b[36m";
        let blue = "\x1b[34m";
        let reset = "\x1b[0m";

        match self.kind {
            ErrorKind::Syntax => {
                println!("{}Oops!{} I couldn't understand this line.", red, reset);
            }
            ErrorKind::TypeMismatch => {
                println!("{}Type mismatch{} — I feel a bit confused.", red, reset);
            }
            ErrorKind::Memory => {
                println!("{}Memory error{}: borrowed value does not live long enough.", red, reset);
            }
            ErrorKind::Parse => {
                println!("{}Parse error{}: {}", red, reset, self.message);
            }
            ErrorKind::Runtime => {
                println!("{}Runtime error{}: {}", red, reset, self.message);
            }
        }

        println!("{}--> {}:{}:{}{}", cyan, self.file, self.line, self.column, reset);
        println!("{}|{}", blue, reset);
        println!("{}{} | {}{}", blue, self.line, self.context.as_deref().unwrap_or(""), reset);
        println!("{}|{} {}", blue, reset, " ".repeat(self.column - 1));
        println!("{}|{} {}{}{}^{}{}", blue, reset, " ".repeat(self.column - 1), yellow, reset, reset);

        if let Some(ref hint) = self.hint {
            println!("{}= help: {}{}{}", cyan, hint, reset, "");
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
