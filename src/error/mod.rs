pub fn error(line_number: u32, message: &[String]) {
    let _ = custom_report(line_number, "", message);
}

fn custom_report(line_number: u32, where_err: &[String], message: &[String]) {
    eprintln!("[line: {line_number}] Error: {where_err}: {message}");
}
