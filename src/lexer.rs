use crate::ast::{Literal, Type};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
    
    // Keywords
    Let,
    Mut,
    Const,
    Fn,
    Return,
    If,
    Else,
    Loop,
    While,
    For,
    In,
    Break,
    Continue,
    Match,
    Struct,
    Enum,
    Impl,
    Use,
    As,
    Pub,
    Mod,
    Type,
    Unsafe,
    Async,
    Await,
    Move,
    Ref,
    Box,
    Vec,
    Null,
    True,
    False,
    Self_,
    Underscore,
    Yield,
    Type,
    Delegate,
    Alias,
    
    // AI Keywords
    Neural,
    Network,
    Layer,
    Train,
    Inference,
    Tensor,
    Model,
    Dataset,
    Epoch,
    Batch,
    LearningRate,
    Optimizer,
    Loss,
    Accuracy,
    Embedding,
    Attention,
    Transformer,
    RNN,
    CNN,
    LSTM,
    GRU,
    
    // Crystal-like
    Macro,
    Fun,
    Lib,
    Struct,
    Record,
    Union,
    UnionType,
    Enum,
    Annotation,
    UnionOf,
    Is,
    As,
    RespondsTo,
    Include,
    Extend,
    Primitive,
    Forward,
    Abstract,
    Final,
    Def,
    Class,
    Property,
    Getter,
    Setter,
    Do,
    End,
    When,
    Unless,
    Then,
    Nil,
    True,
    False,
    And,
    Or,
    Not,
    In,
    Is,
    As,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Bang,
    Eq,
    Lt,
    Gt,
    Dot,
    DoubleDot,
    Arrow,
    DoubleArrow,
    FatArrow,
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semicolon,
    
    // Assignment
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpersandAssign,
    PipeAssign,
    CaretAssign,
    ShlAssign,
    ShrAssign,
    
    // Comparison
    EqEq,
    Ne,
    LtEq,
    GtEq,
    Shl,
    Shr,
    
    // Special
    Ident(String),
    TypeIdent(String),
    Newline,
    Comment(String),
    DocComment(String),
    Eof,
    
    // Error
    Error(String),
}

