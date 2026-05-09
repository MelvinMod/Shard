use crate::ir::Module;
use crate::ast::Type;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct NativeBackend {
    output: String,
    opt_level: u8,
}

impl NativeBackend {
    pub fn new(opt_level: u8) -> Self {
        Self {
            output: String::new(),
            opt_level,
        }
    }

    pub fn compile(module: &Module, output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
        let mut backend = Self::new(opt_level);
        backend.generate(module);
        
        let c_code = backend.get_output();
        let c_path = output_path.with_extension("c");
        fs::write(&c_path, c_code)
            .map_err(|e| format!("Failed to write C file: {}", e))?;
        
        Self::compile_to_native(&c_path, output_path, opt_level)?;
        fs::remove_file(&c_path).ok();
        
        Ok(())
    }

    fn generate(&mut self, module: &Module) {
        self.emit("// Shard - Native Backend");
        self.emit("// Author: MelvinSGjr (MelvinMod)");
        self.emit("");
        
        self.emit("#include <stdio.h>");
        self.emit("#include <stdlib.h>");
        self.emit("#include <string.h>");
        self.emit("#include <stdbool.h>");
        self.emit("#include <stdint.h>");
        self.emit("#include <math.h>");
        self.emit("");
        
        self.emit("// Core types");
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
        
        for func in &module.functions {
            if func.name != "main" {
                self.generate_function(func);
            } else {
                self.generate_user_main(func);
            }
        }
        
        let has_main = module.functions.iter().any(|f| f.name == "main");
        if has_main {
            self.generate_c_main_wrapper();
        }
    }

    fn map_type(&self, type_: &Type) -> String {
        match type_ {
            Type::Int | Type::Int64 => "Int64".to_string(),
            Type::Int8 => "Int8".to_string(),
            Type::Int16 => "Int16".to_string(),
            Type::Int32 => "Int32".to_string(),
            Type::UInt | Type::UInt64 => "UInt64".to_string(),
            Type::UInt8 => "UInt8".to_string(),
            Type::UInt16 => "UInt16".to_string(),
            Type::UInt32 => "UInt32".to_string(),
            Type::Float | Type::Float64 => "Float64".to_string(),
            Type::Float32 => "Float32".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "String".to_string(),
            Type::Void => "void".to_string(),
            _ => "void*".to_string(),
        }
    }

    fn generate_function(&mut self, func: &crate::ir::Function) {
        let ret_type = self.map_type(&func.return_type.clone().unwrap_or(Type::Void));
        
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

    fn generate_user_main(&mut self, func: &crate::ir::Function) {
        self.emit("void shard_main() {");
        
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

    fn generate_instruction(&mut self, instr: &crate::ir::IRInstruction) {
        match instr {
            crate::ir::IRInstruction::Nop => {}
            crate::ir::IRInstruction::Label(_) => {}
            crate::ir::IRInstruction::Alloca(name, type_) => {
                self.emit(&format!("    {} {} = 0;", self.map_type(type_), name));
            }
            crate::ir::IRInstruction::Store(val, name) => {
                self.emit(&format!("    {} = {};", name, val));
            }
            crate::ir::IRInstruction::Call(result, func_name, args) => {
                if func_name == "say" || func_name == "print" {
                    let arg_str = args.get(0).map(|s| format!("\"%s\", {}", s)).unwrap_or_default();
                    self.emit(&format!("    printf({});", arg_str));
                    self.emit("    printf(\"\\n\");");
                } else {
                    let args_str = args.join(", ");
                    self.emit(&format!("    {} = {}({});", result, func_name, args_str));
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
            _ => {}
        }
    }

    fn generate_c_main_wrapper(&mut self) {
        self.emit("int main(int argc, char** argv) {");
        self.emit("    (void)argc;");
        self.emit("    (void)argv;");
        self.emit("");
        self.emit("    shard_main();");
        self.emit("");
        self.emit("    return 0;");
        self.emit("}");
    }

    fn emit(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn get_output(&self) -> String {
        self.output.clone()
    }

    fn compile_to_native(c_path: &PathBuf, output_path: &PathBuf, opt_level: u8) -> Result<(), String> {
        let optimize_flag = match opt_level {
            0 => "-O0",
            1 => "-O1",
            2 => "-O2",
            3 => "-O3",
            _ => "-O2",
        };

        let platform = if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "freebsd") {
            "freebsd"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "unknown"
        };

        let compiler = if platform == "windows" { "gcc" } else { "cc" };

        let status = Command::new(compiler)
            .arg(c_path)
            .arg("-o")
            .arg(output_path)
            .arg(optimize_flag)
            .arg("-Wall")
            .arg("-lm")
            .status()
            .map_err(|e| format!("Compilation failed: {}", e))?;

        if !status.success() {
            return Err("Compilation failed".to_string());
        }

        println!("  [+] Native compilation successful: {}", output_path.display());
        Ok(())
    }
}
