#![allow(unused_variables)]
use lexical::types::Types;
use lexical::types::Types::Bool;
use syntax::expr::ExprWrapper;
use syntax::expr::Expr::*;
use semantic::analyzer_trait::ASTAnalyzer;

pub struct TypeChecker;

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker
    }

    fn cmp_lhs_rhs(lhs_str: String, rhs_str: String) -> Option<String> {
        match (lhs_str.parse::<Types>(), rhs_str.parse::<Types>()) {
            (Ok(lhs_type), Ok(rhs_type)) => {
                if lhs_type != rhs_type {
                    panic!("Error goes here"); // FIXME: Better errors
                }

                Some(lhs_str)
            },
            (Err(()), Err(())) => {
                // TODO: Custom type comparison. May or may not be an error.
                panic!("Found a custom type references");
            },
            // Found a builtin type and a custom type -> error
            _ => panic!("Error goes here") // FIXME: Better errors
        }
    }
}

impl ASTAnalyzer<Option<String>> for TypeChecker {
    fn analyze(&mut self, ast_root: &mut ExprWrapper) -> Option<String> {
        // TODO: Handle errors, Option<ExprWrapper> like elsewhere for now?
        // How to get Expr's type?
        // Basics:
        // Literal -> Builtin Type (Done) or Custom Type
        // Variable -> VarName -> Lookup VarDecl -> Type
        // FnCall -> FnName -> LookUp FnDecl -> Type
        // InfixOp -> analyze bubble up type from Expr op Expr
        // UnaryOp -> analyze bubble up type from Expr

        // Future?:
        // if rust style let i = if a {} else {}
        // If needs lookup all returns in all possible code blocks, assert they're the same type

        match *ast_root.get_mut_expr() {
            Block(ref mut vec) => {
                for expr_wrapper in vec {
                    println!("Looping over expr {:?}!", expr_wrapper);
                    self.analyze(expr_wrapper);
                }

                None // FIXME?
            },
            FnDecl(ref mut name, ref mut args, ref mut ret_type, ref mut body_expr_wrapper) => {
                // TODO: Check if args are of a valid type

                if ret_type != &self.analyze(body_expr_wrapper) {
                    panic!("Error goes here") // FIXME: Better errors
                }

                ret_type.clone() // Better way than to clone?
            },
            InfixOp(ref mut op, ref mut lhs_expr_wrapper, ref mut rhs_expr_wrapper) => {
                let lhs_type = match self.analyze(lhs_expr_wrapper) {
                    Some(t) => t,
                    None => unreachable!("This should not happen??") // VERIFY, unwrap?
                };
                let rhs_type = match self.analyze(rhs_expr_wrapper) {
                    Some(t) => t,
                    None => unreachable!("This should not happen??") // VERIFY, unwrap?
                };

                TypeChecker::cmp_lhs_rhs(lhs_type, rhs_type)
            },
            VarDecl(ref mut const_, ref mut name, ref mut opt_type, ref mut expr_wrapper) => {
                let rhs_type = match self.analyze(expr_wrapper) {
                    Some(t) => t,
                    None => unreachable!("This should not happen??") // VERIFY, unwrap?
                };

                println!("Var decl rhs type: {:?}", rhs_type);

                match *opt_type {
                    Some(ref lhs_type) => TypeChecker::cmp_lhs_rhs(lhs_type.clone(), rhs_type),  // No way to not clone?
                    None => {
                        *opt_type = Some(rhs_type); // Done?

                        None
                    }
                    // REVIEW: The above was *opt_type = Some("i32".parse().unwrap())
                    // Codegen doesn't seem to care what the actual type is.
                    // Does that matter if type checker does its job correctly?
                }
            },
            WhileLoop(ref mut cond_expr_wrapper, ref mut body_expr_wrapper) => {
                match self.analyze(cond_expr_wrapper) {
                    Some(ref string) => match string.parse::<Types>() {
                        Ok(t) => {
                            if t != Bool {
                                panic!("Error goes here, not a bool") // FIXME: Better errors
                            }
                        },
                        Err(()) => unreachable!("Should not happen") // VERIFY, unwrap?
                    },
                    None => unreachable!("Should not happen") // VERIFY, unwrap?
                }

                // Type of a while loop is the type it returns
                self.analyze(body_expr_wrapper)
            },
            FnCall(ref name, ref args) => None, // FIXME: Compare to fn declaration?
            Literal(ref literal) => Some(literal.to_string()), // Done?
            Return(ref mut opt_ret_type) => match *opt_ret_type { // Done?
                Some(ref mut expr_wrapper) => self.analyze(expr_wrapper),
                None => Some("None".into()) // None type?
            },
            NoOp => None, // Done?
            ref node => panic!("Unimplemented node: {:?}", node)
        }
    }
}
