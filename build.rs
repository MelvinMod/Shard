use std::fs;
use std::path::PathBuf;

#[path = "src/ast.rs"]
mod ast;
#[path = "src/ir/mod.rs"]
mod ir;

use ir::Module;

pub struct CodeGenerator {
    output: String,
    opt_level: u8,
    module: ir::Module,
}

impl CodeGenerator {
    pub fn new(opt_level: u8, module: Module) -> Self {
        Self {
            output: String::new(),
            opt_level,
            module,
        }
    }

    pub fn compile(module: &Module, output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
        let mut gen = Self::new(opt_level, module.clone());
        gen.generate(module)?;
        
        let c_code = gen.get_output();
        fs::write(output_path.with_extension("c"), c_code)
            .map_err(|e| format!("Failed to write C file: {}", e))?;
        
        Self::compile_to_native(output_path, opt_level)?;
        
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
        
        for global in &module.globals {
            self.generate_global(global);
        }
        
        for func in &module.functions {
            self.generate_function(func);
        }
        
        self.generate_main_wrapper();
        
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
        let ret_type = func.return_type
            .as_ref()
            .map(|t| self.map_type(t))
            .unwrap_or_else(|| "void".to_string());
        
        self.emit(&format!("{} {}(", ret_type, func.name));
        
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

    fn generate_main_wrapper(&mut self) {
        self.emit("int main(int argc, char** argv) {");
        self.emit("    (void)argc;");
        self.emit("    (void)argv;");
        self.emit("");
        self.emit("    printf(\"No main function found!\\n\");");
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

    fn compile_to_native(output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
        let c_file = output_path.with_extension("c");
        let _c_code = fs::read_to_string(&c_file)
            .map_err(|e| format!("Failed to read C file: {}", e))?;
        
        let optimize_flag = match opt_level {
            0 => "-O0",
            1 => "-O1",
            2 => "-O2",
            3 => "-O3",
            _ => "-O2",
        };
        
        let platform = Self::detect_platform();
        let compiler = Self::get_compiler(&platform);
        
        let status = std::process::Command::new(compiler)
            .arg(&c_file)
            .arg("-o")
            .arg(output_path)
            .arg(optimize_flag)
            .arg("-Wall")
            .arg("-Wextra")
            .status()
            .map_err(|e| format!("Compilation failed: {}", e))?;
        
        if status.success() {
            println!("  [+] Native compilation successful: {}", output_path.display());
        } else {
            return Err("Compilation failed".to_string());
        }
        
        fs::remove_file(&c_file).ok();
        
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

fn main() {
    println!("cargo:rerun-if-changed=src/ast.rs");
    println!("cargo:rerun-if-changed=src/ir/mod.rs");
    println!("cargo:rerun-if-changed=build.rs");
}

pub fn compile(module: &Module, output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
    CodeGenerator::compile(module, output_path, opt_level)
}