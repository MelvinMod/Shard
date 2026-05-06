use shard::lexer;
use shard::parser;
use shard::typechecker;
use shard::ir;

#[test]
fn test_lex_int_literal() {
    let tokens = lexer::lex("42").unwrap();
    assert_eq!(tokens.len(), 1);
}

#[test]
fn test_lex_string_literal() {
    let tokens = lexer::lex("\"hello\"").unwrap();
    assert_eq!(tokens.len(), 1);
}

#[test]
fn test_lex_function() {
    let tokens = lexer::lex("fn main() { }").unwrap();
    assert!(tokens.len() > 1);
}

#[test]
fn test_parse_variable() {
    let tokens = lexer::lex("let x: Int = 10;").unwrap();
    let ast = parser::parse(tokens).unwrap();
    assert_eq!(ast.nodes.len(), 1);
}

#[test]
fn test_parse_function() {
    let tokens = lexer::lex("fn add(a: Int, b: Int) -> Int { return a + b; }").unwrap();
    let ast = parser::parse(tokens).unwrap();
    assert_eq!(ast.nodes.len(), 1);
}

#[test]
fn test_typecheck_valid() {
    let tokens = lexer::lex("fn main() { let x: Int = 10; }").unwrap();
    let ast = parser::parse(tokens).unwrap();
    assert!(typechecker::check(&ast).is_ok());
}

#[test]
fn test_ir_generation() {
    let tokens = lexer::lex("fn main() { let x: Int = 10; }").unwrap();
    let ast = parser::parse(tokens).unwrap();
    let module = ir::generate(&ast).unwrap();
    assert!(!module.functions.is_empty());
}

#[test]
fn test_compile_hello() {
    let source = r#"
        fn main() {
            print("Hello, World!");
        }
    "#;
    let tokens = lexer::lex(source).unwrap();
    let ast = parser::parse(tokens).unwrap();
    assert!(typechecker::check(&ast).is_ok());
    let module = ir::generate(&ast).unwrap();
    assert!(!module.functions.is_empty());
}
