extern crate limonite;

use std::vec::IntoIter;

use limonite::syntax::lexer::Tokenizer;
use limonite::syntax::parser::Parser;
use limonite::syntax::core::tokens::Token;
use limonite::syntax::core::tokens::Token::*;
use limonite::syntax::core::types::Types;
use limonite::syntax::core::keywords::Keywords;
use limonite::syntax::core::symbols::Symbols;
use limonite::syntax::ast::expr::{Expr, ExprWrapper};
use limonite::syntax::ast::consts::*;
use limonite::syntax::ast::op::*;

struct MockLexer {
    tokens: IntoIter<Token>
}

impl MockLexer {
    fn new(v: Vec<Token>) -> MockLexer {
        MockLexer {
            tokens: v.into_iter()
        }
    }
}

impl Tokenizer for MockLexer {
    fn get_tok(&mut self) -> Token {
        let next = self.tokens.next();
        match next {
            Some(tok) => tok,
            None => EOF,
        }
    }

    fn get_error_pos(&self) -> (usize, usize, usize, usize) {
        (1, 1, 1, 1)
    }
}

#[test]
fn test_print() {
    let lexer = MockLexer::new(vec![
        Keyword(Keywords::Print),
        Symbol(Symbols::ParenOpen),
        Identifier("meow".to_string()),
        Symbol(Symbols::Comma),
        Identifier("meow".to_string()),
        Symbol(Symbols::Comma),
        Identifier("meow".to_string()),
        Symbol(Symbols::ParenClose),
    ]);
    let mut parser = Parser::new(lexer);
    parser.parse();
    let ast = parser.get_ast().get_expr();
    println!("{:?}", ast);

    assert!(ast != ast);
}

#[test]
fn test_variable_int_declaration() {
    // let mLexer = MockLexer::new(vec![
        // Keyword(Keywords::Var),
        // Identifier("meow".to_string()),
        // Symbol(Symbols::Equals),
    // ]);
    // let mut parser = Parser::new(mLexer);

    // parser.parse();
}

#[test]
fn test_valid_fn_declaration() {
    // No args: foo() -> u64
    let lexer = MockLexer::new(vec![Keyword(Keywords::Fn),
                                    Identifier(String::from_str("foo")),
                                    Symbol(Symbols::ParenOpen),
                                    Symbol(Symbols::ParenClose),
                                    Symbol(Symbols::RightThinArrow),
                                    Type(Types::UInt64Bit),
                                    Indent(1),
                                    EOF]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::FnDecl(String::from_str("foo"), Vec::new(), Type(Types::UInt64Bit),
                      ExprWrapper::default(Expr::Block(Vec::new()))))]));

    if *ast != desired_ast {
        panic!("No argument test failed");
    }

    // One arg: foo(bar: i32) -> str
    let args = vec![(String::from_str("bar"), Type(Types::Int32Bit))];

    let lexer = MockLexer::new(vec![Keyword(Keywords::Fn),
                                    Identifier(String::from_str("foo")),
                                    Symbol(Symbols::ParenOpen),
                                    Identifier(String::from_str("bar")),
                                    Symbol(Symbols::Colon),
                                    Type(Types::Int32Bit),
                                    Symbol(Symbols::ParenClose),
                                    Symbol(Symbols::RightThinArrow),
                                    Type(Types::Str),
                                    Indent(1),
                                    EOF]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::FnDecl(String::from_str("foo"), args, Type(Types::Str),
                      ExprWrapper::default(Expr::Block(Vec::new()))))]));

    if *ast != desired_ast {
        panic!("One argument test failed");
    }

    // Multiple args: foo(bar: i32, left: Obj, right: Obj) -> None
    let args = vec![(String::from_str("bar"), Type(Types::Int32Bit)),
                    (String::from_str("left"), Identifier(String::from_str("Obj"))),
                    (String::from_str("right"), Identifier(String::from_str("Obj")))];

    let lexer = MockLexer::new(vec![Keyword(Keywords::Fn),
                                    Identifier(String::from_str("foo")),
                                    Symbol(Symbols::ParenOpen),
                                    Identifier(String::from_str("bar")),
                                    Symbol(Symbols::Colon),
                                    Type(Types::Int32Bit),
                                    Symbol(Symbols::Comma),
                                    Identifier(String::from_str("left")),
                                    Symbol(Symbols::Colon),
                                    Identifier(String::from_str("Obj")),
                                    Symbol(Symbols::Comma),
                                    Identifier(String::from_str("right")),
                                    Symbol(Symbols::Colon),
                                    Identifier(String::from_str("Obj")),
                                    Symbol(Symbols::ParenClose),
                                    Symbol(Symbols::RightThinArrow),
                                    Type(Types::NoneType),
                                    Indent(1),
                                    EOF]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::FnDecl(String::from_str("foo"), args, Type(Types::NoneType),
                      ExprWrapper::default(Expr::Block(Vec::new()))))]));

    if *ast != desired_ast {
        panic!("Multiple argument test failed");
    }

}

