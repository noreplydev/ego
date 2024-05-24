pub enum ErrorType {
    SyntaxError,
    EgoUsageError,
    FatalError,
    ParsingError,
    InterpretingError,
    ReferenceError,
    StackUnderflowError,
    UnknownArithmeticOperator,
    TypeError,
    InvalidTypeAnnotation,
}

pub fn throw(error_type: ErrorType, error_message: &str, line: Option<usize>) {
    let error_string = match error_type {
        ErrorType::SyntaxError => "Syntax error:",
        ErrorType::EgoUsageError => "Usage error:",
        ErrorType::FatalError => "Fatal error:",
        ErrorType::ParsingError => "Parsing error:",
        ErrorType::InterpretingError => "Interpreting error:",
        ErrorType::ReferenceError => "Reference error:",
        ErrorType::StackUnderflowError => "Stack underflow error:",
        ErrorType::UnknownArithmeticOperator => "Unknown arithmetic operator error:",
        ErrorType::TypeError => "Type Error: ",
        ErrorType::InvalidTypeAnnotation => "Invalid type annotation: ",
    };

    println!("\n[ego] {error_string} {error_message}\n");
    if let Some(line) = line {
        println!("      └ on line: {line}");
    }
    std::process::exit(1);
}