pub fn lex(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    
    while let Some(c) = chars.next() {
        match c {
            // Whitespace
            ' ' | '\t' | '\r' => continue,
            '\n' => {
                tokens.push(Token::Newline);
                line += 1;
            }
            
            // Single-line comment
            '/' if chars.peek() == Some(&'/') => {
                let mut comment = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    comment.push(chars.next().unwrap());
                }
                tokens.push(Token::Comment(comment));
            }
            
            // Multi-line comment
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut comment = String::new();
                let mut depth = 1;
                while depth > 0 {
                    match chars.next() {
                        Some('/') if chars.peek() == Some(&'*') => {
                            chars.next();
                            depth += 1;
                            comment.push_str("/*");
                        }
                        Some('*') if chars.peek() == Some(&'/') => {
                            chars.next();
                            depth -= 1;
                            if depth > 0 {
                                comment.push_str("*/");
                            }
                        }
                        Some(ch) => comment.push(ch),
                        None => return Err("Unclosed multi-line comment".to_string()),
                    }
                }
                tokens.push(Token::Comment(comment));
            }
            
            // Doc comment
            '/' if chars.peek() == Some(&'/') && chars.clone().skip(1).next() == Some('/') => {
                chars.next();
                let mut comment = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    comment.push(chars.next().unwrap());
                }
                tokens.push(Token::DocComment(comment));
            }
            
            // String literal
            '"' => {
                let mut string = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next();
                        break;
                    }
                    if ch == '\\' {
                        chars.next();
                        match chars.next() {
                            Some('n') => string.push('\n'),
                            Some('t') => string.push('\t'),
                            Some('r') => string.push('\r'),
                            Some('0') => string.push('\0'),
                            Some('\'') => string.push('\''),
                            Some('"') => string.push('"'),
                            Some('\\') => string.push('\\'),
                            Some(ch) => string.push(ch),
                            None => return Err("Unterminated string escape".to_string()),
                        }
                    } else {
                        string.push(chars.next().unwrap());
                    }
                }
                tokens.push(Token::StringLiteral(string));
            }
            
            // Char literal
            '\'' => {
                let mut char_val = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '\'' {
                        chars.next();
                        break;
                    }
                    if ch == '\\' {
                        chars.next();
                        match chars.next() {
                            Some('n') => char_val.push('\n'),
                            Some('t') => char_val.push('\t'),
                            Some('r') => char_val.push('\r'),
                            Some('0') => char_val.push('\0'),
                            Some('\'') => char_val.push('\''),
                            Some('"') => char_val.push('"'),
                            Some('\\') => char_val.push('\\'),
                            Some(ch) => char_val.push(ch),
                            None => return Err("Unterminated char escape".to_string()),
                        }
                    } else {
                        char_val.push(chars.next().unwrap());
                    }
                }
                if char_val.len() != 1 {
                    return Err(format!("Char literal must be exactly one character: '{}'", char_val));
                }
                tokens.push(Token::CharLiteral(char_val.chars().next().unwrap()));
            }
            
            // Numbers
            c if c.is_ascii_digit() => {
                let mut num_str = String::new();
                num_str.push(c);
                
                if chars.peek() == Some(&'x') {
                    chars.next();
                    num_str.push('x');
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_hexdigit() || ch == '_' {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let clean = num_str.replace('_', "");
                    let val = i64::from_str_radix(&clean[2..], 16)
                        .map_err(|e| format!("Invalid hex number: {}", e))?;
                    tokens.push(Token::IntLiteral(val));
                } else if chars.peek() == Some(&'b') {
                    chars.next();
                    num_str.push('b');
                    while let Some(&ch) = chars.peek() {
                        if ch == '0' || ch == '1' || ch == '_' {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let clean = num_str.replace('_', "");
                    let val = i64::from_str_radix(&clean[2..], 2)
                        .map_err(|e| format!("Invalid binary number: {}", e))?;
                    tokens.push(Token::IntLiteral(val));
                } else {
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    // Check for float
                    if chars.peek() == Some(&'.') {
                        num_str.push(chars.next().unwrap());
                        while let Some(&ch) = chars.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                num_str.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        // Check for exponent
                        if chars.peek() == Some(&'e') || chars.peek() == Some(&'E') {
                            num_str.push(chars.next().unwrap());
                            if chars.peek() == Some(&'+') || chars.peek() == Some(&'-') {
                                num_str.push(chars.next().unwrap());
                            }
                            while let Some(&ch) = chars.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    num_str.push(chars.next().unwrap());
                                } else {
                                    break;
                                }
                            }
                        }
                        let clean = num_str.replace('_', "");
                        let val = clean.parse::<f64>()
                            .map_err(|e| format!("Invalid float: {}", e))?;
                        tokens.push(Token::FloatLiteral(val));
                    } else {
                        // Check for type suffix
                        let mut suffix = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch.is_ascii_alphabetic() || ch == '_' {
                                suffix.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        
                        let clean = num_str.replace('_', "");
                        let val = clean.parse::<i64>()
                            .map_err(|e| format!("Invalid integer: {}", e))?;
                        tokens.push(Token::IntLiteral(val));
                    }
                }
            }
            
            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                let token = match ident.as_str() {
                    "let" => Token::Let,
                    "mut" => Token::Mut,
                    "const" => Token::Const,
                    "fn" => Token::Fn,
                    "def" => Token::Def,
                    "class" => Token::Class,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "elsif" => Token::ElseIf,
                    "unless" => Token::Unless,
                    "loop" => Token::Loop,
                    "while" => Token::While,
                    "for" => Token::For,
                    "in" => Token::In,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    "match" => Token::Match,
                    "case" => Token::Case,
                    "when" => Token::When,
                    "then" => Token::Then,
                    "struct" => Token::Struct,
                    "enum" => Token::Enum,
                    "class" => Token::Class,
                    "impl" => Token::Impl,
                    "use" => Token::Use,
                    "as" => Token::As,
                    "pub" => Token::Pub,
                    "mod" => Token::Mod,
                    "type" => Token::Type,
                    "unsafe" => Token::Unsafe,
                    "async" => Token::Async,
                    "await" => Token::Await,
                    "move" => Token::Move,
                    "ref" => Token::Ref,
                    "box" => Token::Box,
                    "vec" => Token::Vec,
                    "null" => Token::Null,
                    "nil" => Token::Nil,
                    "true" => Token::BoolLiteral(true),
                    "false" => Token::BoolLiteral(false),
                    "self" => Token::Self_,
                    "_" => Token::Underscore,
                    "yield" => Token::Yield,
                    "type" => Token::Type,
                    "delegate" => Token::Delegate,
                    "alias" => Token::Alias,
                    "macro" => Token::Macro,
                    "fun" => Token::Fun,
                    "lib" => Token::Lib,
                    "record" => Token::Record,
                    "abstract" => Token::Abstract,
                    "final" => Token::Final,
                    "property" => Token::Property,
                    "getter" => Token::Getter,
                    "setter" => Token::Setter,
                    "do" => Token::Do,
                    "end" => Token::End,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "not" => Token::Not,
                    "is" => Token::Is,
                    
                    // AI Keywords
                    "neural" => Token::Neural,
                    "network" => Token::Network,
                    "layer" => Token::Layer,
                    "train" => Token::Train,
                    "inference" => Token::Inference,
                    "tensor" => Token::Tensor,
                    "model" => Token::Model,
                    "dataset" => Token::Dataset,
                    "epoch" => Token::Epoch,
                    "batch" => Token::Batch,
                    "learning_rate" => Token::LearningRate,
                    "optimizer" => Token::Optimizer,
                    "loss" => Token::Loss,
                    "accuracy" => Token::Accuracy,
                    "embedding" => Token::Embedding,
                    "attention" => Token::Attention,
                    "transformer" => Token::Transformer,
                    "rnn" => Token::RNN,
                    "cnn" => Token::CNN,
                    "lstm" => Token::LSTM,
                    "gru" => Token::GRU,
                    
                    // AI Keywords
                    "neural" => Token::Neural,
                    "network" => Token::Network,
                    "layer" => Token::Layer,
                    "train" => Token::Train,
                    "inference" => Token::Inference,
                    "tensor" => Token::Tensor,
                    "model" => Token::Model,
                    "dataset" => Token::Dataset,
                    "epoch" => Token::Epoch,
                    "batch" => Token::Batch,
                    "learning_rate" => Token::LearningRate,
                    "optimizer" => Token::Optimizer,
                    "loss" => Token::Loss,
                    "accuracy" => Token::Accuracy,
                    "embedding" => Token::Embedding,
                    "attention" => Token::Attention,
                    "transformer" => Token::Transformer,
                    "rnn" => Token::RNN,
                    "cnn" => Token::CNN,
                    "lstm" => Token::LSTM,
                    "gru" => Token::GRU,
                    
                    "Int" => Token::TypeIdent("Int".to_string()),
                    "Int8" => Token::TypeIdent("Int8".to_string()),
                    "Int16" => Token::TypeIdent("Int16".to_string()),
                    "Int32" => Token::TypeIdent("Int32".to_string()),
                    "Int64" => Token::TypeIdent("Int64".to_string()),
                    "UInt" => Token::TypeIdent("UInt".to_string()),
                    "UInt8" => Token::TypeIdent("UInt8".to_string()),
                    "UInt16" => Token::TypeIdent("UInt16".to_string()),
                    "UInt32" => Token::TypeIdent("UInt32".to_string()),
                    "UInt64" => Token::TypeIdent("UInt64".to_string()),
                    "Float" => Token::TypeIdent("Float".to_string()),
                    "Float32" => Token::TypeIdent("Float32".to_string()),
                    "Float64" => Token::TypeIdent("Float64".to_string()),
                    "Bool" => Token::TypeIdent("Bool".to_string()),
                    "Char" => Token::TypeIdent("Char".to_string()),
                    "String" => Token::TypeIdent("String".to_string()),
                    "Void" => Token::TypeIdent("Void".to_string()),
                    _ => Token::Ident(ident),
                };
                tokens.push(token);
            }
            
            // Operators and delimiters
            '+' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::PlusAssign);
                    }
                    Some(&'+') => return Err("++ operator not supported, use += 1 instead".to_string()),
                    _ => tokens.push(Token::Plus),
                }
            }
            '-' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::MinusAssign);
                    }
                    Some(&'>') => {
                        chars.next();
                        tokens.push(Token::Arrow);
                    }
                    _ => tokens.push(Token::Minus),
                }
            }
            '*' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::StarAssign);
                    }
                    _ => tokens.push(Token::Star),
                }
            }
            '/' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::SlashAssign);
                    }
                    _ => tokens.push(Token::Slash),
                }
            }
            '%' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::PercentAssign);
                    }
                    _ => tokens.push(Token::Percent),
                }
            }
            '&' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::AmpersandAssign);
                    }
                    Some(&'&') => return Err("&& operator not supported, use 'and' keyword instead".to_string()),
                    _ => tokens.push(Token::Ampersand),
                }
            }
            '|' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::PipeAssign);
                    }
                    Some(&'|') => return Err("|| operator not supported, use 'or' keyword instead".to_string()),
                    _ => tokens.push(Token::Pipe),
                }
            }
            '^' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::CaretAssign);
                    }
                    _ => tokens.push(Token::Caret),
                }
            }
            '~' => tokens.push(Token::Tilde),
            '!' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::Ne);
                    }
                    _ => tokens.push(Token::Bang),
                }
            }
            '=' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::EqEq);
                    }
                    _ => tokens.push(Token::Assign),
                }
            }
            '<' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::LtEq);
                    }
                    Some(&'<') => {
                        chars.next();
                        match chars.peek() {
                            Some(&'=') => {
                                chars.next();
                                tokens.push(Token::ShlAssign);
                            }
                            _ => tokens.push(Token::Shl),
                        }
                    }
                    _ => tokens.push(Token::Lt),
                }
            }
            '>' => {
                match chars.peek() {
                    Some(&'=') => {
                        chars.next();
                        tokens.push(Token::GtEq);
                    }
                    Some(&'>') => {
                        chars.next();
                        match chars.peek() {
                            Some(&'=') => {
                                chars.next();
                                tokens.push(Token::ShrAssign);
                            }
                            _ => tokens.push(Token::Shr),
                        }
                    }
                    _ => tokens.push(Token::Gt),
                }
            }
            '.' => {
                match chars.peek() {
                    Some(&'.') => {
                        chars.next();
                        tokens.push(Token::DoubleDot);
                    }
                    _ => tokens.push(Token::Dot),
                }
            }
            ':' => {
                match chars.peek() {
                    Some(&':') => {
                        chars.next();
                        tokens.push(Token::DoubleArrow);
                    }
                    _ => tokens.push(Token::Colon),
                }
            }
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            
            // Unknown character
            c => {
                tokens.push(Token::Error(format!("Unexpected character: '{}'", c)));
            }
        }
    }
    
    tokens.push(Token::Eof);
    Ok(tokens)
}
