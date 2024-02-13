pub enum ErrorType {
    SyntaxError,
    CeiUsageError,
    FatalError,
    ReferenceError,
    StackUnderflowError,
}

pub fn throw(error_type: ErrorType, error_message: &str) {
    let mut error_string = "";
    match error_type {
        ErrorType::SyntaxError => error_string = "Syntax error:",
        ErrorType::CeiUsageError => error_string = "Cei usage error:",
        ErrorType::FatalError => error_string = "Fatal error:",
        ErrorType::ReferenceError => error_string = "Reference error:",
        ErrorType::StackUnderflowError => error_string = "Stack underflow error:",
        _ => {}
    }

    println!("[cei] {error_string} {error_message}");
    std::process::exit(1);
}
