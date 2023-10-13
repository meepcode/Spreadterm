#[derive(Debug,Eq, PartialEq, Clone, Copy)]
pub enum TokenType {
    BitwiseAnd,
    BitwiseNot,
    BitwiseOr,
    BitwiseXor,
    CloseBracket,
    CloseCurlyBracket,
    CloseParenthesis,
    Comma,
    Divide,
    DoubleEquals,
    Power,
    False,
    FloatCast,
    FloatLiteral,
    GreaterThan,
    GreaterThanOrEqual,
    IntegerCast,
    IntegerLiteral,
    LeftShift,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalNot,
    LogicalOr,
    Max,
    Mean,
    Min,
    Minus,
    Modulus,
    NotEquals,
    OpenBracket,
    OpenCurlyBracket,
    OpenParenthesis,
    Plus,
    RightShift,
    StringLiteral,
    Sum,
    Multiply,
    True,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

pub fn lex<'a>(text: &'a str) -> Result<Vec<Token>, String> {
    Lexer::new(text).lex()
}

#[derive(Debug)]
struct Lexer<'a> {
    text: &'a str,
    cur_index: usize,
    start_index: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(text: &'a str) -> Lexer<'a> {
        Lexer { text: text, cur_index: 0, start_index: 0, tokens: Vec::new() }
    }

    fn lex(mut self) -> Result<Vec<Token<'a>>, String> {
        while !self.is_at_end() {
            if self.has("|") {
                self.capture();
                if self.has("|") {
                    self.capture();
                    self.emit_token(TokenType::LogicalOr);
                } else {
                    self.emit_token(TokenType::BitwiseOr);
                }
            } else if self.has("&") {
                self.capture();
                if self.has("&") {
                    self.capture();
                    self.emit_token(TokenType::LogicalAnd);
                } else {
                    self.emit_token(TokenType::BitwiseAnd);
                }
            } else if self.has("<") {
                self.capture();
                if self.has("<") {
                    self.capture();
                    self.emit_token(TokenType::LeftShift);
                } else if self.has("=") {
                    self.capture();
                    self.emit_token(TokenType::LessThanOrEqual);
                } else {
                    self.emit_token(TokenType::LessThan);
                }
            } else if self.has(">") {
                self.capture();
                if self.has(">") {
                    self.capture();
                    self.emit_token(TokenType::RightShift);
                } else if self.has("=") {
                    self.capture();
                    self.emit_token(TokenType::GreaterThanOrEqual);
                } else {
                    self.emit_token(TokenType::GreaterThan);
                } 
            } else if self.has("^") {
                self.capture();
                self.emit_token(TokenType::BitwiseXor);
            } else if self.has(",") {
                self.capture();
                self.emit_token(TokenType::Comma);
            } else if self.has(")") {
                self.capture();
                self.emit_token(TokenType::CloseParenthesis);
            } else if self.has("}") {
                self.capture();
                self.emit_token(TokenType::CloseCurlyBracket);
            } else if self.has("]") {
                self.capture();
                self.emit_token(TokenType::CloseBracket);
            } else if self.has("(") {
                self.capture();
                self.emit_token(TokenType::OpenParenthesis);
            } else if self.has("{") {
                self.capture();
                self.emit_token(TokenType::OpenCurlyBracket);
            } else if self.has("[") {
                self.capture();
                self.emit_token(TokenType::OpenBracket);
            }else if self.has("float") {
                self.capture();
                self.capture();
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::FloatCast);
            } else if self.has("int") {
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::IntegerCast);
            } else if self.has("max") {
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::Max);
            } else if self.has("mean") {
                self.capture();
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::Mean);
            } else if self.has("min") {
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::Min);
            } else if self.has("sum") {
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::Sum);
            } else if self.has("-") {
                self.capture();
                if self.has_digit() {
                    self.capture();
                    while self.has_digit() {
                        self.capture();
                    }
                    if self.has(".") {
                        self.capture();
                        while self.has_digit() {
                            self.capture();
                        }
                        self.emit_token(TokenType::FloatLiteral);
                    } else {
                        self.emit_token(TokenType::IntegerLiteral);
                    }
                } else {
                    self.emit_token(TokenType::Minus);
                }
            } else if self.has("+") {
                self.capture();
                self.emit_token(TokenType::Plus);
            } else if self.has("*") {
                self.capture();
                if self.has("*") {
                    self.capture();
                    self.emit_token(TokenType::Power);
                } else {
                    self.emit_token(TokenType::Multiply);
                }
            } else if self.has("/") {
                self.capture();
                self.emit_token(TokenType::Divide);
            } else if self.has("%") {
                self.capture();
                self.emit_token(TokenType::Modulus);
            } else if self.has("==") {
                self.capture();
                self.capture();
                self.emit_token(TokenType::DoubleEquals);
            } else if self.has("!") {
                self.capture();
                if self.has("=") {
                    self.capture();
                    self.emit_token(TokenType::NotEquals);
                } else {
                    self.emit_token(TokenType::LogicalNot);
                }
            } else if self.has("~") {
                self.capture();
                self.emit_token(TokenType::BitwiseNot);
            } else if self.has("\"") { // String literals
                self.abandon();
                while !self.has("\"") || self.is_at_end() {
                    if self.has("\\\\") {
                        self.capture();
                        self.capture();
                    } else if self.has("\\\"") {
                        self.capture();
                        self.capture();
                    } else if self.is_at_end() {
                        return Err(format!("Unclosed string literal starting at index {}: \"{}", self.start_index - 1, &self.text[self.start_index..self.cur_index]));
                    }else {
                        self.capture();
                    }
                }
                if !self.is_at_end() {
                    self.emit_token(TokenType::StringLiteral);
                    self.abandon(); // Skip next "
                }
            } else if self.has("true") { // Booleans
                self.capture();
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::True);
            } else if self.has("false") {
                self.capture();
                self.capture();
                self.capture();
                self.capture();
                self.capture();
                self.emit_token(TokenType::False);
            } else if self.has_digit() {
                self.capture();
                while self.has_digit() {
                    self.capture();
                }
                if self.has(".") {
                    self.capture();
                    while self.has_digit() {
                        self.capture();
                    }
                    self.emit_token(TokenType::FloatLiteral);
                } else {
                    self.emit_token(TokenType::IntegerLiteral);
                }
            } else if self.has_whitespace() {
                self.abandon();
            } else {
                return Err(format!("Unexpected character '{}' at index {}", &self.text[self.start_index..self.start_index + 1], self.start_index));
            }
        }

        if self.cur_index != self.start_index {
            Err(format!("An unexpected string \"{}\" at index {}", &self.text[self.start_index..self.text.len()], self.start_index))
        } else {
            Ok(self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.cur_index >= self.text.len()
    }

    fn has(&self, str: &str) -> bool {
        self.cur_index + str.len() - 1 < self.text.len() && &self.text[self.cur_index..self.cur_index + str.len()] == str 
    } 

    fn has_whitespace(&self) -> bool {
        !self.is_at_end() && self.text.chars().nth(self.cur_index).unwrap().is_whitespace()
    }

    fn has_digit(&self) -> bool {
        !self.is_at_end() && self.text.chars().nth(self.cur_index).unwrap().is_digit(10)
    }

    fn capture(&mut self) {
        self.cur_index += 1;
    }

    fn abandon(&mut self) {
        self.cur_index += 1;
        self.start_index = self.cur_index;
    }

    fn emit_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type: token_type,
            text: &self.text[self.start_index..self.cur_index],
            start: self.start_index,
            end: self.cur_index - 1,
        });
        
        self.start_index = self.cur_index;
    }
}
