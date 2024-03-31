pub enum ErrorType {
    SyntaxError,
    EgoUsageError,
    FatalError,
    ParsingError,
    ReferenceError,
    StackUnderflowError,
    ExpressionError,
    UnknownArithmeticOperator,
}

pub fn throw(error_type: ErrorType, error_message: &str, line: Option<usize>) {
    let mut error_string = "";
    match error_type {
        ErrorType::SyntaxError => error_string = "Syntax error:",
        ErrorType::EgoUsageError => error_string = "Usage error:",
        ErrorType::FatalError => error_string = "Fatal error:",
        ErrorType::ParsingError => error_string = "Parsing error:",
        ErrorType::ReferenceError => error_string = "Reference error:",
        ErrorType::StackUnderflowError => error_string = "Stack underflow error:",
        ErrorType::ExpressionError => error_string = "Expression error:",
        ErrorType::UnknownArithmeticOperator => error_string = "Unknown arithmetic operator error:",
    }

    println!("[ego] {error_string} {error_message}");
    if let Some(line) = line {
        println!("      └ on line: {line}");
    }
    std::process::exit(1);
}
