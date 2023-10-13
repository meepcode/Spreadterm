// use std::fmt::format;

use crate::{lexer::{Token, TokenType}, model::{Evaluatable, Primitive, Operation, Statistics, CellAddress, CellValue}};

pub fn parse(tokens: Vec<Token>) -> Result<Box<dyn Evaluatable>, String> {
    Parser::new(tokens).parse()
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
}

impl Parser<'_> {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens, index: 0 }
    }

    fn parse(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        self.expression()
    }

    fn has(&self, token_type: TokenType) -> bool {
        self.index < self.tokens.len() && self.tokens.get(self.index).unwrap().token_type == token_type
    }

    fn cur_token_index(&self) -> usize {
        if self.index > self.tokens.len() {
            self.tokens[self.index].start
        } else {
            self.tokens[self.tokens.len() - 1].start
        }
    }

    fn capture(&mut self) -> Token {
        let token = self.tokens.get(self.index).unwrap();
        self.index += 1;
        return token.clone();
    }

    fn expression(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.logical_and();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::LogicalOr) {
            self.capture();
            let right_result = self.logical_and();
            let right: Box<dyn Evaluatable>;

            match right_result {
                Ok(val) => right = val,
                Err(val) => return Err(val), 
            }
            left = Box::new(Operation::LogicalOr(left, right));
        }

        Ok(left)
    }
    
    fn logical_and(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.bitwise_or();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::LogicalAnd) {
            self.capture();
            let right_result = self.bitwise_or();
            let right: Box<dyn Evaluatable>;

            match right_result {
                Ok(val) => right = val,
                Err(val) => return Err(val), 
            }
            left = Box::new(Operation::LogicalAnd(left, right));
        }

        Ok(left)

    }

    fn bitwise_or(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.bitwise_xor();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::BitwiseOr) {
            self.capture();
            let right_result = self.bitwise_xor();
            let right: Box<dyn Evaluatable>;

            match right_result {
                Ok(val) => right = val,
                Err(val) => return Err(val), 
            }

            left = Box::new(Operation::BitwiseOr(left, right));
        }

        Ok(left)

    }

    fn bitwise_xor(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.bitwise_and();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::BitwiseXor) {
            self.capture();
            let right_result = self.bitwise_and();
            let right: Box<dyn Evaluatable>;

            match right_result {
                Ok(val) => right = val,
                Err(val) => return Err(val), 
            }
            left = Box::new(Operation::BitwiseXor(left, right));
        }

        Ok(left)


    }

    fn bitwise_and(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.equality();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::BitwiseAnd) {
            self.capture();
            let right_result = self.equality();
            let right: Box<dyn Evaluatable>;

            match right_result {
                Ok(val) => right = val,
                Err(val) => return Err(val), 
            }
            left = Box::new(Operation::BitwiseAnd(left, right));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.comparison();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }
        while self.has(TokenType::DoubleEquals) || self.has(TokenType::NotEquals) {
            if self.has(TokenType::DoubleEquals) {
                self.capture();
                let right_result = self.comparison();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::Equals(left, right));
            } else if self.has(TokenType::NotEquals) {
                self.capture();
                let right_result = self.comparison();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::NotEquals(left, right));
            }
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.shift();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }
        while self.has(TokenType::LessThan) || self.has(TokenType::LessThanOrEqual) || self.has(TokenType::GreaterThan) || self.has(TokenType::GreaterThanOrEqual) {
            if self.has(TokenType::LessThan) {
                self.capture();
                let right_result = self.shift();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::LessThan(left, right));
            } else if self.has(TokenType::LessThanOrEqual) {
                self.capture();
                let right_result = self.shift();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::LessThanOrEqual(left, right));
            } else if self.has(TokenType::GreaterThan) {
                self.capture();
                let right_result = self.shift();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::GreaterThan(left, right));
            } else if self.has(TokenType::GreaterThanOrEqual) {
                self.capture();
                let right_result = self.shift();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::GreaterThanOrEqual(left, right));
            }
        }

        Ok(left)
    }

    fn shift(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.additive();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }
        while self.has(TokenType::LeftShift) || self.has(TokenType::RightShift) {
            if self.has(TokenType::LeftShift) {
                self.capture();
                let right_result = self.additive();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::LeftShift(left, right));
            } else if self.has(TokenType::RightShift) {
                self.capture();
                let right_result = self.additive();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::RightShift(left, right));
            }
        }

        Ok(left)
    }

    fn additive(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.multiplicative();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::Plus) || self.has(TokenType::Minus) {
            if self.has(TokenType::Plus) {
                self.capture();
                let right_result = self.multiplicative();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::Add(left, right));
            } else if self.has(TokenType::Minus) {
                self.capture();
                let right_result = self.multiplicative();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::Subtract(left, right));
            }
        }

        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.power();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::Multiply) || self.has(TokenType::Divide) || self.has(TokenType::Modulus) {
            if self.has(TokenType::Multiply) {
                self.capture();
                let right_result = self.power();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::Multiply(left, right));
            } else if self.has(TokenType::Divide) {
                self.capture();
                let right_result = self.power();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::Divide(left, right));
            } else if self.has(TokenType::Modulus) {
                self.capture();
                let right_result = self.power();
                let right: Box<dyn Evaluatable>;

                match right_result {
                    Ok(val) => right = val,
                    Err(val) => return Err(val), 
                }
                left = Box::new(Operation::Modulus(left, right));
            }
        }

        Ok(left)
    }
    
    fn power(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        let left_result = self.unary();
        let mut left: Box<dyn Evaluatable>;

        match left_result {
            Ok(val) => left = val,
            Err(val) => return Err(val), 
        }

        while self.has(TokenType::Power) {
            self.capture();
            let right_result = self.unary();
            let right: Box<dyn Evaluatable>;

            match right_result {
                Ok(val) => right = val,
                Err(val) => return Err(val), 
            }
            left = Box::new(Operation::Power(left, right));
        }

        Ok(left)
    }
    
    fn unary(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        if self.has(TokenType::LogicalNot) {
            self.capture();
            match self.unary() {
                Ok(val) => Ok(Box::new(Operation::LogicalNot(val))),
                Err(err) => Err(err),
            }
        } else if self.has(TokenType::BitwiseNot) {
            self.capture();
            match self.unary() {
                Ok(val) => Ok(Box::new(Operation::BitwiseNot(val))),
                Err(err) => Err(err),
            }
        } else if self.has(TokenType::IntegerCast) {
            self.capture();
            match self.unary() {
                Ok(val) => Ok(Box::new(Operation::FloatToInt(val))),
                Err(err) => Err(err),
            }
        } else if self.has(TokenType::FloatCast) {
            self.capture();
            match self.unary() {
                Ok(val) => Ok(Box::new(Operation::IntToFloat(val))),
                Err(err) => Err(err),
            }
        } else {
            self.statistics()
        }
    }

    fn statistics(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        if self.has(TokenType::Max) || self.has(TokenType::Mean) || self.has(TokenType::Min) || self.has(TokenType::Sum) {
            let function = self.tokens[self.index].token_type;
            let left: CellAddress;
            let right: CellAddress;
            self.capture();
            if self.has(TokenType::OpenParenthesis) {
                self.capture();
                let left_cell = self.cell_address();
                match left_cell {
                    Ok(val) => left = val,
                    Err(val) => return Err(val),
                }

                if self.has(TokenType::Comma) {
                    self.capture();
                    let right_cell = self.cell_address();
                    match right_cell {
                        Ok(val) => right = val,
                        Err(val) => return Err(val)
                    }

                    if self.has(TokenType::CloseParenthesis) {
                        self.capture();
                    } else {
                        return Err(format!("Missing Closing Parenthesis at {}", self.index))
                    }
                } else {
                    return Err(format!("Missing Comma at {}", self.index))
                }
            } else {
                return Err(format!("Missing Open Parenthesis at {}", self.index))
            }

            return match function {
                TokenType::Max => Ok(Box::new(Statistics::Max(left, right))),
                TokenType::Mean => Ok(Box::new(Statistics::Mean(left, right))),
                TokenType::Min => Ok(Box::new(Statistics::Min(left, right))),
                TokenType::Sum => Ok(Box::new(Statistics::Sum(left, right))),
                _ => Err(format!("Incorrect function type"))
            };
        }

        self.atom()
    }

    fn atom(&mut self) -> Result<Box<dyn Evaluatable>, String> {
        if self.has(TokenType::OpenParenthesis) {
            self.capture();
            let result = self.expression();
            let result_unwrapped: Box<dyn Evaluatable>;
            match result {
                Ok(val) => result_unwrapped = val,
                Err(val) => return Err(val)
            }
            if self.has(TokenType::CloseParenthesis) {
                self.capture();
                Ok(result_unwrapped)
            } else {
                Err(format!("Missing Closing Parenthesis at {}", self.index))
            }
        } else if self.has(TokenType::IntegerLiteral) {
            let token = self.capture();
            Ok(Box::new(Primitive::Integer(token.text.parse().unwrap())))
        } else if self.has(TokenType::FloatLiteral) {
            let token = self.capture();
            Ok(Box::new(Primitive::Float(token.text.to_string().parse().unwrap())))
        } else if self.has(TokenType::StringLiteral) {
            let token = self.capture();
            Ok(Box::new(Primitive::String(token.text.to_string())))
        } else if self.has(TokenType::False) {
            self.capture();
            Ok(Box::new(Primitive::Boolean(false)))
        } else if self.has(TokenType::True) {
            self.capture();
            Ok(Box::new(Primitive::Boolean(true)))
        } else if self.has(TokenType::OpenBracket) {
            match self.cell_address() {
                Ok(val) => Ok(Box::new(CellValue(val.0, val.1))),
                Err(val) => Err(val)
            }
        } else {
            if self.index < self.tokens.len() {
                Err(format!("Unexpected Token {} at index {}", self.tokens.get(self.index).unwrap().text, self.cur_token_index()))
            } else {
                Err(format!("Incomplete Input String (Likely a dropped primitive after an operator)"))
            }
        }
    }

    fn cell_address(&mut self) -> Result<CellAddress, String> {
        let left: i32;
        let right: i32;
        if self.has(TokenType::OpenBracket) {
            self.capture();
            if self.has(TokenType::IntegerLiteral) {
                left = self.capture().text.parse().unwrap();
                if self.has(TokenType::Comma) {
                    self.capture();
                    if self.has(TokenType::IntegerLiteral) {
                        right = self.capture().text.parse().unwrap();
                        if self.has(TokenType::CloseBracket) {
                            self.capture();
                            Ok(CellAddress(left, right))
                        } else {
                            Err(format!("Missing Close Bracket for Cell Address at index {}", self.cur_token_index()))
                        }
                    } else {
                        Err(format!("Missing Second Integer in Cell Address at index {}", self.cur_token_index()))
                    }
                } else {
                    Err(format!("Missing Comma in Cell Address at index {}", self.cur_token_index()))
                }
            } else {
                Err(format!("Missing First Integer in Cell Address at index {}", self.cur_token_index()))
            }
        } else {
            Err(format!("Missing open bracket for cell address at index {}", self.cur_token_index()))
        }
    }
}
