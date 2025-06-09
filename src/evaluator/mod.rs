pub mod builtins;
pub mod environment;

use crate::ast::{Expression, Identifier, Operator, Program, Statement, Assignment, CompoundAssignment, IfStatement, WhileStatement, ForStatement, BlockStatement};
use crate::object::{Builtin, BuiltinFunction, Function, Object};
use environment::Environment;

pub fn eval(program: &Program) -> Result<Object, String> {
    let mut env = Environment::new();
    eval_program(program, &mut env)
}

pub fn eval_with_env(program: &Program, env: &mut Environment) -> Result<Object, String> {
    eval_program(program, env)
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
        Statement::Assignment(assignment) => {
            // Evaluate the right-hand side value
            let value = eval_expression(&assignment.value, env)?;
            
            // Handle single assignment
            if assignment.targets.len() == 1 {
                if let Expression::Identifier(ident) = &assignment.targets[0] {
                    env.set(ident.0.clone(), value.clone());
                    Ok(value)
                } else {
                    Err("Assignment target must be an identifier".to_string())
                }
            } else {
                // Handle multiple assignment (unpacking)
                match &value {
                    Object::List(values) => {
                        if values.len() != assignment.targets.len() {
                            return Err(format!(
                                "Assignment count mismatch: {} targets but {} values",
                                assignment.targets.len(),
                                values.len()
                            ));
                        }
                        
                        for (i, target) in assignment.targets.iter().enumerate() {
                            if let Expression::Identifier(ident) = target {
                                env.set(ident.0.clone(), values[i].clone());
                            } else {
                                return Err("Assignment target must be an identifier".to_string());
                            }
                        }
                        Ok(value)
                    }
                    _ => {
                        // If it's not a list, assign the same value to all targets
                        for target in &assignment.targets {
                            if let Expression::Identifier(ident) = target {
                                env.set(ident.0.clone(), value.clone());
                            } else {
                                return Err("Assignment target must be an identifier".to_string());
                            }
                        }
                        Ok(value)
                    }
                }
            }
        }
        Statement::CompoundAssignment(compound_assignment) => {
            // Get the current value of the target
            if let Expression::Identifier(ident) = &compound_assignment.target {
                let current_value = env.get(&ident.0)
                    .ok_or_else(|| format!("Undefined variable: {}", ident.0))?
                    .clone();
                    
                // Evaluate the right-hand side
                let rhs_value = eval_expression(&compound_assignment.value, env)?;
                
                // Perform the compound operation
                let new_value = eval_infix_expression(
                    &compound_assignment.operator,
                    current_value,
                    rhs_value
                )?;
                
                // Set the new value
                env.set(ident.0.clone(), new_value.clone());
                Ok(new_value)
            } else {
                Err("Compound assignment target must be an identifier".to_string())
            }
        }
        Statement::If(if_stmt) => eval_if_statement(if_stmt, env),
        Statement::While(while_stmt) => eval_while_statement(while_stmt, env),
        Statement::For(for_stmt) => eval_for_statement(for_stmt, env),
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
        Expression::List(elements) => {
            let mut list_objects = Vec::new();
            for elem in elements {
                list_objects.push(eval_expression(elem, env)?);
            }
            Ok(Object::List(list_objects))
        }
        Expression::Dict { pairs } => {
            let mut dict_map = std::collections::HashMap::new();
            for (key_expr, value_expr) in pairs {
                let key_obj = eval_expression(key_expr, env)?;
                let key_str = match key_obj {
                    Object::String(s) => s,
                    _ => key_obj.to_string(),
                };
                let value_obj = eval_expression(value_expr, env)?;
                dict_map.insert(key_str, value_obj);
            }
            Ok(Object::Dict(dict_map))
        }
        Expression::Index(index_expr) => {
            let object = eval_expression(&index_expr.object, env)?;
            let index = eval_expression(&index_expr.index, env)?;
            eval_index_expression(object, index)
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
        Operator::LessThanEqual => Ok(Object::Boolean(left <= right)),
        Operator::GreaterThanEqual => Ok(Object::Boolean(left >= right)),
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
        Operator::LessThanEqual => Ok(Object::Boolean(left <= right)),
        Operator::GreaterThanEqual => Ok(Object::Boolean(left >= right)),
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

fn eval_index_expression(object: Object, index: Object) -> Result<Object, String> {
    match (&object, &index) {
        (Object::List(elements), Object::Integer(idx)) => {
            let idx = *idx as usize;
            if idx < elements.len() {
                Ok(elements[idx].clone())
            } else {
                Err(format!("Index out of bounds: {} (list length: {})", idx, elements.len()))
            }
        }
        (Object::Dict(map), key) => {
            let key_str = match key {
                Object::String(s) => s.clone(),
                _ => key.to_string(),
            };
            if let Some(value) = map.get(&key_str) {
                Ok(value.clone())
            } else {
                Ok(Object::None)
            }
        }
        (Object::String(s), Object::Integer(idx)) => {
            let idx = *idx as usize;
            let chars: Vec<char> = s.chars().collect();
            if idx < chars.len() {
                Ok(Object::String(chars[idx].to_string()))
            } else {
                Err(format!("Index out of bounds: {} (string length: {})", idx, chars.len()))
            }
        }
        _ => Err(format!("Index operation not supported for {} with index {}", object, index)),
    }
}

fn eval_if_statement(if_stmt: &IfStatement, env: &mut Environment) -> Result<Object, String> {
    let condition = eval_expression(&if_stmt.condition, env)?;
    
    if is_truthy(condition) {
        eval_block_statement(&if_stmt.consequence, env)
    } else {
        // Check otherwise clauses
        for (alt_condition, alt_consequence) in &if_stmt.alternatives {
            let alt_cond_result = eval_expression(alt_condition, env)?;
            if is_truthy(alt_cond_result) {
                return eval_block_statement(alt_consequence, env);
            }
        }
        
        // Check else clause
        if let Some(default_block) = &if_stmt.default {
            eval_block_statement(default_block, env)
        } else {
            Ok(Object::None)
        }
    }
}

fn eval_while_statement(while_stmt: &WhileStatement, env: &mut Environment) -> Result<Object, String> {
    let mut result = Object::None;
    
    loop {
        let condition = eval_expression(&while_stmt.condition, env)?;
        if !is_truthy(condition) {
            break;
        }
        
        result = eval_block_statement(&while_stmt.body, env)?;
        
        // Handle return values
        if let Object::ReturnValue(_) = result {
            break;
        }
    }
    
    Ok(result)
}

fn eval_for_statement(for_stmt: &ForStatement, env: &mut Environment) -> Result<Object, String> {
    let iterable = eval_expression(&for_stmt.iter, env)?;
    let mut result = Object::None;
    
    match iterable {
        Object::List(elements) => {
            for element in elements {
                env.set(for_stmt.target.0.clone(), element);
                result = eval_block_statement(&for_stmt.body, env)?;
                
                // Handle return values
                if let Object::ReturnValue(_) = result {
                    break;
                }
            }
        }
        Object::String(s) => {
            for ch in s.chars() {
                env.set(for_stmt.target.0.clone(), Object::String(ch.to_string()));
                result = eval_block_statement(&for_stmt.body, env)?;
                
                // Handle return values
                if let Object::ReturnValue(_) = result {
                    break;
                }
            }
        }
        _ => {
            return Err(format!("Object is not iterable: {}", iterable));
        }
    }
    
    Ok(result)
}

fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> Result<Object, String> {
    let mut result = Object::None;
    
    for statement in block {
        result = eval_statement(statement, env)?;
        
        // Handle return values
        if let Object::ReturnValue(_) = result {
            break;
        }
    }
    
    Ok(result)
}
