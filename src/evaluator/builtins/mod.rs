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
        other => Err(format!("Object of type {} has no length.", other)),
    }
}
