pub fn error(line_number: u32, message: &str) {
    let _ = custom_report(line_number, "", message);
}

fn custom_report(line_number: u32, where_err: &str, message: &str) {
    eprintln!("[line: {line_number}] Error: {where_err}: {message}");
}
