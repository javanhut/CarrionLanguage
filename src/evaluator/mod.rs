pub mod builtins;
pub mod environment;

use crate::ast::{Expression, Identifier, Operator, Program, Statement};
use crate::object::{Builtin, BuiltinFunction, Function, Object};
use environment::Environment;

pub fn eval(program: &Program) -> Result<Object, String> {
    let mut env = Environment::new();
    eval_program(program, &mut env)
}

fn eval_program(program: &Program, env: &mut Environment) -> Result<Object, String> {
    let mut result = Object::None;
    for statement in &program.statements {
        let value = eval_statement(statement, env)?;

        if let Object::ReturnValue(return_val) = value {
            return Ok(*return_val);
        }
        result = value;
    }
    Ok(result)
}

fn eval_statement(statement: &Statement, env: &mut Environment) -> Result<Object, String> {
    match statement {
        Statement::Expression(expr_stmt) => eval_expression(expr_stmt, env),
        Statement::Return(ret_stmt) => {
            let value = match &ret_stmt.value {
                Some(expr) => eval_expression(expr, env)?,
                None => Object::None,
            };
            Ok(Object::ReturnValue(Box::new(value)))
        }
        _ => Err(format!(
            "Evaluation for this statement type is not yet implemented: {:?}",
            statement
        )),
    }
}

fn eval_expression(expression: &Expression, env: &mut Environment) -> Result<Object, String> {
    match expression {
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::IntegerLiteral(val) => Ok(Object::Integer(*val)),
        Expression::FloatLiteral(val) => Ok(Object::Float(*val)),
        Expression::BooleanLiteral(val) => Ok(Object::Boolean(*val)),
        Expression::StringLiteral(val) => Ok(Object::String(val.clone())),

        Expression::Prefix(prefix_expr) => {
            let right = eval_expression(&prefix_expr.right, env)?;
            eval_prefix_expression(&prefix_expr.operator, right)
        }
        Expression::Infix(infix_expr) => {
            let left = eval_expression(&infix_expr.left, env)?;
            let right = eval_expression(&infix_expr.right, env)?;
            eval_infix_expression(&infix_expr.operator, left, right)
        }
        Expression::Call(call_expr) => {
            let function_obj = eval_expression(&call_expr.function, env)?;
            let mut args = Vec::new();
            for arg_expr in &call_expr.arguments {
                args.push(eval_expression(arg_expr, env)?);
            }
            apply_function(function_obj, args)
        }
        _ => Err(format!(
            "Evaluation for this expression type is not yet implemented: {:?}",
            expression
        )),
    }
}

fn apply_function(func: Object, args: Vec<Object>) -> Result<Object, String> {
    match func {
        Object::Builtin(builtin) => (builtin.func)(args),
        Object::Function(_user_func) => {
            Err("User-defined function calls not yet implemented.".to_string())
        }
        _ => Err(format!("Not a function: {}", func)),
    }
}

fn eval_identifier(ident: &Identifier, env: &Environment) -> Result<Object, String> {
    if let Some(val) = env.get(&ident.0) {
        Ok(val.clone())
    } else {
        Err(format!("Identifier not found: {}", ident.0))
    }
}

fn eval_prefix_expression(operator: &Operator, right: Object) -> Result<Object, String> {
    match operator {
        Operator::Not => Ok(Object::Boolean(!is_truthy(right))),
        Operator::Minus => {
            if let Object::Integer(val) = right {
                Ok(Object::Integer(-val))
            } else if let Object::Float(val) = right {
                Ok(Object::Float(-val))
            } else {
                Err(format!("Unknown operator: -{}", right))
            }
        }
        _ => Err(format!("Unknown prefix operator: {:?}", operator)),
    }
}

fn eval_infix_expression(
    operator: &Operator,
    left: Object,
    right: Object,
) -> Result<Object, String> {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix_operator(operator, *l, *r),
        (Object::Float(l), Object::Float(r)) => eval_float_infix_operator(operator, *l, *r),
        (Object::String(l), Object::String(r)) => {
            if *operator == Operator::Plus {
                Ok(Object::String(format!("{}{}", l, r)))
            } else {
                Err(format!("Unknown operator for Strings: {:?}", operator))
            }
        }
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            Operator::Equal => Ok(Object::Boolean(l == r)),
            Operator::NotEqual => Ok(Object::Boolean(l != r)),
            _ => Err(format!("Unknown operator for Booleans: {:?}", operator)),
        },
        _ => Err(format!(
            "Type mismatch: cannot apply operator {:?} to {} and {}",
            operator, left, right
        )),
    }
}

fn eval_integer_infix_operator(
    operator: &Operator,
    left: i64,
    right: i64,
) -> Result<Object, String> {
    match operator {
        Operator::Plus => Ok(Object::Integer(left + right)),
        Operator::Minus => Ok(Object::Integer(left - right)),
        Operator::Multiply => Ok(Object::Integer(left * right)),
        Operator::Divide => Ok(Object::Integer(left / right)),
        Operator::Equal => Ok(Object::Boolean(left == right)),
        Operator::NotEqual => Ok(Object::Boolean(left != right)),
        Operator::LessThan => Ok(Object::Boolean(left < right)),
        Operator::GreaterThan => Ok(Object::Boolean(left > right)),
        _ => Err(format!("Unknown operator for Integers: {:?}", operator)),
    }
}

fn eval_float_infix_operator(operator: &Operator, left: f64, right: f64) -> Result<Object, String> {
    match operator {
        Operator::Plus => Ok(Object::Float(left + right)),
        Operator::Minus => Ok(Object::Float(left - right)),
        Operator::Multiply => Ok(Object::Float(left * right)),
        Operator::Divide => Ok(Object::Float(left / right)),
        Operator::Equal => Ok(Object::Boolean(left == right)),
        Operator::NotEqual => Ok(Object::Boolean(left != right)),
        Operator::LessThan => Ok(Object::Boolean(left < right)),
        Operator::GreaterThan => Ok(Object::Boolean(left > right)),
        _ => Err(format!("Unknown operator for Floats: {:?}", operator)),
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Boolean(b) => b,
        Object::None => false,
        _ => true,
    }
}
