use crate::ast::{
    CompilationUnit, Node, FunctionDef, VariableDef, Statement, Expression,
    Type, Literal, BinOp, UnaryOp,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeCheckError {
    pub message: String,
    pub line: usize,
}

pub struct TypeChecker {
    env: HashMap<String, Type>,
    errors: Vec<TypeCheckError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = HashMap::new();
        
        env.insert("true".to_string(), Type::Bool);
        env.insert("false".to_string(), Type::Bool);
        env.insert("null".to_string(), Type::Infer);
        
        Self {
            env,
            errors: Vec::new(),
        }
    }

    pub fn check(ast: &CompilationUnit) -> Result<(), String> {
        let mut checker = Self::new();
        checker.check_unit(ast)?;
        
        if checker.errors.is_empty() {
            Ok(())
        } else {
            let mut msg = String::new();
            for err in &checker.errors {
                msg.push_str(&format!("Error at line {}: {}\n", err.line, err.message));
            }
            Err(msg)
        }
    }

    fn check_unit(&mut self, unit: &CompilationUnit) -> Result<(), String> {
        for node in &unit.nodes {
            self.check_node(node)?;
        }
        Ok(())
    }

    fn check_node(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::Function(func) => self.check_function(func),
            Node::Variable(var) => self.check_variable(var),
            Node::Struct(struct_def) => self.check_struct(struct_def),
            Node::Enum(enum_def) => self.check_enum(enum_def),
            Node::Import(_) => Ok(()),
            Node::Expression(expr) => {
                self.infer_expression(expr)?;
                Ok(())
            }
        }
    }

    fn check_function(&mut self, func: &FunctionDef) -> Result<(), String> {
        for param in &func.params {
            self.env.insert(param.name.clone(), param.type_.clone());
        }
        
        if let Some(ref return_type) = func.return_type {
            self.env.insert("return_type".to_string(), return_type.clone());
        }
        
        self.check_block(&func.body)?;
        
        for param in &func.params {
            self.env.remove(&param.name);
        }
        self.env.remove("return_type");
        
        Ok(())
    }

    fn check_variable(&mut self, var: &VariableDef) -> Result<(), String> {
        let expr_type = self.infer_expression(&var.value)?;
        
        if let Some(ref var_type) = var.type_ {
            if !self.types_match(&expr_type, var_type) {
                self.errors.push(TypeCheckError {
                    message: format!(
                        "Type mismatch: expected {}, got {}",
                        var_type, expr_type
                    ),
                    line: 0,
                });
            }
        }
        
        self.env.insert(var.name.clone(), expr_type);
        Ok(())
    }

    fn check_struct(&mut self, struct_def: &crate::ast::StructDef) -> Result<(), String> {
        for field in &struct_def.fields {
            self.env.insert(format!("{}.{}", struct_def.name, field.name), field.type_.clone());
        }
        Ok(())
    }

    fn check_enum(&mut self, enum_def: &crate::ast::EnumDef) -> Result<(), String> {
        for variant in &enum_def.variants {
            if let Some(ref data) = variant.data {
                self.env.insert(format!("{}.{}", enum_def.name, variant.name), data.clone());
            } else {
                self.env.insert(format!("{}.{}", enum_def.name, variant.name), Type::Void);
            }
        }
        Ok(())
    }

    fn check_block(&mut self, block: &crate::ast::Block) -> Result<(), String> {
        for stmt in &block.statements {
            self.check_statement(stmt)?;
        }
        Ok(())
    }

    fn check_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Expression(expr) => {
                self.infer_expression(expr)?;
            }
            Statement::Variable(var) => {
                self.check_variable(var)?;
            }
            Statement::Return(expr) => {
                if let Some(ref e) = expr {
                    self.infer_expression(e)?;
                }
            }
            Statement::If(if_expr) => {
                let cond_type = self.infer_expression(&if_expr.condition)?;
                if !self.is_bool(&cond_type) {
                    self.errors.push(TypeCheckError {
                        message: format!("If condition must be bool, got {}", cond_type),
                        line: 0,
                    });
                }
                self.check_block(&if_expr.then_block)?;
                if let Some(ref else_block) = if_expr.else_block {
                    self.check_block(else_block)?;
                }
            }
            Statement::Loop(loop_expr) => {
                self.check_block(&loop_expr.body)?;
            }
            Statement::For(for_expr) => {
                self.infer_expression(&for_expr.iterable)?;
                self.check_block(&for_expr.body)?;
            }
            Statement::Match(match_expr) => {
                self.infer_expression(&match_expr.value)?;
                for arm in &match_expr.arms {
                    self.check_block(&arm.body)?;
                }
            }
            Statement::Break | Statement::Continue => {}
        }
        Ok(())
    }

    fn infer_expression(&mut self, expr: &Expression) -> Result<Type, String> {
        match expr {
            Expression::Literal(lit) => Ok(self.literal_type(lit)),
            Expression::Identifier(name) => {
                if let Some(type_) = self.env.get(name) {
                    Ok(type_.clone())
                } else {
                    self.errors.push(TypeCheckError {
                        message: format!("Undefined variable: {}", name),
                        line: 0,
                    });
                    Ok(Type::Infer)
                }
            }
            Expression::BinaryOp(left, op, right) => {
                let left_type = self.infer_expression(left)?;
                let right_type = self.infer_expression(right)?;
                
                let expected = match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                        self.check_numeric(&left_type)?;
                        self.check_numeric(&right_type)?;
                        left_type
                    }
                    BinOp::And | BinOp::Or => {
                        self.is_bool(&left_type);
                        self.is_bool(&right_type);
                        Type::Bool
                    }
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                        Type::Bool
                    }
                    BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::Shl | BinOp::Shr => {
                        self.check_integer(&left_type)?;
                        self.check_integer(&right_type)?;
                        left_type
                    }
                };
                Ok(expected)
            }
            Expression::UnaryOp(op, expr) => {
                let inner_type = self.infer_expression(expr)?;
                match op {
                    UnaryOp::Neg | UnaryOp::BitNot => {
                        self.check_numeric(&inner_type)?;
                        Ok(inner_type)
                    }
                    UnaryOp::Not => {
                        self.is_bool(&inner_type);
                        Ok(Type::Bool)
                    }
                    UnaryOp::Deref => Ok(Type::Infer),
                }
            }
            Expression::Call { func, args } => {
                self.infer_expression(func)?;
                for arg in args {
                    self.infer_expression(arg)?;
                }
                Ok(Type::Infer)
            }
            Expression::MethodCall { object, method, args } => {
                self.infer_expression(object)?;
                for arg in args {
                    self.infer_expression(arg)?;
                }
                Ok(Type::Infer)
            }
            Expression::FieldAccess { object, field } => {
                self.infer_expression(object)?;
                Ok(Type::Infer)
            }
            Expression::Index { collection, index } => {
                self.infer_expression(collection)?;
                self.infer_expression(index)?;
                Ok(Type::Infer)
            }
            Expression::Tuple(exprs) => {
                for e in exprs {
                    self.infer_expression(e)?;
                }
                Ok(Type::Infer)
            }
            Expression::Array(exprs) => {
                for e in exprs {
                    self.infer_expression(e)?;
                }
                Ok(Type::Infer)
            }
            Expression::Block(block) => {
                self.check_block(block)?;
                Ok(Type::Infer)
            }
            Expression::Closure { params, body } => {
                for param in params {
                    self.env.insert(param.name.clone(), param.type_.clone());
                }
                self.check_block(body)?;
                for param in params {
                    self.env.remove(&param.name);
                }
                Ok(Type::Infer)
            }
            Expression::If(if_expr) => {
                let cond_type = self.infer_expression(&if_expr.condition)?;
                if !self.is_bool(&cond_type) {
                    self.errors.push(TypeCheckError {
                        message: format!("If condition must be bool, got {}", cond_type),
                        line: 0,
                    });
                }
                self.check_block(&if_expr.then_block)?;
                if let Some(ref else_block) = if_expr.else_block {
                    self.check_block(else_block)?;
                }
                Ok(Type::Infer)
            }
            Expression::Match(match_expr) => {
                self.infer_expression(&match_expr.value)?;
                for arm in &match_expr.arms {
                    self.check_block(&arm.body)?;
                }
                Ok(Type::Infer)
            }
            Expression::Assign { target, value } => {
                self.infer_expression(target)?;
                self.infer_expression(value)?;
                Ok(Type::Infer)
            }
        }
    }

    fn literal_type(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Bool(_) => Type::Bool,
            Literal::Char(_) => Type::Char,
            Literal::String(_) => Type::String,
            Literal::Null => Type::Infer,
        }
    }

    fn types_match(&self, a: &Type, b: &Type) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    fn is_bool(&self, t: &Type) -> bool {
        std::mem::discriminant(t) == std::mem::discriminant(&Type::Bool)
    }

    fn check_numeric(&self, t: &Type) -> Result<(), String> {
        match t {
            Type::Int | Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 |
            Type::UInt | Type::UInt8 | Type::UInt16 | Type::UInt32 | Type::UInt64 |
            Type::Float | Type::Float32 | Type::Float64 => Ok(()),
            _ => Err(format!("Expected numeric type, got {}", t)),
        }
    }

    fn check_integer(&self, t: &Type) -> Result<(), String> {
        match t {
            Type::Int | Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 |
            Type::UInt | Type::UInt8 | Type::UInt16 | Type::UInt32 | Type::UInt64 => Ok(()),
            _ => Err(format!("Expected integer type, got {}", t)),
        }
    }
}

pub fn check(ast: &CompilationUnit) -> Result<(), String> {
    TypeChecker::check(ast)
}