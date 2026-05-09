use crate::error::{Error, ErrorKind, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literals {
        int: i64,
        float: f64,
        string: String,
        char: char,
        bool: bool,
    },
    
    Keywords {
        let_kw: (),
        mut_kw: (),
        const_kw: (),
        func_kw: (),
        fn_kw: (),
        return_kw: (),
        when_kw: (),
        if_kw: (),
        else_kw: (),
        elsif_kw: (),
        unless_kw: (),
        loop_kw: (),
        while_kw: (),
        each_kw: (),
        for_kw: (),
        in_kw: (),
        break_kw: (),
        continue_kw: (),
        match_kw: (),
        case_kw: (),
        entity_kw: (),
        class_kw: (),
        struct_kw: (),
        enum_kw: (),
        use_kw: (),
        require_kw: (),
        as_kw: (),
        pub_kw: (),
        mod_kw: (),
        type_kw: (),
        unsafe_kw: (),
        async_kw: (),
        await_kw: (),
        move_kw: (),
        ref_kw: (),
        null_kw: (),
        nil_kw: (),
        true_kw: (),
        false_kw: (),
        self_kw: (),
        new_kw: (),
        super_kw: (),
        try_kw: (),
        catch_kw: (),
        finally_kw: (),
        defer_kw: (),
        comptime_kw: (),
        asm_kw: (),
        export_kw: (),
        import_kw: (),
        from_kw: (),
    },
    
    Types {
        int_type: (),
        int8_type: (),
        int16_type: (),
        int32_type: (),
        int64_type: (),
        uint_type: (),
        uint8_type: (),
        uint16_type: (),
        uint32_type: (),
        uint64_type: (),
        float_type: (),
        float32_type: (),
        float64_type: (),
        bool_type: (),
        char_type: (),
        string_type: (),
        void_type: (),
    },
    
    Operators {
        arrow: (),
        fat_arrow: (),
        exclamation: (),
        question: (),
        colon: (),
        double_colon: (),
    },
    
    Delimiters {
        l_paren: (),
        r_paren: (),
        l_brace: (),
        r_brace: (),
        l_bracket: (),
        r_bracket: (),
        comma: (),
        semicolon: (),
        dot: (),
        double_dot: (),
        tilde: (),
    },
    
    Identifiers {
        ident: String,
        type_ident: String,
    },
    
    Special {
        eof: (),
        newline: (),
        comment: String,
    },
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while !self.is_eof() {
            self.skip_whitespace();
            
            if self.is_eof() {
                break;
            }
            
            let token = self.read_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token::Special { eof: () });
        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_eof() {
            let ch = self.peek();
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else if ch == '\n' {
                self.advance();
                self.line += 1;
                self.column = 1;
            } else if ch == '#' {
                self.skip_comment();
            } else {
                break;
            }
        }
    }

    fn read_token(&mut self) -> Result<Token> {
        let ch = self.peek();
        
        match ch {
            '"' => self.read_string(),
            '\'' => self.read_char(),
            '0'..='9' => self.read_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
            '+' => { self.advance(); Ok(Token::Operators { arrow: () }) }
            '-' => {
                self.advance();
                if self.peek() == '>' {
                    self.advance();
                    Ok(Token::Operators { arrow: () })
                } else {
                    Ok(Token::Operators { arrow: () })
                }
            }
            '!' => {
                self.advance();
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::Operators { exclamation: () })
                } else {
                    Ok(Token::Operators { exclamation: () })
                }
            }
            '?' => {
                self.advance();
                Ok(Token::Operators { question: () })
            }
            ':' => {
                self.advance();
                if self.peek() == ':' {
                    self.advance();
                    Ok(Token::Operators { double_colon: () })
                } else {
                    Ok(Token::Operators { colon: () })
                }
            }
            '(' => { self.advance(); Ok(Token::Delimiters { l_paren: () }) }
            ')' => { self.advance(); Ok(Token::Delimiters { r_paren: () }) }
            '{' => { self.advance(); Ok(Token::Delimiters { l_brace: () }) }
            '}' => { self.advance(); Ok(Token::Delimiters { r_brace: () }) }
            '[' => { self.advance(); Ok(Token::Delimiters { l_bracket: () }) }
            ']' => { self.advance(); Ok(Token::Delimiters { r_bracket: () }) }
            ',' => { self.advance(); Ok(Token::Delimiters { comma: () }) }
            ';' => { self.advance(); Ok(Token::Delimiters { semicolon: () }) }
            '.' => {
                self.advance();
                if self.peek() == '.' {
                    self.advance();
                    Ok(Token::Delimiters { double_dot: () })
                } else {
                    Ok(Token::Delimiters { dot: () })
                }
            }
            '~' => { self.advance(); Ok(Token::Delimiters { tilde: () }) }
            _ => Err(Error::new(
                ErrorKind::Syntax,
                "",
                self.line,
                self.column,
                &format!("Unexpected character: '{}'", ch),
            )),
        }
    }

    fn read_string(&mut self) -> Result<Token> {
        self.advance();
        let mut s = String::new();
        
        while !self.is_eof() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => { s.push('\n'); self.advance(); }
                    't' => { s.push('\t'); self.advance(); }
                    '"' => { s.push('"'); self.advance(); }
                    '\\' => { s.push('\\'); self.advance(); }
                    _ => {}
                }
            } else {
                s.push(self.peek());
                self.advance();
            }
        }
        
        self.advance();
        Ok(Token::Literals { string: s, int: 0, float: 0.0, char: ' ', bool: false })
    }

    fn read_char(&mut self) -> Result<Token> {
        self.advance();
        let c = self.peek();
        self.advance();
        self.advance();
        Ok(Token::Literals { char: c, int: 0, float: 0.0, string: String::new(), bool: false })
    }

    fn read_number(&mut self) -> Result<Token> {
        let mut num_str = String::new();
        
        while !self.is_eof() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            num_str.push(self.peek());
            self.advance();
        }
        
        if num_str.contains('.') {
            let f: f64 = num_str.parse().unwrap_or(0.0);
            Ok(Token::Literals { float: f, int: 0, string: String::new(), char: ' ', bool: false })
        } else {
            let i: i64 = num_str.parse().unwrap_or(0);
            Ok(Token::Literals { int: i, float: 0.0, string: String::new(), char: ' ', bool: false })
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let mut ident = String::new();
        
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.peek());
            self.advance();
        }
        
        Ok(Token::Identifiers { ident, type_ident: String::new() })
    }

    fn skip_comment(&mut self) {
        self.advance();
        while !self.is_eof() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        self.input.get(self.pos).copied().unwrap_or('\0')
    }

    fn advance(&mut self) {
        if !self.is_eof() {
            self.pos += 1;
            self.column += 1;
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
