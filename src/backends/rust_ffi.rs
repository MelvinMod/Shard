use crate::ir::{Module, Function, BasicBlock, IRInstruction};
use crate::ast::Type;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct RustFFIBackend {
    output: String,
    opt_level: u8,
}

impl RustFFIBackend {
    pub fn new(opt_level: u8) -> Self {
        Self {
            output: String::new(),
            opt_level,
        }
    }

    pub fn compile(module: &Module, output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
        let mut backend = Self::new(opt_level);
        backend.generate(module)?;
        
        let rust_code = backend.get_output();
        let rs_path = output_path.with_extension("rs");
        fs::write(&rs_path, rust_code)
            .map_err(|e| format!("Failed to write Rust file: {}", e))?;
        
        println!("  [+] Rust FFI output generated: {}", rs_path.display());
        println!("  Note: To compile, run: rustc {} -o {}", rs_path.display(), output_path.display());
        
        Ok(())
    }

    fn generate(&mut self, module: &Module) -> Result<(), String> {
        self.emit("// Shard - Rust FFI Backend");
        self.emit("// Author: MelvinSGjr (MelvinMod)");
        self.emit("");
        
        self.emit("#[allow(unused_imports)]");
        self.emit("use std::io;");
        self.emit("#[allow(unused_imports)]");
        self.emit("use std::process::Command;");
        self.emit("#[allow(unused_imports)]");
        self.emit("use std::fs;");
        self.emit("");
        
        for func in &module.functions {
            self.generate_function(func);
        }
        
        self.generate_main();
        
        Ok(())
    }

    fn map_type(&self, type_: &Type) -> String {
        match type_ {
            Type::Int | Type::Int64 => "i64".to_string(),
            Type::Int8 => "i8".to_string(),
            Type::Int16 => "i16".to_string(),
            Type::Int32 => "i32".to_string(),
            Type::UInt | Type::UInt64 => "u64".to_string(),
            Type::UInt8 => "u8".to_string(),
            Type::UInt16 => "u16".to_string(),
            Type::UInt32 => "u32".to_string(),
            Type::Float | Type::Float64 => "f64".to_string(),
            Type::Float32 => "f32".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "String".to_string(),
            Type::Void => "()".to_string(),
            _ => "()".to_string(),
        }
    }

    fn generate_function(&mut self, func: &Function) {
        let ret_type = self.map_type(&func.return_type.clone().unwrap_or(Type::Void));
        
        self.emit("#[allow(unused_variables)]");
        self.emit(&format!("fn {}(", func.name));
        
        let mut first = true;
        for (name, type_) in &func.params {
            if !first {
                self.emit(", ");
            }
            first = false;
            self.emit(&format!("{}: {}", name, self.map_type(type_)));
        }
        self.emit(&format!(") -> {} {{", ret_type));
        
        for block in &func.blocks {
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

    fn generate_instruction(&mut self, instr: &IRInstruction) {
        match instr {
            IRInstruction::Nop => {
                self.emit("    // nop");
            }
            IRInstruction::Alloca(name, type_) => {
                self.emit(&format!("    let mut {} : {} = 0;", name, self.map_type(type_)));
            }
            IRInstruction::Store(val, name) => {
                self.emit(&format!("    {} = {};", name, val));
            }
            _ => {
                self.emit(&format!("    // {:?}", instr));
            }
        }
    }

    fn generate_terminator(&mut self, term: &IRInstruction) {
        match term {
            IRInstruction::Return(val) => {
                if let Some(v) = val {
                    self.emit(&format!("    return {};", v));
                } else {
                    self.emit("    return;");
                }
            }
            _ => {
                self.emit(&format!("    // {:?}", term));
            }
        }
    }

    fn generate_main(&mut self) {
        self.emit("#[allow(dead_code)]");
        self.emit("fn main() {");
        self.emit("    // Main entry point");
        self.emit("}");
    }

    fn emit(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn get_output(&self) -> String {
        self.output.clone()
    }
}
