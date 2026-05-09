use crate::ast::{
    CompilationUnit, FunctionDef, VariableDef, Type, Literal, Block, Statement, Expression,
    BinOp, UnaryOp, IfExpr, LoopExpr, ForExpr, MatchExpr,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub instructions: Vec<IRInstruction>,
    pub terminator: Option<IRInstruction>,
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

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub blocks: Vec<BasicBlock>,
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

#[derive(Debug, Clone)]
pub struct Module {
    pub globals: Vec<(String, Type, Option<Literal>)>,
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
    Nop,
    Label(String),
    Load(String, String),
    Store(String, String),
    BinaryOp(String, String, BinOp, String),
    UnaryOp(String, UnaryOp, String),
    Call(String, String, Vec<String>),
    Return(Option<String>),
    Jump(String),
    Branch(String, String, String),
    Phi(String, Vec<(String, String)>),
    Alloca(String, Type),
    LoadFrom(String, String),
    StoreTo(String, String),
}

pub struct IRGenerator {
    current_func: Option<String>,
    current_block: Option<String>,
    temp_counter: usize,
    label_counter: usize,
    locals: HashMap<String, String>,
    blocks: Vec<BasicBlock>,
    functions: Vec<Function>,
    globals: Vec<(String, Type, Option<Literal>)>,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            current_func: None,
            current_block: None,
            temp_counter: 0,
            label_counter: 0,
            locals: HashMap::new(),
            blocks: Vec::new(),
            functions: Vec::new(),
            globals: Vec::new(),
        }
    }

    pub fn generate(ast: &CompilationUnit) -> Result<Module, String> {
        let mut generator = Self::new();
        generator.generate_unit(ast)?;
        Ok(generator.build_module())
    }

    fn generate_unit(&mut self, unit: &CompilationUnit) -> Result<(), String> {
        for node in &unit.nodes {
            match node {
                crate::ast::Node::Function(func) => self.generate_function(func)?,
                crate::ast::Node::Variable(var) => self.generate_global_var(var)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn generate_function(&mut self, func: &FunctionDef) -> Result<(), String> {
        let params: Vec<(String, Type)> = func.params.iter()
            .map(|p| (p.name.clone(), p.type_.clone()))
            .collect();
        
        let mut function = Function::new(
            func.name.clone(),
            params,
            func.return_type.clone(),
        );
        
        self.current_func = Some(func.name.clone());
        self.label_counter = 0;
        self.temp_counter = 0;
        self.blocks.clear();
        self.locals.clear();
        
        let entry_block = BasicBlock::new(self.new_label("entry"));
        self.blocks.push(entry_block);
        
        for (i, param) in func.params.iter().enumerate() {
            let param_name = format!("param_{}", param.name);
            self.locals.insert(param.name.clone(), param_name.clone());
            let block = self.blocks.last_mut().unwrap();
            block.instructions.push(IRInstruction::Alloca(
                param_name.clone(),
                param.type_.clone(),
            ));
            block.instructions.push(IRInstruction::Store(
                format!("arg_{}", i),
                param_name,
            ));
        }
        
        self.generate_block(&func.body, &mut function)?;
        // generate_block already moved blocks into function.blocks
        
        self.current_func = None;
        
        // Store the function
        println!("DEBUG generate_function: pushing function with {} blocks", function.blocks.len());
        self.functions.push(function);
        println!("DEBUG generate_function: self.functions.len()={} now", self.functions.len());
        
        Ok(())
    }

    fn generate_global_var(&mut self, _var: &VariableDef) -> Result<(), String> {
        Ok(())
    }

    fn generate_block(&mut self, block: &Block, func: &mut Function) -> Result<(), String> {
        println!("DEBUG generate_block: {} statements", block.statements.len());
        for stmt in &block.statements {
            self.generate_statement(stmt, func)?;
        }
        println!("DEBUG generate_block before take: self.blocks={}, func.blocks={}", self.blocks.len(), func.blocks.len());
        
        if func.blocks.last().and_then(|b| b.terminator.as_ref()).is_none() {
            if let Some(ref ret_type) = func.return_type {
                if *ret_type != Type::Void {
                    self.emit_instruction(IRInstruction::Return(Some("0".to_string())));
                } else {
                    self.emit_instruction(IRInstruction::Return(None));
                }
            } else {
                self.emit_instruction(IRInstruction::Return(None));
            }
        }
        
        println!("DEBUG generate_block after emit: self.blocks={}", self.blocks.len());
        func.blocks = std::mem::take(&mut self.blocks);
        println!("DEBUG generate_block after take: func.blocks={}", func.blocks.len());
        Ok(())
    }

    fn generate_statement(&mut self, stmt: &Statement, func: &mut Function) -> Result<(), String> {
        match stmt {
            Statement::Expression(expr) => {
                self.generate_expression(expr, func)?;
            }
            Statement::Variable(var) => {
                let temp = self.generate_expression(&var.value, func)?;
                let var_name = format!("var_{}", var.name);
                self.locals.insert(var.name.clone(), var_name.clone());
                self.emit_instruction(IRInstruction::Alloca(var_name.clone(), var.type_.clone().unwrap_or(Type::Infer)));
                self.emit_instruction(IRInstruction::Store(temp, var_name));
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let temp = self.generate_expression(e, func)?;
                    self.emit_instruction(IRInstruction::Return(Some(temp)));
                } else {
                    self.emit_instruction(IRInstruction::Return(None));
                }
            }
            Statement::If(if_expr) => {
                self.generate_if(if_expr, func)?;
            }
            Statement::Loop(loop_expr) => {
                self.generate_loop(loop_expr, func)?;
            }
            Statement::For(for_expr) => {
                self.generate_for(for_expr, func)?;
            }
            Statement::Match(match_expr) => {
                self.generate_match(match_expr, func)?;
            }
            Statement::Break => {
                let label = self.new_label("break");
                self.emit_instruction(IRInstruction::Jump(label));
            }
            Statement::Continue => {
                let label = self.new_label("continue");
                self.emit_instruction(IRInstruction::Jump(label));
            }
        }
        Ok(())
    }

    fn generate_expression(&mut self, expr: &Expression, func: &mut Function) -> Result<String, String> {
        match expr {
            Expression::Literal(lit) => Ok(self.generate_literal(lit)),
            Expression::Identifier(name) => {
                if let Some(var_name) = self.locals.get(name) {
                    Ok(var_name.clone())
                } else {
                    Ok(name.clone())
                }
            }
            Expression::BinaryOp(left, op, right) => {
                let left_temp = self.generate_expression(left, func)?;
                let right_temp = self.generate_expression(right, func)?;
                let result = self.new_temp();
                self.emit_instruction(IRInstruction::BinaryOp(
                    result.clone(),
                    left_temp,
                    op.clone(),
                    right_temp,
                ));
                Ok(result)
            }
            Expression::UnaryOp(op, expr) => {
                let operand = self.generate_expression(expr, func)?;
                let result = self.new_temp();
                self.emit_instruction(IRInstruction::UnaryOp(result.clone(), op.clone(), operand));
                Ok(result)
            }
            Expression::Call { func: func_expr, args } => {
                let func_name = self.generate_expression(func_expr, func)?;
                let mut arg_temps = Vec::new();
                for arg in args {
                    arg_temps.push(self.generate_expression(arg, func)?);
                }
                let result = self.new_temp();
                self.emit_instruction(IRInstruction::Call(
                    result.clone(),
                    func_name,
                    arg_temps,
                ));
                Ok(result)
            }
            Expression::MethodCall { object, method, args } => {
                let object_temp = self.generate_expression(object, func)?;
                let mut arg_temps = vec![object_temp];
                for arg in args {
                    arg_temps.push(self.generate_expression(arg, func)?);
                }
                let result = self.new_temp();
                self.emit_instruction(IRInstruction::Call(
                    result.clone(),
                    method.clone(),
                    arg_temps,
                ));
                Ok(result)
            }
            Expression::FieldAccess { object, field: _ } => {
                self.generate_expression(object, func)?;
                let result = self.new_temp();
                Ok(result)
            }
            Expression::Index { collection, index } => {
                let coll_temp = self.generate_expression(collection, func)?;
                let idx_temp = self.generate_expression(index, func)?;
                let result = self.new_temp();
                self.emit_instruction(IRInstruction::BinaryOp(
                    result.clone(),
                    coll_temp,
                    BinOp::Add,
                    idx_temp,
                ));
                Ok(result)
            }
            Expression::Tuple(exprs) => {
                for e in exprs {
                    self.generate_expression(e, func)?;
                }
                Ok(self.new_temp())
            }
            Expression::Array(exprs) => {
                for e in exprs {
                    self.generate_expression(e, func)?;
                }
                Ok(self.new_temp())
            }
            Expression::Block(block) => {
                let last = block.statements.last();
                if let Some(Statement::Expression(expr)) = last {
                    self.generate_expression(expr, func)
                } else {
                    for stmt in &block.statements {
                        self.generate_statement(stmt, func)?;
                    }
                    Ok(self.new_temp())
                }
            }
            Expression::Closure { params, body } => {
                for param in params {
                    self.locals.insert(param.name.clone(), format!("closure_{}", param.name));
                }
                self.generate_block(body, func)?;
                for param in params {
                    self.locals.remove(&param.name);
                }
                Ok(self.new_temp())
            }
            Expression::If(if_expr) => {
                self.generate_if(if_expr, func)?;
                Ok(self.new_temp())
            }
            Expression::Match(match_expr) => {
                self.generate_match(match_expr, func)?;
                Ok(self.new_temp())
            }
            Expression::Assign { target, value } => {
                let var_name = self.generate_expression(target, func)?;
                let val_temp = self.generate_expression(value, func)?;
                self.emit_instruction(IRInstruction::Store(val_temp, var_name.clone()));
                Ok(var_name)
            }
        }
    }

    fn generate_if(&mut self, if_expr: &IfExpr, func: &mut Function) -> Result<(), String> {
        let cond_temp = self.generate_expression(&if_expr.condition, func)?;
        let then_label = self.new_label("then");
        let else_label = self.new_label("else");
        let merge_label = self.new_label("merge");
        
        self.emit_instruction(IRInstruction::Branch(
            cond_temp,
            then_label.clone(),
            else_label.clone(),
        ));
        
        self.emit_label(then_label);
        for stmt in &if_expr.then_block.statements {
            self.generate_statement(stmt, func)?;
        }
        self.emit_instruction(IRInstruction::Jump(merge_label.clone()));
        
        if let Some(ref else_block) = if_expr.else_block {
            self.emit_label(else_label);
            for stmt in &else_block.statements {
                self.generate_statement(stmt, func)?;
            }
            self.emit_instruction(IRInstruction::Jump(merge_label.clone()));
        } else {
            self.emit_label(else_label);
            self.emit_instruction(IRInstruction::Jump(merge_label.clone()));
        }
        
        self.emit_label(merge_label);
        Ok(())
    }

    fn generate_loop(&mut self, loop_expr: &LoopExpr, func: &mut Function) -> Result<(), String> {
        let header_label = self.new_label("loop_header");
        let body_label = self.new_label("loop_body");
        let exit_label = self.new_label("loop_exit");
        
        self.emit_label(header_label.clone());
        self.emit_instruction(IRInstruction::Jump(body_label.clone()));
        
        self.emit_label(body_label);
        for stmt in &loop_expr.body.statements {
            self.generate_statement(stmt, func)?;
        }
        self.emit_instruction(IRInstruction::Jump(header_label));
        
        self.emit_label(exit_label);
        Ok(())
    }

    fn generate_for(&mut self, for_expr: &ForExpr, func: &mut Function) -> Result<(), String> {
        let iter_temp = self.generate_expression(&for_expr.iterable, func)?;
        let var_name = format!("for_{}", for_expr.variable);
        self.locals.insert(for_expr.variable.clone(), var_name.clone());
        
        let header_label = self.new_label("for_header");
        let body_label = self.new_label("for_body");
        let exit_label = self.new_label("for_exit");
        
        self.emit_label(header_label.clone());
        self.emit_instruction(IRInstruction::LoadFrom(var_name.clone(), iter_temp));
        self.emit_instruction(IRInstruction::Jump(body_label.clone()));
        
        self.emit_label(body_label);
        for stmt in &for_expr.body.statements {
            self.generate_statement(stmt, func)?;
        }
        self.emit_instruction(IRInstruction::Jump(header_label));
        
        self.emit_label(exit_label);
        Ok(())
    }

    fn generate_match(&mut self, match_expr: &MatchExpr, func: &mut Function) -> Result<(), String> {
        let _value_temp = self.generate_expression(&match_expr.value, func)?;
        
        for arm in &match_expr.arms {
            let arm_label = self.new_label("match_arm");
            self.emit_label(arm_label);
            for stmt in &arm.body.statements {
                self.generate_statement(stmt, func)?;
            }
            let exit_label = self.new_label("match_exit");
            self.emit_instruction(IRInstruction::Jump(exit_label));
        }
        
        let exit_label = self.new_label("match_exit");
        self.emit_label(exit_label);
        Ok(())
    }

    fn generate_literal(&self, lit: &Literal) -> String {
        match lit {
            Literal::Int(n) => n.to_string(),
            Literal::Float(n) => n.to_string(),
            Literal::Bool(b) => if *b { "1".to_string() } else { "0".to_string() },
            Literal::Char(c) => format!("'{}'", c),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Null => "NULL".to_string(),
        }
    }

    fn emit_instruction(&mut self, instr: IRInstruction) {
        println!("DEBUG emit_instruction: {} blocks in self.blocks", self.blocks.len());
        if let Some(block) = self.blocks.last_mut() {
            block.instructions.push(instr);
            println!("DEBUG: Added instruction to block {}", block.label);
        } else {
            println!("DEBUG: No block to add instruction to!");
        }
    }

    fn emit_label(&mut self, label: String) {
        if let Some(block) = self.blocks.last_mut() {
            block.terminator = Some(IRInstruction::Label(label));
        }
        let new_block = BasicBlock::new(self.new_label("temp"));
        self.blocks.push(new_block);
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("%{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn build_module(mut self) -> Module {
        let mut module = Module::new();
        module.functions = self.functions;
        module.globals = self.globals;
        module
    }
}

pub fn generate(ast: &CompilationUnit) -> Result<Module, String> {
    IRGenerator::generate(ast)
}
