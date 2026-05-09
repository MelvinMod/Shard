use std::path::PathBuf;

use crate::ir::Module;
use std::fs;

pub struct CodeGenerator {
    output: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    pub fn compile(module: &Module, output_path: &PathBuf, _opt_level: u8) -> Result<(), String> {
        let mut gen = Self::new();
        gen.generate(module)?;
        
        let c_code = gen.get_output();
        let c_file_path = output_path.with_extension("c");
        println!("DEBUG: Writing C file to: {}", c_file_path.display());
        fs::write(&c_file_path, &c_code)
            .map_err(|e| format!("Failed to write C file: {}", e))?;
        
        println!("DEBUG: C file written, size: {} bytes", c_code.len());
        
        Self::compile_to_native(output_path, &c_file_path)?;
        
        Ok(())
    }

    fn generate(&mut self, module: &Module) -> Result<(), String> {
        self.emit("// Shard Programming Language - Generated C Code");
        self.emit("// Author: MelvinSGjr (MelvinMod)");
        self.emit("");
        
        self.emit("#include <stdio.h>");
        self.emit("#include <stdlib.h>");
        self.emit("#include <string.h>");
        self.emit("#include <stdbool.h>");
        self.emit("#include <stdint.h>");
        self.emit("");
        
        self.emit("typedef int64_t Int;");
        self.emit("typedef int8_t Int8;");
        self.emit("typedef int16_t Int16;");
        self.emit("typedef int32_t Int32;");
        self.emit("typedef int64_t Int64;");
        self.emit("typedef uint64_t UInt;");
        self.emit("typedef uint8_t UInt8;");
        self.emit("typedef uint16_t UInt16;");
        self.emit("typedef uint32_t UInt32;");
        self.emit("typedef uint64_t UInt64;");
        self.emit("typedef float Float32;");
        self.emit("typedef double Float64;");
        self.emit("typedef char* String;");
        self.emit("");
        
        // Check if main function exists
        let has_main = module.functions.iter().any(|f| f.name == "main");
        
        for global in &module.globals {
            self.generate_global(global);
        }
        
        for func in &module.functions {
            if func.name == "main" {
                // Rename user's main to shard_main
                self.generate_function_with_name(func, "shard_main");
            } else {
                self.generate_function(func);
            }
        }
        
        self.generate_main_wrapper(has_main);
        
        Ok(())
    }

    fn generate_global(&mut self, global: &(String, crate::ast::Type, Option<crate::ast::Literal>)) {
        let type_str = self.map_type(&global.1);
        let value = match &global.2 {
            Some(crate::ast::Literal::Int(n)) => format!("{}", n),
            Some(crate::ast::Literal::Float(n)) => format!("{}", n),
            Some(crate::ast::Literal::Bool(b)) => format!("{}", b),
            Some(crate::ast::Literal::String(s)) => format!("\"{}\"", s),
            Some(crate::ast::Literal::Char(c)) => format!("'{}'", c),
            Some(crate::ast::Literal::Null) => "NULL".to_string(),
            None => "0".to_string(),
        };
        self.emit(&format!("{} {} = {};", type_str, global.0, value));
    }

    fn generate_function(&mut self, func: &crate::ir::Function) {
        self.generate_function_with_name(func, &func.name);
    }
    
    fn generate_function_with_name(&mut self, func: &crate::ir::Function, name: &str) {
        println!("DEBUG codegen: Generating function {} with {} blocks", name, func.blocks.len());
        let ret_type = func.return_type
            .as_ref()
            .map(|t| self.map_type(t))
            .unwrap_or_else(|| "void".to_string());
        
        self.emit(&format!("{} {}(", ret_type, name));
        
        let mut first = true;
        for (name, type_) in &func.params {
            if !first {
                self.emit(", ");
            }
            first = false;
            self.emit(&format!("{} {}", self.map_type(type_), name));
        }
        self.emit(") {");
        
        for block in &func.blocks {
            self.emit(&format!("  {}: ", block.label));
            for instr in &block.instructions {
                self.generate_instruction(instr);
            }
            if let Some(ref term) = block.terminator {
                self.generate_terminator(term);
            }
        }
        
        self.emit("}");
        self.emit("");
    }

