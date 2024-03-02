pub enum ErrorType {
    SyntaxError,
    CeiUsageError,
    FatalError,
    ReferenceError,
    StackUnderflowError,
    ExpressionError,
}

pub fn throw(error_type: ErrorType, error_message: &str, line: Option<usize>) {
    let mut error_string = "";
    match error_type {
        ErrorType::SyntaxError => error_string = "Syntax error:",
        ErrorType::CeiUsageError => error_string = "Usage error:",
        ErrorType::FatalError => error_string = "Fatal error:",
        ErrorType::ReferenceError => error_string = "Reference error:",
        ErrorType::StackUnderflowError => error_string = "Stack underflow error:",
        ErrorType::ExpressionError => error_string = "Expression error:",
        _ => {}
    }

    println!("[cei] {error_string} {error_message}");
    if let Some(line) = line {
        println!("      └ on line: {line}");
    }
    std::process::exit(1);
}
