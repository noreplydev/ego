use std::fmt;

#[derive(Clone, Debug)]
pub enum LexerTokenType {
    LetKeyword,
    ImportKeyword,
    FnKeyword,
    WhileKeyword,
    IfKeyword,
    ElseKeyword,
    TrueKeyword,
    FalseKeyword,
    ReturnKeyword,
    BreakKeyword,
    NothingKeyword,
    StringKeyword,
    NumberKeyword,
    BoolKeyword,
    Identifier,
    AssignmentOperator,
    AddOperator,
    SubtractOperator,
    MultiplyOperator,
    DivideOperator,
    OrOperator,
    AmpersandOperator,
    LessThanOperator,
    LessThanOrEqualOperator,
    GreaterThanOperator,
    GreaterThanOrEqualOperator,
    EqualityOperator,
    NotEqualOperator,
    NotOperator,
    StringLiteral,
    Number,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenSquareBracket,
    CloseSquareBracket,
    Comma,
    Dot,
    Colon,
    EndOfStatement,
    Unknown,
}

impl fmt::Display for LexerTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerTokenType::LetKeyword => write!(f, "LetKeyword"),
            LexerTokenType::ImportKeyword => write!(f, "ImportKeyword"),
            LexerTokenType::FnKeyword => write!(f, "FnKeyword"),
            LexerTokenType::WhileKeyword => write!(f, "WhileKeyword"),
            LexerTokenType::IfKeyword => write!(f, "IfKeyword"),
            LexerTokenType::ElseKeyword => write!(f, "ElseKeyword"),
            LexerTokenType::TrueKeyword => write!(f, "TrueKeyword"),
            LexerTokenType::FalseKeyword => write!(f, "FalseKeyword"),
            LexerTokenType::ReturnKeyword => write!(f, "ReturnKeyword"),
            LexerTokenType::BreakKeyword => write!(f, "BreakKeyword"),
            LexerTokenType::NothingKeyword => write!(f, "NothingKeyword"),
            LexerTokenType::StringKeyword => write!(f, "StringKeyword"),
            LexerTokenType::NumberKeyword => write!(f, "NumberKeyword"),
            LexerTokenType::BoolKeyword => write!(f, "BoolKeyword"),
            LexerTokenType::Identifier => write!(f, "Identifier"),
            LexerTokenType::AssignmentOperator => write!(f, "AssignmentOperator"),
            LexerTokenType::EqualityOperator => write!(f, "EqualityOperator"),
            LexerTokenType::AddOperator => write!(f, "AddOperator"),
            LexerTokenType::SubtractOperator => write!(f, "SubtractOperator"),
            LexerTokenType::MultiplyOperator => write!(f, "MultiplyOperator"),
            LexerTokenType::DivideOperator => write!(f, "DivideOperator"),
            LexerTokenType::OrOperator => write!(f, "OrOperator"),
            LexerTokenType::AmpersandOperator => write!(f, "AmpersandOperator"),
            LexerTokenType::LessThanOperator => write!(f, "LessThanOperator"),
            LexerTokenType::LessThanOrEqualOperator => write!(f, "LessThanOrEqualOperator"),
            LexerTokenType::GreaterThanOperator => write!(f, "GreaterThanOperator"),
            LexerTokenType::GreaterThanOrEqualOperator => write!(f, "GreaterThanOrEqualOperator"),
            LexerTokenType::NotEqualOperator => write!(f, "NotEqualOperator"),
            LexerTokenType::NotOperator => write!(f, "NotOperator"),
            LexerTokenType::StringLiteral => write!(f, "StringLiteral"),
            LexerTokenType::Number => write!(f, "Number"),
            LexerTokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            LexerTokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            LexerTokenType::OpenCurlyBrace => write!(f, "OpenCurlyBrace"),
            LexerTokenType::CloseCurlyBrace => write!(f, "CloseCurlyBrace"),
            LexerTokenType::OpenSquareBracket => write!(f, "OpenSquareBracket"),
            LexerTokenType::CloseSquareBracket => write!(f, "CloseSquareBracket"),
            LexerTokenType::Dot => write!(f, "Dot"),
            LexerTokenType::Colon => write!(f, "Colon"),
            LexerTokenType::Comma => write!(f, "Comma"),
            LexerTokenType::EndOfStatement => write!(f, "EndOfStatement"),
            LexerTokenType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl PartialEq for LexerTokenType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Clone)]
pub struct LexerToken {
    pub token_type: LexerTokenType,
    pub value: String,
    pub line: usize,
    pub at: usize,
}

impl LexerToken {
    pub fn new(token_type: LexerTokenType, value: String, line: usize, at: usize) -> LexerToken {
        LexerToken {
            token_type,
            value,
            line,
            at,
        }
    }
}

impl fmt::Display for LexerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (line: {}, char: {})",
            self.token_type, self.value, self.line, self.at
        )
    }
}