    fn generate_instruction(&mut self, instr: &crate::ir::IRInstruction) {
        match instr {
            crate::ir::IRInstruction::Nop => {}
            crate::ir::IRInstruction::Label(label) => {
                self.emit(&format!("    {}:", label));
            }
            crate::ir::IRInstruction::Call(result, func_name, args) => {
                let args_str = args.join(", ");
                if result.starts_with('%') && args.is_empty() {
                    // Void call - just execute
                    self.emit(&format!("    {}({});", func_name, args_str));
                } else {
                    self.emit(&format!("    {} = {}({});", result, func_name, args_str));
                }
            }
            crate::ir::IRInstruction::Alloca(name, _type) => {
                self.emit(&format!("    {} {};", self.map_type(_type), name));
            }
            crate::ir::IRInstruction::Store(src, dst) => {
                self.emit(&format!("    {} = {};", dst, src));
            }
            crate::ir::IRInstruction::Load(dst, src) => {
                self.emit(&format!("    {} = {};", dst, src));
            }
            crate::ir::IRInstruction::BinaryOp(result, left, op, right) => {
                let op_str = match op {
                    crate::ast::BinOp::Add => "+",
                    crate::ast::BinOp::Sub => "-",
                    crate::ast::BinOp::Mul => "*",
                    crate::ast::BinOp::Div => "/",
                    crate::ast::BinOp::Mod => "%",
                    crate::ast::BinOp::Eq => "==",
                    crate::ast::BinOp::Ne => "!=",
                    crate::ast::BinOp::Lt => "<",
                    crate::ast::BinOp::Le => "<=",
                    crate::ast::BinOp::Gt => ">",
                    crate::ast::BinOp::Ge => ">=",
                    crate::ast::BinOp::And => "&&",
                    crate::ast::BinOp::Or => "||",
                    _ => "?",
                };
                self.emit(&format!("    {} = {} {} {};", result, left, op_str, right));
            }
            crate::ir::IRInstruction::Return(val) => {
                if let Some(v) = val {
                    self.emit(&format!("    return {};", v));
                } else {
                    self.emit("    return;");
                }
            }
            _ => {
                self.emit(&format!("    // {:?}", instr));
            }
        }
    }

    fn generate_terminator(&mut self, term: &crate::ir::IRInstruction) {
        match term {
            crate::ir::IRInstruction::Return(val) => {
                if let Some(v) = val {
                    self.emit(&format!("    return {};", v));
                } else {
                    self.emit("    return;");
                }
            }
            crate::ir::IRInstruction::Jump(label) => {
                self.emit(&format!("    goto {};", label));
            }
            crate::ir::IRInstruction::Branch(cond, then_lbl, else_lbl) => {
                self.emit(&format!("    if ({}) goto {}; else goto {};", cond, then_lbl, else_lbl));
            }
            _ => {
                self.emit(&format!("    // {:?}", term));
            }
        }
    }

    fn generate_main_wrapper(&mut self, has_main: bool) {
        self.emit("int main(int argc, char** argv) {");
        self.emit("    (void)argc;");
        self.emit("    (void)argv;");
        self.emit("");
        
        if has_main {
            self.emit("    shard_main();");
        } else {
            self.emit("    printf(\"No main function found!\\n\");");
        }
        
        self.emit("");
        self.emit("    return 0;");
        self.emit("}");
    }

