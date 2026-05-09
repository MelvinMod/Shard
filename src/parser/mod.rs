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

    fn skip_newlines(&mut self) {
        while self.check_token_type(TokenKind::Newline) || self.check_token_type(TokenKind::Comment) {
            self.advance();
        }
    }

    pub fn parse(mut self) -> Result<CompilationUnit, String> {
        let mut nodes = Vec::new();
        
        self.skip_newlines();
        
        while !self.check_token_type(TokenKind::Eof) {
            if let Some(node) = self.parse_top_level()? {
                nodes.push(node);
            }
            self.skip_newlines();
        }
        
        Ok(CompilationUnit { nodes })
    }

    fn parse_top_level(&mut self) -> Result<Option<Node>, String> {
        self.skip_newlines();
        
        if self.check_token_type(TokenKind::Fn) {
            Ok(Some(Node::Function(self.parse_function()?)))
        } else if self.check_token_type(TokenKind::Let) || self.check_token_type(TokenKind::Mut) || self.check_token_type(TokenKind::Const) {
            Ok(Some(Node::Variable(self.parse_variable()?)))
        } else if self.check_token_type(TokenKind::Struct) {
            Ok(Some(Node::Struct(self.parse_struct()?)))
        } else if self.check_token_type(TokenKind::Enum) {
            Ok(Some(Node::Enum(self.parse_enum()?)))
        } else if self.check_token_type(TokenKind::Use) {
            Ok(Some(Node::Import(self.parse_import()?)))
        } else if self.check_token_type(TokenKind::Eof) {
            Ok(None)
        } else {
            return Err(format!("Unexpected token at top level: {:?}", self.peek()));
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDef, String> {
        self.expect_token_type(TokenKind::Fn)?;
        
        let name = self.parse_ident()?;
        
        self.expect_token_type(TokenKind::LParen)?;
        let mut params = Vec::new();
        
        while !self.check_token_type(TokenKind::RParen) {
            let param_name = self.parse_ident()?;
            self.expect_token_type(TokenKind::Colon)?;
            let param_type = self.parse_type()?;
            params.push(Parameter {
                name: param_name,
                type_: param_type,
            });
            
            if !self.check_token_type(TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        
        self.expect_token_type(TokenKind::RParen)?;
        
        let return_type = if self.check_token_type(TokenKind::Arrow) {
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
        let is_mut = self.check_token_type(TokenKind::Mut);
        if is_mut || self.check_token_type(TokenKind::Let) || self.check_token_type(TokenKind::Const) {
            self.advance();
        }
        
        let name = self.parse_ident()?;
        
        let type_ = if self.check_token_type(TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.expect_token_type(TokenKind::Assign)?;
        let value = self.parse_expression()?;
        
        Ok(VariableDef {
            name,
            type_,
            value,
            is_mut,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDef, String> {
        self.expect_token_type(TokenKind::Struct)?;
        let name = self.parse_ident()?;
        
        self.expect_token_type(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        
        while !self.check_token_type(TokenKind::RBrace) {
            let field_name = self.parse_ident()?;
            self.expect_token_type(TokenKind::Colon)?;
            let field_type = self.parse_type()?;
            fields.push(Field {
                name: field_name,
                type_: field_type,
            });
            
            if !self.check_token_type(TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        
        self.expect_token_type(TokenKind::RBrace)?;
        
        Ok(StructDef {
            name,
            fields,
            is_pub: false,
        })
    }

    fn parse_enum(&mut self) -> Result<EnumDef, String> {
        self.expect_token_type(TokenKind::Enum)?;
        let name = self.parse_ident()?;
        
        self.expect_token_type(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        
        while !self.check_token_type(TokenKind::RBrace) {
            let variant_name = self.parse_ident()?;
            let data = if self.check_token_type(TokenKind::LParen) {
                self.advance();
                let data_type = self.parse_type()?;
                self.expect_token_type(TokenKind::RParen)?;
                Some(data_type)
            } else {
                None
            };
            variants.push(EnumVariant {
                name: variant_name,
                data,
            });
            
            if !self.check_token_type(TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        
        self.expect_token_type(TokenKind::RBrace)?;
        
        Ok(EnumDef {
            name,
            variants,
            is_pub: false,
        })
    }

    fn parse_import(&mut self) -> Result<ImportDef, String> {
        self.expect_token_type(TokenKind::Use)?;
        let path = self.parse_ident()?;
        self.expect_token_type(TokenKind::Semicolon)?;
        Ok(ImportDef { path })
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.skip_newlines();
        
        // Accept either { or nothing (newline already consumed)
        if self.check_token_type(TokenKind::LBrace) {
            self.advance();
            self.skip_newlines();
        }
        
        let mut statements = Vec::new();
        
        // Parse until } or ~ or EOF
        while !self.check_token_type(TokenKind::RBrace) 
            && !self.check_token_type(TokenKind::Tilde)
            && !self.check_token_type(TokenKind::Eof) {
            
            self.skip_newlines();
            
            if self.check_token_type(TokenKind::RBrace) || self.check_token_type(TokenKind::Tilde) || self.check_token_type(TokenKind::Eof) {
                break;
            }
            
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
            self.skip_newlines();
        }
        
        // Consume the closing token if present
        if self.check_token_type(TokenKind::RBrace) {
            self.advance();
        } else if self.check_token_type(TokenKind::Tilde) {
            self.advance();
        }
        
        Ok(Block { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, String> {
        if self.check_token_type(TokenKind::Return) {
            self.advance();
            let value = if self.check_token_type(TokenKind::Semicolon) {
                None
            } else {
                Some(self.parse_expression()?)
            };
            self.expect_token_type(TokenKind::Semicolon)?;
            return Ok(Some(Statement::Return(value)));
        }
        
        if self.check_token_type(TokenKind::If) {
            return Ok(Some(Statement::If(self.parse_if_expr()?)));
        }
        
        if self.check_token_type(TokenKind::Loop) {
            return Ok(Some(Statement::Loop(self.parse_loop_expr()?)));
        }
        
        if self.check_token_type(TokenKind::For) {
            return Ok(Some(Statement::For(self.parse_for_expr()?)));
        }
        
        if self.check_token_type(TokenKind::Match) {
            return Ok(Some(Statement::Match(self.parse_match_expr()?)));
        }
        
        if self.check_token_type(TokenKind::Break) {
            self.advance();
            self.expect_token_type(TokenKind::Semicolon)?;
            return Ok(Some(Statement::Break));
        }
        
        if self.check_token_type(TokenKind::Continue) {
            self.advance();
            self.expect_token_type(TokenKind::Semicolon)?;
            return Ok(Some(Statement::Continue));
        }
        
        if self.check_token_type(TokenKind::Let) || self.check_token_type(TokenKind::Mut) || self.check_token_type(TokenKind::Const) {
            return Ok(Some(Statement::Variable(self.parse_variable()?)));
        }
        
        // Try to parse as expression (function call, etc.)
        let expr = self.parse_expression()?;
        
        // Optional semicolon (Shard syntax doesn't require it)
        if self.check_token_type(TokenKind::Semicolon) {
            self.advance();
        }
        
        Ok(Some(Statement::Expression(expr)))
    }

    fn parse_if_expr(&mut self) -> Result<IfExpr, String> {
        self.expect_token_type(TokenKind::If)?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        
        let else_block = if self.check_token_type(TokenKind::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(IfExpr {
            condition: Box::new(condition),
            then_block,
            else_block,
        })
    }

    fn parse_loop_expr(&mut self) -> Result<LoopExpr, String> {
        self.expect_token_type(TokenKind::Loop)?;
        let body = self.parse_block()?;
        Ok(LoopExpr { body })
    }

    fn parse_for_expr(&mut self) -> Result<ForExpr, String> {
        self.expect_token_type(TokenKind::For)?;
        let variable = self.parse_ident()?;
        self.expect_token_type(TokenKind::In)?;
        let iterable = self.parse_expression()?;
        let body = self.parse_block()?;
        Ok(ForExpr {
            variable,
            iterable,
            body,
        })
    }

    fn parse_match_expr(&mut self) -> Result<MatchExpr, String> {
        self.expect_token_type(TokenKind::Match)?;
        let value = self.parse_expression()?;
        
        self.expect_token_type(TokenKind::LBrace)?;
        let mut arms = Vec::new();
        
        while !self.check_token_type(TokenKind::RBrace) {
            let pattern = self.parse_pattern()?;
            self.expect_token_type(TokenKind::FatArrow)?;
            let body = self.parse_block()?;
            arms.push(MatchArm { pattern, body });
            
            if self.check_token_type(TokenKind::Comma) {
                self.advance();
            }
        }
        
        self.expect_token_type(TokenKind::RBrace)?;
        Ok(MatchExpr { value: Box::new(value), arms })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, String> {
        if self.check_token_type(TokenKind::Underscore) {
            self.advance();
            return Ok(Pattern::Wildcard);
        }
        
        if self.check_token_type(TokenKind::IntLiteral) || self.check_token_type(TokenKind::StringLiteral) {
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
        
        while self.check_token_type(TokenKind::Pipe) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOp(Box::new(left), BinOp::Or, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_equality()?;
        
        while self.check_token_type(TokenKind::Ampersand) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp(Box::new(left), BinOp::And, Box::new(right));
        }
        
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;
        
        while self.check_token_type(TokenKind::EqEq) || self.check_token_type(TokenKind::Ne) {
            let op = if self.check_token_type(TokenKind::EqEq) {
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
        
        while self.check_token_type(TokenKind::Lt) || self.check_token_type(TokenKind::Gt) || self.check_token_type(TokenKind::LtEq) || self.check_token_type(TokenKind::GtEq) {
            let op = if self.check_token_type(TokenKind::Lt) {
                self.advance();
                BinOp::Lt
            } else if self.check_token_type(TokenKind::Gt) {
                self.advance();
                BinOp::Gt
            } else if self.check_token_type(TokenKind::LtEq) {
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
        
        while self.check_token_type(TokenKind::Plus) || self.check_token_type(TokenKind::Minus) {
            let op = if self.check_token_type(TokenKind::Plus) {
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
        
        while self.check_token_type(TokenKind::Star) || self.check_token_type(TokenKind::Slash) || self.check_token_type(TokenKind::Percent) {
            let op = if self.check_token_type(TokenKind::Star) {
                self.advance();
                BinOp::Mul
            } else if self.check_token_type(TokenKind::Slash) {
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
        if self.check_token_type(TokenKind::Bang) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::Not, Box::new(expr)));
        }
        
        if self.check_token_type(TokenKind::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::Neg, Box::new(expr)));
        }
        
        if self.check_token_type(TokenKind::Tilde) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::BitNot, Box::new(expr)));
        }
        
        if self.check_token_type(TokenKind::Ampersand) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expression::UnaryOp(UnaryOp::Deref, Box::new(expr)));
        }
        
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        if self.check_token_type(TokenKind::IntLiteral) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check_token_type(TokenKind::FloatLiteral) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check_token_type(TokenKind::StringLiteral) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check_token_type(TokenKind::CharLiteral) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check_token_type(TokenKind::BoolLiteral) {
            return Ok(Expression::Literal(self.parse_literal()?));
        }
        
        if self.check_token_type(TokenKind::Null) {
            self.advance();
            return Ok(Expression::Literal(Literal::Null));
        }
        
        if self.check_token_type(TokenKind::LParen) {
            self.advance();
            if self.check_token_type(TokenKind::RParen) {
                self.advance();
                return Ok(Expression::Tuple(Vec::new()));
            }
            let first = self.parse_expression()?;
            if self.check_token_type(TokenKind::Comma) {
                self.advance();
                let mut exprs = vec![first];
                while !self.check_token_type(TokenKind::RParen) {
                    exprs.push(self.parse_expression()?);
                    if !self.check_token_type(TokenKind::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect_token_type(TokenKind::RParen)?;
                return Ok(Expression::Tuple(exprs));
            }
            self.expect_token_type(TokenKind::RParen)?;
            return Ok(first);
        }
        
        if self.check_token_type(TokenKind::LBrace) {
            return Ok(Expression::Block(self.parse_block()?));
        }
        
        if self.check_token_type(TokenKind::LBracket) {
            self.advance();
            let mut elements = Vec::new();
            while !self.check_token_type(TokenKind::RBracket) {
                elements.push(self.parse_expression()?);
                if !self.check_token_type(TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token_type(TokenKind::RBracket)?;
            return Ok(Expression::Array(elements));
        }
        
        if self.check_token_type(TokenKind::If) {
            return Ok(Expression::If(Box::new(self.parse_if_expr()?)));
        }
        
        if self.check_token_type(TokenKind::Match) {
            return Ok(Expression::Match(self.parse_match_expr()?));
        }
        
        if self.check_token_type(TokenKind::Ident) {
            let name = self.parse_ident()?;
            
            if self.check_token_type(TokenKind::LParen) {
                self.advance();
                let mut args = Vec::new();
                while !self.check_token_type(TokenKind::RParen) {
                    args.push(self.parse_expression()?);
                    if !self.check_token_type(TokenKind::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect_token_type(TokenKind::RParen)?;
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
        let token_kind = self.current_token_kind();
        let result = match token_kind {
            TokenKind::IntLiteral => {
                if let Token::IntLiteral(n) = self.current().clone() {
                    Literal::Int(n)
                } else {
                    return Err("Expected IntLiteral".to_string());
                }
            }
            TokenKind::FloatLiteral => {
                if let Token::FloatLiteral(n) = self.current().clone() {
                    Literal::Float(n)
                } else {
                    return Err("Expected FloatLiteral".to_string());
                }
            }
            TokenKind::StringLiteral => {
                if let Token::StringLiteral(s) = self.current().clone() {
                    Literal::String(s)
                } else {
                    return Err("Expected StringLiteral".to_string());
                }
            }
            TokenKind::CharLiteral => {
                if let Token::CharLiteral(c) = self.current().clone() {
                    Literal::Char(c)
                } else {
                    return Err("Expected CharLiteral".to_string());
                }
            }
            TokenKind::BoolLiteral => {
                if let Token::BoolLiteral(b) = self.current().clone() {
                    Literal::Bool(b)
                } else {
                    return Err("Expected BoolLiteral".to_string());
                }
            }
            _ => return Err(format!("Expected literal, got {:?}", self.peek())),
        };
        self.advance();
        Ok(result)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        if self.check_token_type(TokenKind::TypeIdent) {
            if let Token::TypeIdent(name) = self.current().clone() {
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
            } else {
                unreachable!()
            }
        } else if self.check_token_type(TokenKind::Ampersand) {
            self.advance();
            let inner = self.parse_type()?;
            Ok(Type::Reference(Box::new(inner)))
        } else if self.check_token_type(TokenKind::Star) {
            self.advance();
            let inner = self.parse_type()?;
            Ok(Type::Pointer(Box::new(inner)))
        } else if self.check_token_type(TokenKind::LBracket) {
            self.advance();
            let inner = self.parse_type()?;
            let size = if self.check_token_type(TokenKind::Semicolon) {
                self.advance();
                if let Token::IntLiteral(s) = self.current().clone() {
                    self.advance();
                    Some(s as usize)
                } else {
                    None
                }
            } else {
                None
            };
            self.expect_token_type(TokenKind::RBracket)?;
            Ok(Type::Array(Box::new(inner), size))
        } else {
            Err(format!("Expected type, got {:?}", self.peek()))
        }
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        if let Token::Ident(name) = self.current().clone() {
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

    fn current_token_kind(&self) -> TokenKind {
        TokenKind::from_token(self.current())
    }

    fn check_token_type(&self, kind: TokenKind) -> bool {
        self.current_token_kind() == kind
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    fn expect_token_type(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.check_token_type(kind.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected token kind {:?}, got {:?}", kind, self.peek()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    // Literals
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,
    
    // Keywords
    Let,
    Mut,
    Const,
    Fn,
    Return,
    If,
    Else,
    ElsIf,
    Loop,
    While,
    For,
    In,
    Break,
    Continue,
    Match,
    Case,
    When,
    Then,
    Unless,
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
    Nil,
    True,
    False,
    Self_,
    Underscore,
    Yield,
    Delegate,
    Alias,
    Macro,
    Fun,
    Lib,
    Record,
    Union,
    Annotation,
    Is,
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
    And,
    Or,
    Not,
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
    Ident,
    TypeIdent,
    Newline,
    Comment,
    DocComment,
    Eof,
    Error,
}

impl TokenKind {
    fn from_token(token: &Token) -> Self {
        match token {
            Token::IntLiteral(_) => TokenKind::IntLiteral,
            Token::FloatLiteral(_) => TokenKind::FloatLiteral,
            Token::StringLiteral(_) => TokenKind::StringLiteral,
            Token::CharLiteral(_) => TokenKind::CharLiteral,
            Token::BoolLiteral(_) => TokenKind::BoolLiteral,
            Token::Let => TokenKind::Let,
            Token::Mut => TokenKind::Mut,
            Token::Const => TokenKind::Const,
            Token::Fn => TokenKind::Fn,
            Token::Return => TokenKind::Return,
            Token::If => TokenKind::If,
            Token::Else => TokenKind::Else,
            Token::ElsIf => TokenKind::ElsIf,
            Token::Loop => TokenKind::Loop,
            Token::While => TokenKind::While,
            Token::For => TokenKind::For,
            Token::In => TokenKind::In,
            Token::Break => TokenKind::Break,
            Token::Continue => TokenKind::Continue,
            Token::Match => TokenKind::Match,
            Token::Case => TokenKind::Case,
            Token::When => TokenKind::When,
            Token::Then => TokenKind::Then,
            Token::Unless => TokenKind::Unless,
            Token::Struct => TokenKind::Struct,
            Token::Enum => TokenKind::Enum,
            Token::Impl => TokenKind::Impl,
            Token::Use => TokenKind::Use,
            Token::As => TokenKind::As,
            Token::Pub => TokenKind::Pub,
            Token::Mod => TokenKind::Mod,
            Token::Type => TokenKind::Type,
            Token::Unsafe => TokenKind::Unsafe,
            Token::Async => TokenKind::Async,
            Token::Await => TokenKind::Await,
            Token::Move => TokenKind::Move,
            Token::Ref => TokenKind::Ref,
            Token::Box => TokenKind::Box,
            Token::Vec => TokenKind::Vec,
            Token::Null => TokenKind::Null,
            Token::Nil => TokenKind::Nil,
            Token::True | Token::BoolLiteral(_) => TokenKind::BoolLiteral,
            Token::False => TokenKind::False,
            Token::Self_ => TokenKind::Self_,
            Token::Underscore => TokenKind::Underscore,
            Token::Yield => TokenKind::Yield,
            Token::Delegate => TokenKind::Delegate,
            Token::Alias => TokenKind::Alias,
            Token::Macro => TokenKind::Macro,
            Token::Fun => TokenKind::Fun,
            Token::Lib => TokenKind::Lib,
            Token::Record => TokenKind::Record,
            Token::Union => TokenKind::Union,
            Token::Annotation => TokenKind::Annotation,
            Token::Is => TokenKind::Is,
            Token::RespondsTo => TokenKind::RespondsTo,
            Token::Include => TokenKind::Include,
            Token::Extend => TokenKind::Extend,
            Token::Primitive => TokenKind::Primitive,
            Token::Forward => TokenKind::Forward,
            Token::Abstract => TokenKind::Abstract,
            Token::Final => TokenKind::Final,
            Token::Def => TokenKind::Def,
            Token::Class => TokenKind::Class,
            Token::Property => TokenKind::Property,
            Token::Getter => TokenKind::Getter,
            Token::Setter => TokenKind::Setter,
            Token::Do => TokenKind::Do,
            Token::End => TokenKind::End,
            Token::And => TokenKind::And,
            Token::Or => TokenKind::Or,
            Token::Not => TokenKind::Not,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Star => TokenKind::Star,
            Token::Slash => TokenKind::Slash,
            Token::Percent => TokenKind::Percent,
            Token::Ampersand => TokenKind::Ampersand,
            Token::Pipe => TokenKind::Pipe,
            Token::Caret => TokenKind::Caret,
            Token::Tilde => TokenKind::Tilde,
            Token::Bang => TokenKind::Bang,
            Token::Eq => TokenKind::Eq,
            Token::Lt => TokenKind::Lt,
            Token::Gt => TokenKind::Gt,
            Token::Dot => TokenKind::Dot,
            Token::DoubleDot => TokenKind::DoubleDot,
            Token::Arrow => TokenKind::Arrow,
            Token::DoubleArrow => TokenKind::DoubleArrow,
            Token::FatArrow => TokenKind::FatArrow,
            Token::Neural => TokenKind::Neural,
            Token::Network => TokenKind::Network,
            Token::Layer => TokenKind::Layer,
            Token::Train => TokenKind::Train,
            Token::Inference => TokenKind::Inference,
            Token::Tensor => TokenKind::Tensor,
            Token::Model => TokenKind::Model,
            Token::Dataset => TokenKind::Dataset,
            Token::Epoch => TokenKind::Epoch,
            Token::Batch => TokenKind::Batch,
            Token::LearningRate => TokenKind::LearningRate,
            Token::Optimizer => TokenKind::Optimizer,
            Token::Loss => TokenKind::Loss,
            Token::Accuracy => TokenKind::Accuracy,
            Token::Embedding => TokenKind::Embedding,
            Token::Attention => TokenKind::Attention,
            Token::Transformer => TokenKind::Transformer,
            Token::RNN => TokenKind::RNN,
            Token::CNN => TokenKind::CNN,
            Token::LSTM => TokenKind::LSTM,
            Token::GRU => TokenKind::GRU,
            Token::LParen => TokenKind::LParen,
            Token::RParen => TokenKind::RParen,
            Token::LBrace => TokenKind::LBrace,
            Token::RBrace => TokenKind::RBrace,
            Token::LBracket => TokenKind::LBracket,
            Token::RBracket => TokenKind::RBracket,
            Token::Comma => TokenKind::Comma,
            Token::Colon => TokenKind::Colon,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Assign => TokenKind::Assign,
            Token::PlusAssign => TokenKind::PlusAssign,
            Token::MinusAssign => TokenKind::MinusAssign,
            Token::StarAssign => TokenKind::StarAssign,
            Token::SlashAssign => TokenKind::SlashAssign,
            Token::PercentAssign => TokenKind::PercentAssign,
            Token::AmpersandAssign => TokenKind::AmpersandAssign,
            Token::PipeAssign => TokenKind::PipeAssign,
            Token::CaretAssign => TokenKind::CaretAssign,
            Token::ShlAssign => TokenKind::ShlAssign,
            Token::ShrAssign => TokenKind::ShrAssign,
            Token::EqEq => TokenKind::EqEq,
            Token::Ne => TokenKind::Ne,
            Token::LtEq => TokenKind::LtEq,
            Token::GtEq => TokenKind::GtEq,
            Token::Shl => TokenKind::Shl,
            Token::Shr => TokenKind::Shr,
            Token::Ident(_) => TokenKind::Ident,
            Token::TypeIdent(_) => TokenKind::TypeIdent,
            Token::Newline => TokenKind::Newline,
            Token::Comment(_) => TokenKind::Comment,
            Token::DocComment(_) => TokenKind::DocComment,
            Token::Eof => TokenKind::Eof,
            Token::Error(_) => TokenKind::Error,
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<CompilationUnit, String> {
    let parser = Parser::new(tokens);
    parser.parse()
}
