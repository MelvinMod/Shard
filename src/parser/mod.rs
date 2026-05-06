use crate::ast::{
    CompilationUnit, Node, FunctionDef, Parameter, VariableDef, StructDef, Field,
    EnumDef, EnumVariant, ImportDef, Block, Statement, Expression, Literal,
    BinOp, UnaryOp, Type, Pattern, IfExpr, LoopExpr, ForExpr, MatchExpr, MatchArm,
};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(mut self) -> Result<CompilationUnit, String> {
        let mut nodes = Vec::new();
        
        while !self.check(Token::Eof) {
            if let Some(node) = self.parse_top_level()? {
                nodes.push(node);
            }
        }
        
        Ok(CompilationUnit { nodes })
    }

    fn parse_top_level(&mut self) -> Result<Option<Node>, String> {
        if self.check(Token::Fn) {
            Ok(Some(Node::Function(self.parse_function()?)))
        } else if self.check(Token::Let) || self.check(Token::Mut) || self.check(Token::Const) {
            Ok(Some(Node::Variable(self.parse_variable()?)))
        } else if self.check(Token::Struct) {
            Ok(Some(Node::Struct(self.parse_struct()?)))
        } else if self.check(Token::Enum) {
            Ok(Some(Node::Enum(self.parse_enum()?)))
        } else if self.check(Token::Use) {
            Ok(Some(Node::Import(self.parse_import()?)))
        } else {
            self.advance();
            Ok(None)
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDef, String> {
        self.expect(Token::Fn)?;
        
        let name = self.parse_ident()?;
        
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        
        while !self.check(Token::RParen) {
            let param_name = self.parse_ident()?;
            self.expect(Token::Colon)?;
            let param_type = self.parse_type()?;
            params.push(Parameter {
                name: param_name,
                type_: param_type,
            });
            
            if !self.check(Token::Comma) {
                break;
            }
            self.advance();
        }
        
        self.expect(Token::RParen)?;
        
        let return_type = if self.check(Token::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            is_pub: false,
        })
    }

    fn parse_variable(&mut self) -> Result<VariableDef, String> {
        let is_mut = self.check(Token::Mut);
        if is_mut || self.check(Token::Let) || self.check(Token::Const) {
            self.advance();
        }
        
        let name = self.parse_ident()?;
        
        let type_ = if self.check(Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect(Token::Assign)?;
        let value = self.parse_expression()?;
        
        Ok(VariableDef {
            name,
            type_,
            value,
            is_mut,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDef, String> {
        self.expect(Token::Struct)?;
        let name = self.parse_ident()?;
        
        self.expect(Token::LBrace)?;
        let mut fields = Vec::new();
        
        while !self.check(Token::RBrace) {
            let field_name = self.parse_ident()?;
            self.expect(Token::Colon)?;
            let field_type = self.parse_type()?;
            fields.push(Field {
                name: field_name,
                type_: field_type,
            });
            
            if !self.check(Token::Comma) {
                break;
            }
            self.advance();
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(StructDef {
            name,
            fields,
            is_pub: false,
        })
    }

    fn parse_enum(&mut self) -> Result<EnumDef, String> {
        self.expect(Token::Enum)?;
        let name = self.parse_ident()?;
        
        self.expect(Token::LBrace)?;
        let mut variants = Vec::new();
        
        while !self.check(Token::RBrace) {
            let variant_name = self.parse_ident()?;
            let data = if self.check(Token::LParen) {
                self.advance();
                let data_type = self.parse_type()?;
                self.expect(Token::RParen)?;
                Some(data_type)
            } else {
                None
            };
            variants.push(EnumVariant {
                name: variant_name,
                data,
            });
            
            if !self.check(Token::Comma) {
                break;
            }
            self.advance();
        }
        
        self.expect(Token::RBrace)?;
        
        Ok(EnumDef {
            name,
            variants,
            is_pub: false,
        })
    }

    fn parse_import(&mut self) -> Result<ImportDef, String> {
        self.expect(Token::Use)?;
        let path = self.parse_ident()?;
        self.expect(Token::Semicolon)?;
        Ok(ImportDef { path })
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();
        
        while !self.check(Token::RBrace) && !self.check(Token::Eof) {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }
        
        self.expect(Token::RBrace)?;
        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        if self.check(Token::Return) {
            self.advance();
            let value = if self.check(Token::Semicolon) {
                None
            } else {
                Some(self.parse_expression()?)
            };
            self.expect(Token::Semicolon)?;
            return Ok(Some(Statement::Return(value)));
        }
        
        if self.check(Token::If) {
            return Ok(Some(Statement::If(self.parse_if_expr()?)));
        }
        
        if self.check(Token::Loop) {
            return Ok(Some(Statement::Loop(self.parse_loop_expr()?)));
        }
        
        if self.check(Token::For) {
            return Ok(Some(Statement::For(self.parse_for_expr()?)));
        }
        
        if self.check(Token::Match) {
            return Ok(Some(Statement::Match(self.parse_match_expr()?)));
        }
        
        if self.check(Token::Break) {
            self.advance();
            self.expect(Token::Semicolon)?;
            return Ok(Some(Statement::Break));
        }
        
        if self.check(Token::Continue) {
            self.advance();
            self.expect(Token::Semicolon)?;
            return Ok(Some(Statement::Continue));
        }
        
        if self.check(Token::Let) || self.check(Token::Mut) || self.check(Token::Const) {
            return Ok(Some(Statement::Variable(self.parse_variable()?)));
        }
        
        let expr = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        Ok(Some(Statement::Expression(expr)))
    }

    fn parse_if_expr(&mut self) -> Result<IfExpr, String> {
        self.expect(Token::If)?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        
        let else_block = if self.check(Token::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(IfExpr {
            condition,
            then_block,
            else_block,
        })
    }

    fn parse_loop_expr(&mut self) -> Result<LoopExpr, String> {
        self.expect(Token::Loop)?;
        let body = self.parse_block()?;
        Ok(LoopExpr { body })
    }

    fn parse_for_expr(&mut self) -> Result<ForExpr, String> {
        self.expect(Token::For)?;
        let variable = self.parse_ident()?;
        self.expect(Token::In)?;
        let iterable = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(ForExpr {
            variable,
            iterable,
            body,
        })
    }

    fn parse_match_expr(&mut self) -> Result<MatchExpr, String> {
        self.expect(Token::Match)?;
        let value = self.parse_expression()?;
        
        self.expect(Token::LBrace)?;
        let mut arms = Vec::new();
        
        while !self.check(Token::RBrace) {
            let pattern = self.parse_pattern()?;
            self.expect(Token::FatArrow)?;
            let body = self.parse_block()?;
            arms.push(MatchArm { pattern, body });
            
            if self.check(Token::Comma) {
                self.advance();
            }
        }
        
        self.expect(Token::RBrace)?;
        Ok(MatchExpr { value, arms })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        if self.check(Token::Underscore) {
            self.advance();
            return Ok(Pattern::Wildcard);
        }
        
        if self.check(Token::IntLiteral(_)) || self.check(Token::StringLiteral(_)) {
            return Ok(Pattern::Literal(self.parse_literal()?));
        }
        
        let ident = self.parse_ident()?;
        Ok(Pattern::Identifier(ident))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_and()?;
        
        while self.check(Token::Pipe) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOp(Box::new(left), BinOp::Or, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_equality()?;
        
        while self.check(Token::Ampersand) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp(Box::new(left), BinOp::And, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;
        
        while self.check(Token::EqEq) || self.check(Token::Ne) {
            let op = if self.check(Token::EqEq) {
                self.advance();
                BinOp::Eq
            } else {
                self.advance();
                BinOp::Ne
            };
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_additive()?;
        
        while self.check(Token::Lt) || self.check(Token::Gt) || self.check(Token::LtEq) || self.check(Token::GtEq) {
            let op = if self.check(Token::Lt) {
                self.advance();
                BinOp::Lt
            } else if self.check(Token::Gt) {
                self.advance();
                BinOp::Gt
            } else if self.check(Token::LtEq) {
                self.advance();
                BinOp::Le
            } else {
                self.advance();
                BinOp::Ge
            };
            let right = self.parse_additive()?;
            left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplicative()?;
        
        while self.check(Token::Plus) || self.check(Token::Minus) {
            let op = if self.check(Token::Plus) {
                self.advance();
                BinOp::Add
            } else {
                self.advance();
                BinOp::Sub
            };
            let right = self.parse_multiplicative()?;
            left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;
        
        while self.check(Token::Star) || self.check(Token::Slash) || self.check(Token::Percent) {
            let op = if self.check(Token::Star) {
                self.advance();
                BinOp::Mul
            } else if self.check(Token::Slash) {
                self.advance();
                BinOp::Div
            } else {
                self.advance();
                BinOp::Mod
            };
            let right = self.parse_unary()?;
            left = Expression::BinaryOp(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.check(Token::Bang) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::Not, Box::new(expr)));
        }
        
        if self.check(Token::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::Neg, Box::new(expr)));
        }
        
        if self.check(Token::Tilde) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::BitNot, Box::new(expr)));
        }
        
        if self.check(Token::Ampersand) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::Deref, Box::new(expr)));
        }
        
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        if self.check(Token::IntLiteral(_)) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check(Token::FloatLiteral(_)) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check(Token::StringLiteral(_)) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check(Token::CharLiteral(_)) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check(Token::BoolLiteral(_)) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check(Token::Null) {
            self.advance();
            return Ok(Expression::Literal(Literal::Null));
        }
        
        if self.check(Token::LParen) {
            self.advance();
            if self.check(Token::RParen) {
                self.advance();
                return Ok(Expression::Tuple(Vec::new()));
            }
            let first = self.parse_expression()?;
            if self.check(Token::Comma) {
                self.advance();
                let mut exprs = vec![first];
                while !self.check(Token::RParen) {
                    exprs.push(self.parse_expression()?);
                    if !self.check(Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect(Token::RParen)?;
                return Ok(Expression::Tuple(exprs));
            }
            self.expect(Token::RParen)?;
            return Ok(first);
        }
        
        if self.check(Token::LBrace) {
            return Ok(Expression::Block(self.parse_block()?));
        }
        
        if self.check(Token::LBracket) {
            self.advance();
            let mut elements = Vec::new();
            while !self.check(Token::RBracket) {
                elements.push(self.parse_expression()?);
                if !self.check(Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect(Token::RBracket)?;
            return Ok(Expression::Array(elements));
        }
        
        if self.check(Token::If) {
            return Ok(Expression::If(self.parse_if_expr()?));
        }
        
        if self.check(Token::Match) {
            return Ok(Expression::Match(self.parse_match_expr()?));
        }
        
        if self.check(Token::Ident(_)) {
            let name = self.parse_ident()?;
            
            if self.check(Token::LParen) {
                self.advance();
                let mut args = Vec::new();
                while !self.check(Token::RParen) {
                    args.push(self.parse_expression()?);
                    if !self.check(Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect(Token::RParen)?;
                return Ok(Expression::Call {
                    func: Box::new(Expression::Identifier(name)),
                    args,
                });
            }
            
            return Ok(Expression::Identifier(name));
        }
        
        Err(format!("Unexpected token: {:?}", self.peek()))
    }

    fn parse_literal(&mut self) -> Result<Literal, String> {
        match self.current() {
            Token::IntLiteral(n) => {
                self.advance();
                Ok(Literal::Int(*n))
            }
            Token::FloatLiteral(n) => {
                self.advance();
                Ok(Literal::Float(*n))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Literal::String(s.clone()))
            }
            Token::CharLiteral(c) => {
                self.advance();
                Ok(Literal::Char(*c))
            }
            Token::BoolLiteral(b) => {
                self.advance();
                Ok(Literal::Bool(*b))
            }
            _ => Err(format!("Expected literal, got {:?}", self.peek())),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.check(Token::TypeIdent(_)) {
            match self.current() {
                Token::TypeIdent(name) => {
                    self.advance();
                    match name.as_str() {
                        "Int" => Ok(Type::Int),
                        "Int8" => Ok(Type::Int8),
                        "Int16" => Ok(Type::Int16),
                        "Int32" => Ok(Type::Int32),
                        "Int64" => Ok(Type::Int64),
                        "UInt" => Ok(Type::UInt),
                        "UInt8" => Ok(Type::UInt8),
                        "UInt16" => Ok(Type::UInt16),
                        "UInt32" => Ok(Type::UInt32),
                        "UInt64" => Ok(Type::UInt64),
                        "Float" => Ok(Type::Float),
                        "Float32" => Ok(Type::Float32),
                        "Float64" => Ok(Type::Float64),
                        "Bool" => Ok(Type::Bool),
                        "Char" => Ok(Type::Char),
                        "String" => Ok(Type::String),
                        "Void" => Ok(Type::Void),
                        _ => Ok(Type::UserDefined(name.clone())),
                    }
                }
                _ => unreachable!(),
            }
        } else if self.check(Token::Ampersand) {
            self.advance();
            let inner = self.parse_type()?;
            Ok(Type::Reference(Box::new(inner)))
        } else if self.check(Token::Star) {
            self.advance();
            let inner = self.parse_type()?;
            Ok(Type::Pointer(Box::new(inner)))
        } else if self.check(Token::LBracket) {
            self.advance();
            let inner = self.parse_type()?;
            let size = if self.check(Token::Semicolon) {
                self.advance();
                if let Token::IntLiteral(s) = self.current() {
                    self.advance();
                    Some(*s as usize)
                } else {
                    None
                }
            } else {
                None
            };
            self.expect(Token::RBracket)?;
            Ok(Type::Array(Box::new(inner), size))
        } else {
            Err(format!("Expected type, got {:?}", self.peek()))
        }
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        if let Token::Ident(name) = self.current() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(format!("Expected identifier, got {:?}", self.peek()))
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek(&self) -> &Token {
        if self.pos + 1 < self.tokens.len() {
            &self.tokens[self.pos + 1]
        } else {
            &self.tokens[self.pos]
        }
    }

    fn check(&self, token: Token) -> bool {
        std::mem::discriminant(self.current()) == std::mem::discriminant(&token)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.check(token.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", token, self.peek()))
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<CompilationUnit, String> {
    let parser = Parser::new(tokens);
    parser.parse()
}