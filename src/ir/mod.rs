use crate::ast::{
    CompilationUnit, Node, FunctionDef, VariableDef, Statement, Expression,
    Literal, BinOp, UnaryOp, Type,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum IRInstruction {
    ConstInt(i64),
    ConstFloat(f64),
    ConstBool(bool),
    ConstString(String),
    ConstChar(char),
    
    Load(String),
    Store(String),
    
    BinaryOp(BinOp, String, String),
    UnaryOp(UnaryOp, String),
    
    Jump(String),
    Branch(String, String, String),
    Return(Option<String>),
    
    Call(String, Vec<String>),
    Arg(String),
    
    Alloca(String, Type),
    LoadVar(String, String),
    StoreVar(String, String),
    
    Phi(Vec<(String, String)>),
    
    Label(String),
    
    PhiMerge(String),
    
    Nop,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<IRInstruction>,
    pub terminator: Option<IRInstruction>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub globals: Vec<(String, Type, Option<Literal>)>,
}

impl BasicBlock {
    fn new(label: String) -> Self {
        Self {
            label,
            instructions: Vec::new(),
            terminator: None,
        }
    }
}

impl Function {
    fn new(name: String, params: Vec<(String, Type)>, return_type: Option<Type>) -> Self {
        Self {
            name,
            params,
            return_type,
            blocks: Vec::new(),
        }
    }
}

pub struct IRGenerator {
    module: Module,
    current_func: Option<String>,
    block_counter: usize,
    temp_counter: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            module: Module {
                functions: Vec::new(),
                globals: Vec::new(),
            },
            current_func: None,
            block_counter: 0,
            temp_counter: 0,
        }
    }

    pub fn generate(ast: &CompilationUnit) -> Result<Module, String> {
        let mut gen = Self::new();
        gen.generate_unit(ast)?;
        Ok(gen.module)
    }

    fn generate_unit(&mut self, unit: &CompilationUnit) -> Result<(), String> {
        for node in &unit.nodes {
            match node {
                Node::Function(func) => self.generate_function(func)?,
                Node::Variable(var) => self.generate_global_var(var)?,
                Node::Struct(_) | Node::Enum(_) | Node::Import(_) => {}
                Node::Expression(expr) => {
                    let _ = self.generate_expression(expr)?;
                }
            }
        }
        Ok(())
    }

    fn generate_function(&mut self, func: &FunctionDef) -> Result<(), String> {
        let params: Vec<(String, Type)> = func.params
            .iter()
            .map(|p| (p.name.clone(), p.type_.clone()))
            .collect();
        
        let mut function = Function::new(
            func.name.clone(),
            params,
            func.return_type.clone(),
        );
        
        let entry_label = self.new_label("entry");
        let mut entry_block = BasicBlock::new(entry_label);
        
        for (i, param) in func.params.iter().enumerate() {
            entry_block.instructions.push(IRInstruction::Alloca(
                param.name.clone(),
                param.type_.clone(),
            ));
            entry_block.instructions.push(IRInstruction::StoreVar(
                format!("%{}", i),
                param.name.clone(),
            ));
        }
        
        self.current_func = Some(func.name.clone());
        self.generate_block(&func.body, &mut entry_block)?;
        
        if entry_block.terminator.is_none() {
            entry_block.terminator = Some(IRInstruction::Return(None));
        }
        
        function.blocks.push(entry_block);
        self.current_func = None;
        
        self.module.functions.push(function);
        Ok(())
    }

    fn generate_global_var(&mut self, var: &VariableDef) -> Result<(), String> {
        let lit = match &var.value {
            Expression::Literal(l) => Some(l.clone()),
            _ => None,
        };
        self.module.globals.push((
            var.name.clone(),
            var.type_.clone().unwrap_or(Type::Infer),
            lit,
        ));
        Ok(())
    }

    fn generate_block(&mut self, block: &crate::ast::Block, current_block: &mut BasicBlock) -> Result<(), String> {
        for stmt in &block.statements {
            self.generate_statement(stmt, current_block)?;
        }
        Ok(())
    }

    fn generate_statement(&mut self, stmt: &Statement, block: &mut BasicBlock) -> Result<(), String> {
        match stmt {
            Statement::Expression(expr) => {
                let _ = self.generate_expression(expr)?;
            }
            Statement::Variable(var) => {
                block.instructions.push(IRInstruction::Alloca(
                    var.name.clone(),
                    var.type_.clone().unwrap_or(Type::Infer),
                ));
                let val = self.generate_expression(&var.value)?;
                block.instructions.push(IRInstruction::StoreVar(val, var.name.clone()));
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let val = self.generate_expression(e)?;
                    block.terminator = Some(IRInstruction::Return(Some(val)));
                } else {
                    block.terminator = Some(IRInstruction::Return(None));
                }
            }
            Statement::If(if_expr) => {
                let cond = self.generate_expression(&if_expr.condition)?;
                
                let then_label = self.new_label("then");
                let else_label = if_expr.else_block.is_some() 
                    .then(|| self.new_label("else"))
                    .unwrap_or_else(|| self.new_label("end"));
                let end_label = self.new_label("end");
                
                block.terminator = Some(IRInstruction::Branch(
                    cond,
                    then_label.clone(),
                    else_label.clone(),
                ));
                
                let mut then_block = BasicBlock::new(then_label);
                self.generate_block(&if_expr.then_block, &mut then_block)?;
                if then_block.terminator.is_none() {
                    then_block.terminator = Some(IRInstruction::Jump(end_label.clone()));
                }
                block.instructions.push(IRInstruction::Label(then_label.clone()));
                block.instructions.extend(then_block.instructions);
                if let Some(terminator) = then_block.terminator {
                    block.instructions.push(terminator);
                }
                
                if let Some(ref else_block) = if_expr.else_block {
                    let mut else_blk = BasicBlock::new(else_label.clone());
                    self.generate_block(else_block, &mut else_blk)?;
                    if else_blk.terminator.is_none() {
                        else_blk.terminator = Some(IRInstruction::Jump(end_label.clone()));
                    }
                    block.instructions.push(IRInstruction::Label(else_label.clone()));
                    block.instructions.extend(else_blk.instructions);
                    if let Some(terminator) = else_blk.terminator {
                        block.instructions.push(terminator);
                    }
                } else {
                    block.instructions.push(IRInstruction::Label(else_label.clone()));
                }
                
                block.instructions.push(IRInstruction::Label(end_label.clone()));
            }
            Statement::Loop(loop_expr) => {
                let body_label = self.new_label("loop_body");
                let end_label = self.new_label("loop_end");
                
                block.instructions.push(IRInstruction::Label(body_label.clone()));
                
                let mut body_block = BasicBlock::new(body_label.clone());
                self.generate_block(&loop_expr.body, &mut body_block)?;
                if body_block.terminator.is_none() {
                    body_block.terminator = Some(IRInstruction::Jump(body_label.clone()));
                }
                block.instructions.extend(body_block.instructions);
                if let Some(terminator) = body_block.terminator {
                    block.instructions.push(terminator);
                }
                
                block.instructions.push(IRInstruction::Label(end_label.clone()));
            }
            Statement::For(for_expr) => {
                let iter = self.generate_expression(&for_expr.iterable)?;
                let body_label = self.new_label("for_body");
                let end_label = self.new_label("for_end");
                
                block.instructions.push(IRInstruction::Alloca(
                    for_expr.variable.clone(),
                    Type::Infer,
                ));
                block.instructions.push(IRInstruction::StoreVar(
                    iter,
                    for_expr.variable.clone(),
                ));
                block.instructions.push(IRInstruction::Label(body_label.clone()));
                
                let mut body_block = BasicBlock::new(body_label.clone());
                self.generate_block(&for_expr.body, &mut body_block)?;
                if body_block.terminator.is_none() {
                    body_block.terminator = Some(IRInstruction::Jump(body_label.clone()));
                }
                block.instructions.extend(body_block.instructions);
                if let Some(terminator) = body_block.terminator {
                    block.instructions.push(terminator);
                }
                
                block.instructions.push(IRInstruction::Label(end_label.clone()));
            }
            Statement::Match(match_expr) => {
                let val = self.generate_expression(&match_expr.value)?;
                
                for arm in &match_expr.arms {
                    let arm_label = self.new_label("match_arm");
                    block.instructions.push(IRInstruction::Label(arm_label.clone()));
                    self.generate_block(&arm.body, block)?;
                }
                
                let end_label = self.new_label("match_end");
                block.instructions.push(IRInstruction::Label(end_label.clone()));
            }
            Statement::Break => {
                let end_label = self.new_label("break_target");
                block.terminator = Some(IRInstruction::Jump(end_label.clone()));
            }
            Statement::Continue => {
                let cont_label = self.new_label("continue_target");
                block.terminator = Some(IRInstruction::Jump(cont_label.clone()));
            }
        }
        Ok(())
    }

    fn generate_expression(&mut self, expr: &Expression) -> Result<String, String> {
        match expr {
            Expression::Literal(lit) => {
                let temp = self.new_temp();
                match lit {
                    Literal::Int(n) => {
                        self.module.globals.push((
                            temp.clone(),
                            Type::Int,
                            Some(Literal::Int(*n)),
                        ));
                    }
                    Literal::Float(n) => {
                        self.module.globals.push((
                            temp.clone(),
                            Type::Float,
                            Some(Literal::Float(*n)),
                        ));
                    }
                    Literal::Bool(b) => {
                        self.module.globals.push((
                            temp.clone(),
                            Type::Bool,
                            Some(Literal::Bool(*b)),
                        ));
                    }
                    Literal::String(s) => {
                        self.module.globals.push((
                            temp.clone(),
                            Type::String,
                            Some(Literal::String(s.clone())),
                        ));
                    }
                    Literal::Char(c) => {
                        self.module.globals.push((
                            temp.clone(),
                            Type::Char,
                            Some(Literal::Char(*c)),
                        ));
                    }
                    Literal::Null => {}
                }
                Ok(temp)
            }
            Expression::Identifier(name) => {
                Ok(format!("%{}", name))
            }
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.generate_expression(left)?;
                let right_val = self.generate_expression(right)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::UnaryOp(op, expr) => {
                let val = self.generate_expression(expr)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Call { func, args } => {
                let func_name = match func.as_ref() {
                    Expression::Identifier(name) => name.clone(),
                    _ => return Err("Only identifier calls supported".to_string()),
                };
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::MethodCall { object, method, args } => {
                let _ = self.generate_expression(object)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::FieldAccess { object, field } => {
                let _ = self.generate_expression(object)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Index { collection, index } => {
                let _ = self.generate_expression(collection)?;
                let _ = self.generate_expression(index)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Tuple(exprs) => {
                for e in exprs {
                    let _ = self.generate_expression(e)?;
                }
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Array(exprs) => {
                for e in exprs {
                    let _ = self.generate_expression(e)?;
                }
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Block(block) => {
                let mut dummy_block = BasicBlock::new("dummy".to_string());
                self.generate_block(block, &mut dummy_block)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Closure { params, body } => {
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::If(if_expr) => {
                let _ = self.generate_expression(&if_expr.condition)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Match(match_expr) => {
                let _ = self.generate_expression(&match_expr.value)?;
                let temp = self.new_temp();
                Ok(temp)
            }
            Expression::Assign { target, value } => {
                let _ = self.generate_expression(target)?;
                let val = self.generate_expression(value)?;
                let temp = self.new_temp();
                Ok(temp)
            }
        }
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}{}", prefix, self.block_counter);
        self.block_counter += 1;
        label
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }
}

pub fn generate(ast: &CompilationUnit) -> Result<Module, String> {
    IRGenerator::generate(ast)
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for global in &self.globals {
            writeln!(f, "global {} {} = {:?}", global.0, global.1, global.2)?;
        }
        
        for func in &self.functions {
            writeln!(f, "fn {}(", func.name)?;
            for (i, (name, type_)) in func.params.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", name, type_)?;
            }
            writeln!(f, ")")?;
            
            for block in &func.blocks {
                writeln!(f, "  {}:", block.label)?;
                for instr in &block.instructions {
                    writeln!(f, "    {:?}", instr)?;
                }
                if let Some(ref term) = block.terminator {
                    writeln!(f, "    {:?}", term)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}