    fn map_type(&self, type_: &crate::ast::Type) -> String {
        match type_ {
            crate::ast::Type::Int | crate::ast::Type::Int64 => "Int64".to_string(),
            crate::ast::Type::Int8 => "Int8".to_string(),
            crate::ast::Type::Int16 => "Int16".to_string(),
            crate::ast::Type::Int32 => "Int32".to_string(),
            crate::ast::Type::UInt => "UInt64".to_string(),
            crate::ast::Type::UInt8 => "UInt8".to_string(),
            crate::ast::Type::UInt16 => "UInt16".to_string(),
            crate::ast::Type::UInt32 => "UInt32".to_string(),
            crate::ast::Type::UInt64 => "UInt64".to_string(),
            crate::ast::Type::Float | crate::ast::Type::Float64 => "Float64".to_string(),
            crate::ast::Type::Float32 => "Float32".to_string(),
            crate::ast::Type::Bool => "bool".to_string(),
            crate::ast::Type::Bool8 => "bool".to_string(),
            crate::ast::Type::Char => "char".to_string(),
            crate::ast::Type::String => "String".to_string(),
            crate::ast::Type::Void => "void".to_string(),
            crate::ast::Type::Nil => "void".to_string(),
            crate::ast::Type::Pointer(_) => "void*".to_string(),
            crate::ast::Type::Array(_, _) => "void*".to_string(),
            crate::ast::Type::Slice(_) => "void*".to_string(),
            crate::ast::Type::Reference(_) => "void*".to_string(),
            crate::ast::Type::Function(_, _) => "void*".to_string(),
            crate::ast::Type::UserDefined(name) => name.clone(),
            crate::ast::Type::Generic(name, _) => name.clone(),
            crate::ast::Type::Tuple(_) => "void".to_string(),
            crate::ast::Type::Union(_) => "void".to_string(),
            crate::ast::Type::Infer => "void".to_string(),
            crate::ast::Type::Tensor => "void*".to_string(),
            crate::ast::Type::Model => "void*".to_string(),
            crate::ast::Type::Layer => "void*".to_string(),
            crate::ast::Type::Dataset => "void*".to_string(),
            crate::ast::Type::NeuralNetwork => "void*".to_string(),
            crate::ast::Type::TrainingConfig => "void*".to_string(),
            crate::ast::Type::InferenceConfig => "void*".to_string(),
            crate::ast::Type::Optimizer => "void*".to_string(),
            crate::ast::Type::LossFunction => "void*".to_string(),
            crate::ast::Type::ActivationFunction => "void*".to_string(),
            crate::ast::Type::Embedding => "void*".to_string(),
            crate::ast::Type::Attention => "void*".to_string(),
            crate::ast::Type::Transformer => "void*".to_string(),
            crate::ast::Type::RNN => "void*".to_string(),
            crate::ast::Type::CNN => "void*".to_string(),
        }
    }

    fn emit(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn get_output(&self) -> String {
        self.output.clone()
    }

    fn compile_to_native(output_path: &PathBuf, c_file: &PathBuf) -> Result<(), String> {
        let c_code = fs::read_to_string(c_file)
            .map_err(|e| format!("Failed to read C file '{}': {}", c_file.display(), e))?;
        
        println!("DEBUG: Read C file, size: {} bytes", c_code.len());
        
        let platform = Self::detect_platform();
        let compiler = Self::get_compiler(&platform);
        
        let output = std::process::Command::new(compiler)
            .arg(c_file)
            .arg("-o")
            .arg(output_path)
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .output()
            .map_err(|e| format!("Failed to invoke compiler '{}': {}", compiler, e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("C compilation failed:\n{}", stderr));
        }
        
        if !output_path.exists() {
            return Err(format!("Compiler succeeded but output file '{}' was not created", output_path.display()));
        }
        
        println!("  [+] Native compilation successful: {}", output_path.display());
        // fs::remove_file(c_file).ok();  // Keep for debugging
        
        Ok(())
    }

    fn detect_platform() -> String {
        if cfg!(target_os = "linux") {
            "linux".to_string()
        } else if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "freebsd") {
            "freebsd".to_string()
        } else if cfg!(target_os = "macos") {
            "macos".to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn get_compiler(platform: &str) -> &'static str {
        match platform {
            "windows" => "gcc",
            "linux" | "freebsd" | "macos" => "cc",
            _ => "cc",
        }
    }
}

pub fn compile(module: &Module, output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
    CodeGenerator::compile(module, output_path, opt_level)
}