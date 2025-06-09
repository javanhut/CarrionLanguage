use crate::object::Object;

pub fn builtin_print(args: Vec<Object>) -> Result<Object, String> {
    let parts: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    println!("{}", parts.join(" "));
    Ok(Object::None)
}

pub fn builtin_length(args: Vec<Object>) -> Result<Object, String> {
    // 1. Check for the correct number of arguments.
    if args.len() != 1 {
        return Err(format!(
            "Wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(s) => {
            let length = s.len() as i64;
            Ok(Object::Integer(length))
        }
        Object::List(items) => {
            let length = items.len() as i64;
            Ok(Object::Integer(length))
        }
        Object::Dict(map) => {
            let length = map.len() as i64;
            Ok(Object::Integer(length))
        }
        other => Err(format!("Object of type {} has no length.", other)),
    }
}

pub fn builtin_push(args: Vec<Object>) -> Result<Object, String> {
    if args.len() != 2 {
        return Err(format!(
            "Wrong number of arguments. got={}, want=2",
            args.len()
        ));
    }

    match &args[0] {
        Object::List(items) => {
            let mut new_items = items.clone();
            new_items.push(args[1].clone());
            Ok(Object::List(new_items))
        }
        other => Err(format!("Cannot push to {}", other)),
    }
}

pub fn builtin_pop(args: Vec<Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "Wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::List(items) => {
            if items.is_empty() {
                Err("Cannot pop from empty list".to_string())
            } else {
                let mut new_items = items.clone();
                new_items.pop();
                Ok(Object::List(new_items))
            }
        }
        other => Err(format!("Cannot pop from {}", other)),
    }
}

pub fn builtin_keys(args: Vec<Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "Wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::Dict(map) => {
            let keys: Vec<Object> = map.keys()
                .map(|k| Object::String(k.clone()))
                .collect();
            Ok(Object::List(keys))
        }
        other => Err(format!("Cannot get keys from {}", other)),
    }
}

pub fn builtin_values(args: Vec<Object>) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!(
            "Wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match &args[0] {
        Object::Dict(map) => {
            let values: Vec<Object> = map.values().cloned().collect();
            Ok(Object::List(values))
        }
        other => Err(format!("Cannot get values from {}", other)),
    }
}
