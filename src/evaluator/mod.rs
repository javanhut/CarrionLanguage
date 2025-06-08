use crate::ast::Operator;
use crate::ast::{Expression, Program, Statement};
use crate::object::{Function, Object};

pub fn eval(program: &Program) -> Result<Object, String> {
    eval_program(program)
}

fn eval_program(program: &Program) -> Result<Object, String> {
    // This variable will now hold the result of the last evaluated statement.
    let mut result = Object::None;

    for statement in &program.statements {
        let value = eval_statement(statement)?;

        // If a statement returns a `ReturnValue`, we must stop execution
        // and return the unwrapped value immediately.
        if let Object::ReturnValue(return_val) = value {
            return Ok(*return_val);
        }

        // --- THIS IS THE FIX ---
        // Update the result with the value of the statement we just evaluated.
        result = value;
    }

    Ok(result)
}

// The rest of the file remains the same...

fn eval_statement(statement: &Statement) -> Result<Object, String> {
    match statement {
        // A statement that is just an expression produces a value.
        Statement::Expression(expr_stmt) => eval_expression(expr_stmt),
        Statement::Return(ret_stmt) => {
            let value = match &ret_stmt.value {
                Some(expr) => eval_expression(expr)?,
                None => Object::None,
            };
            Ok(Object::ReturnValue(Box::new(value)))
        }
        // TODO: Add cases for other statement types: Assignment, If, For, etc.
        _ => Err(format!(
            "Evaluation for this statement type is not yet implemented: {:?}",
            statement
        )),
    }
}

fn eval_expression(expression: &Expression) -> Result<Object, String> {
    match expression {
        Expression::IntegerLiteral(val) => Ok(Object::Integer(*val)),
        Expression::FloatLiteral(val) => Ok(Object::Float(*val)),
        Expression::BooleanLiteral(val) => Ok(Object::Boolean(*val)),
        Expression::StringLiteral(val) => Ok(Object::String(val.clone())),

        Expression::Infix(infix_expr) => {
            let left = eval_expression(&infix_expr.left)?;
            let right = eval_expression(&infix_expr.right)?;
            eval_infix_expression(&infix_expr.operator, left, right)
        }

        Expression::Prefix(prefix_expr) => {
            let right = eval_expression(&prefix_expr.right)?;
            eval_prefix_expression(&prefix_expr.operator, right)
        }

        _ => Err(format!(
            "Evaluation for this expression type is not yet implemented: {:?}",
            expression
        )),
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
