use crate::object::Object;

pub fn builtin_print(args: Vec<Object>) -> Result<Object, String> {
    let parts: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    println!("{}", parts.join(" "));
    Ok(Object::None)
}