#[test]
fn test_indentation_levels() {
    // Correct indentation level +1 after fn decl
    let lexer = MockLexer::new(vec![Keyword(Keywords::Fn),
                                    Identifier(String::from_str("foo")),
                                    Symbol(Symbols::ParenOpen),
                                    Symbol(Symbols::ParenClose),
                                    Symbol(Symbols::RightThinArrow),
                                    Type(Types::UInt64Bit),
                                    Indent(1)]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    panic!("Asd");
}

#[test]
fn test_expression() {
    // if foo + bar equals "foobar",
    let lexer = MockLexer::new(vec![Keyword(Keywords::If),
                                    Identifier("foo".to_string()),
                                    Symbol(Symbols::Plus),
                                    Identifier("bar".to_string()),
                                    Keyword(Keywords::Equals),
                                    StrLiteral("foobar".to_string()),
                                    Symbol(Symbols::Comma),
                                    Indent(1)]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let condition = ExprWrapper::default(Expr::InfixOp(InfixOp::Equ,
                    ExprWrapper::default(Expr::InfixOp(InfixOp::Add,
                    ExprWrapper::default(Expr::Ident("foo".to_string())),
                    ExprWrapper::default(Expr::Ident("bar".to_string())))),
                    ExprWrapper::default(Expr::Const(Const::UTF8String("foobar".to_string())))));

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::If(condition, ExprWrapper::default(Expr::Block(vec![])),
                      None))]));

    assert!(*ast == desired_ast);
}

#[test]
fn test_expression_precedence_add_mult() {
    // Make sure a + b * c + d generates a + (b * c) + d
    let lexer = MockLexer::new(vec![Keyword(Keywords::If),
                                    Identifier("a".to_string()),
                                    Symbol(Symbols::Plus),
                                    Identifier("b".to_string()),
                                    Symbol(Symbols::Asterisk),
                                    Identifier("c".to_string()),
                                    Symbol(Symbols::Plus),
                                    Identifier("d".to_string()),
                                    Symbol(Symbols::Comma),
                                    Indent(1)]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let mult = ExprWrapper::default(Expr::InfixOp(InfixOp::Mul,
               ExprWrapper::default(Expr::Ident("b".to_string())),
               ExprWrapper::default(Expr::Ident("c".to_string()))));

    let left_add = ExprWrapper::default(Expr::InfixOp(InfixOp::Add,
                   ExprWrapper::default(Expr::Ident("a".to_string())), mult));

    let condition = ExprWrapper::default(Expr::InfixOp(InfixOp::Add, left_add,
                    ExprWrapper::default(Expr::Ident("d".to_string()))));

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::If(condition, ExprWrapper::default(Expr::Block(vec![])),
                      None))]));


    if *ast != desired_ast {
        println!("Desired ast: {:?}", desired_ast);
        println!("Actual ast: {:?}", ast);
        panic!("Addition and multiplication precedence check failed");
    }
}

#[test]
fn test_expression_precedence_pow() {
    // Make sure a ^ b ^ c generates a ^ (b ^ c) which is right associative
    let lexer = MockLexer::new(vec![Keyword(Keywords::If),
                                    Identifier("a".to_string()),
                                    Symbol(Symbols::Caret),
                                    Identifier("b".to_string()),
                                    Symbol(Symbols::Caret),
                                    Identifier("c".to_string()),
                                    Symbol(Symbols::Comma),
                                    Indent(1)]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let right_pow = ExprWrapper::default(Expr::InfixOp(InfixOp::Pow,
                    ExprWrapper::default(Expr::Ident("b".to_string())),
                    ExprWrapper::default(Expr::Ident("c".to_string()))));

    let condition = ExprWrapper::default(Expr::InfixOp(InfixOp::Pow,
                    ExprWrapper::default(Expr::Ident("a".to_string())), right_pow));

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::If(condition, ExprWrapper::default(Expr::Block(vec![])),
                      None))]));

    if *ast != desired_ast {
        println!("Desired ast: {:?}", desired_ast);
        println!("Actual ast: {:?}", ast);
        panic!("Power precedence check failed");
    }
}

#[test]
fn test_numerics() {
    // Test default type assignment (42.0 should default to f32)
    let lexer = MockLexer::new(vec![Keyword(Keywords::If),
                                    Numeric("42.0".to_string(), None),
                                    Symbol(Symbols::Comma),
                                    Indent(1)]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let condition = ExprWrapper::default(Expr::Const(Const::F32Num(42f32)));

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::If(condition, ExprWrapper::default(Expr::Block(vec![])),
                      None))]));

    if *ast != desired_ast {
        println!("Desired ast: {:?}", desired_ast);
        println!("Actual ast: {:?}", ast);
        panic!("Numeric default type test failed");
    }

    // Test explicit type assignment via suffix (42u32 should be u32 as designated)
    let lexer = MockLexer::new(vec![Keyword(Keywords::If),
                                    Numeric("42".to_string(), Some(Types::UInt32Bit)),
                                    Symbol(Symbols::Comma),
                                    Indent(1)]);

    let mut parser = Parser::new(lexer);
    parser.parse();

    let ast = parser.get_ast();

    let condition = ExprWrapper::default(Expr::Const(Const::U32Num(42u32)));

    let desired_ast = ExprWrapper::default(Expr::Block(vec![ExprWrapper::default(
                      Expr::If(condition, ExprWrapper::default(Expr::Block(vec![])),
                      None))]));

    if *ast != desired_ast {
        println!("Desired ast: {:?}", desired_ast);
        println!("Actual ast: {:?}", ast);
        panic!("Numeric explicit type test failed");
    }

}
