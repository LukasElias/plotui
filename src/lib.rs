use std::collections::HashMap;

struct MathParser {
    functions: HashMap<char, Function>,
    variables: HashMap<char, f32>,
}

struct Function {
    param: char,
    body: CalculatableExpr,
}

enum ExprType {
    FunctionDefinition(char), // The function definition is saved under the MathParser struct in a Hashmap
    Equation {
        left: CalculatableExpr,
        op: ComparisonOp, // <, >, <=, >=, =, !=
        right: CalculatableExpr,
    },
    CalculatableExpression(CalculatableExpr),
    VariableAssignment {
        name: String,
        value: CalculatableExpr,
    },
    Unknown,
}

struct CalculatableExpr(Vec<SimpleToken>);

// Only for tokenizing the strings by the user
#[derive(Debug, PartialEq)]
enum ComplexToken {
    Number(f32),
    Operator(Operator),
    ComparisonOp(ComparisonOp),
    Identifier(char),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Unknown(char),
}

// For tokens that are simple enough to be in an expression that can get calculated
enum SimpleToken {
    Number(f32),
    Operator(Operator),
    Variable(char),
    Constant(char),
    Function(String),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Unknown(char),
}

#[derive(Debug, PartialEq)]
enum ComparisonOp {
    LessThan,             // <
    GreaterThan,          // >
    LessThanOrEqualTo,    // <=
    GreaterThanOrEqualTo, // >=
    EqualTo,              // =
    NotEqualTo,           // !=
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

enum ShuntingYardOperator {
    Operator(Operator),
    Function(String),
    LeftParanthesis,
}

/*

Rules for when an expression is a function

1. It has an single character next to a parathesis with a variable inside e.g. f(x)
2. There is either a y and x in the expr, or there's a function definition as defined in rule 1
3. There is a LHS and RHS, e.g. there's an equal sign =


Steps for parsing a math expression

1 Get the string, for example "g(y)*4=2y-4/10"
1.5 Tokenize the input to a Vec<ComplexToken>
2 Figure out what type of expression this is, e.g. function, equation with unknowns, or maybe a simple equation with a single constant output etc... In this example it would be a function called g, with a variable y
3 Then simplifying till it's something that can be either plotted or calculated, which would be g(y)=(2y-4/10)/4
4 Optional, but simplifying that to the simplest form which would be g(y)=y/2-0.1
5 Then using picking the right implementation for this type of expression which is known since the 2nd step. Fx plotting a function

*/


impl MathParser {
    pub fn tokenize_input(input_str: &str) -> Vec<ComplexToken> {

        let mut expr: Vec<ComplexToken> = Vec::new();

        let mut past_numbers = String::new();

        let mut chars = input_str.chars().peekable();

        while let Some(char) = chars.next() {
            if char.is_ascii_whitespace() {
                continue;
            }

            println!("{}", char);

            if char.is_ascii_digit() || char == '.' {
                past_numbers.push(char);
                continue;
            }

            if !past_numbers.is_empty() {
                expr.push(ComplexToken::Number(past_numbers.parse::<f32>().unwrap()));
                past_numbers.clear();
            }

            if "<>!".contains(char) {
                if let Some('=') = chars.peek() {
                    chars.next().unwrap();
                    if let Some(op) = ComparisonOp::from_char(char, true) {
                        expr.push(ComplexToken::ComparisonOp(op));
                        continue;
                    }
                } else {
                    if let Some(op) = ComparisonOp::from_char(char, false) {
                        expr.push(ComplexToken::ComparisonOp(op));
                        continue;
                    }
                }
            } else if char == '=' {
                expr.push(ComplexToken::ComparisonOp(ComparisonOp::EqualTo));
                continue;
            }

            if let Some(op) = Operator::from_char(char) {
                expr.push(ComplexToken::Operator(op));
                continue;
            }

            if char.is_alphabetic() {
                expr.push(ComplexToken::Identifier(char));

                continue;
            }

            expr.push(match char {
                '(' => ComplexToken::LeftParenthesis,
                ')' => ComplexToken::RightParenthesis,
                ',' => ComplexToken::Comma,
                idk_what_it_is => ComplexToken::Unknown(idk_what_it_is),
            });
        }

        if !past_numbers.is_empty() {
            expr.push(ComplexToken::Number(past_numbers.parse::<f32>().unwrap()));
            past_numbers.clear();
        }

        expr
    }
}

impl ComparisonOp {
    fn from_char(char: char, is_equal: bool) -> Option<Self> {
        match (char, is_equal) {
            ('<', true) => Some(Self::LessThanOrEqualTo),
            ('>', true) => Some(Self::GreaterThanOrEqualTo),
            ('!', true) => Some(Self::NotEqualTo),
            ('<', false) => Some(Self::LessThan),
            ('>', false) => Some(Self::GreaterThan),
            ('=', false) => Some(Self::EqualTo),
            _ => None,
        }
    }
}

impl Operator {
    fn from_char(char: char) -> Option<Self> {
        match char {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '^' => Some(Self::Pow),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MathParser;

    #[test]
    fn test_mathparser() {
        let string = "y=sin(x)";

        let result = MathParser::tokenize_input(string);

        println!("{:?}", result);
    }
}
