#[derive(Debug, PartialEq)]
enum Expr {
    Number(f64),
    Operation(Box<Expr>, Operator, Box<Expr>),
    Function(Function, Box<Expr>),
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiate,
}

#[derive(Debug, PartialEq)]
enum Function {
    Sin,
    Cos,
    Log,
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Sin,
    Cos,
    Log,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) || ch == '.' {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let number: f64 = number.parse().unwrap();
                tokens.push(Token::Number(number));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Power);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            _ if c.is_whitespace() => {
                chars.next();
            }
            _ => {
                // Handle unexpected character
                // For simplicity, we just skip it here
                chars.next();
            }
        }
    }

    tokens
}

fn parse(tokens: &[Token]) -> Result<Expr, &'static str> {
    let mut tokens = tokens.iter().peekable();
    parse_expression(&mut tokens)
}

fn parse_expression(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expr, &'static str> {
    let mut expr = parse_term(tokens)?;

    while let Some(token) = tokens.peek() {
        let op = match token {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Subtract,
            _ => break,
        };

        tokens.next(); // Consume the operator
        let right = parse_term(tokens)?;
        expr = Expr::Operation(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn parse_term(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expr, &'static str> {
    let mut expr = parse_factor(tokens)?;

    while let Some(token) = tokens.peek() {
        let op = match token {
            Token::Multiply => Operator::Multiply,
            Token::Divide => Operator::Divide,
            _ => break,
        };

        tokens.next(); // Consume the operator
        let right = parse_factor(tokens)?;
        expr = Expr::Operation(Box::new(expr), op, Box::new(right));
    }

    Ok(expr)
}

fn parse_factor(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expr, &'static str> {
    let mut expr = parse_power(tokens)?;

    // Handle factorial, etc., here if needed

    Ok(expr)
}

fn parse_power(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expr, &'static str> {
    let mut expr = parse_unary(tokens)?;

    while let Some(&Token::Power) = tokens.peek() {
        tokens.next(); // Consume the power operator
        let right = parse_unary(tokens)?;
        expr = Expr::Operation(Box::new(expr), Operator::Exponentiate, Box::new(right));
    }

    Ok(expr)
}

fn parse_unary(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expr, &'static str> {
    if let Some(&Token::Minus) = tokens.peek() {
        tokens.next(); // Consume the minus
        let expr = parse_primary(tokens)?;
        Ok(Expr::Operation(
            Box::new(Expr::Number(0.0)),
            Operator::Subtract,
            Box::new(expr),
        ))
    } else if let Some(&Token::Plus) = tokens.peek() {
        tokens.next(); // Consume the plus
        parse_primary(tokens)
    } else {
        parse_primary(tokens)
    }
}

fn parse_primary(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Expr, &'static str> {
    if let Some(token) = tokens.next() {
        match token {
            Token::Number(n) => Ok(Expr::Number(*n)),
            Token::LParen => {
                let expr = parse_expression(tokens)?;
                if matches!(tokens.next(), Some(Token::RParen)) {
                    Ok(expr)
                } else {
                    Err("Expected closing parenthesis")
                }
            }
            Token::Sin => {
                let expr = parse_primary(tokens)?;
                Ok(Expr::Function(Function::Sin, Box::new(expr)))
            }
            Token::Cos => {
                let expr = parse_primary(tokens)?;
                Ok(Expr::Function(Function::Cos, Box::new(expr)))
            }
            Token::Log => {
                let expr = parse_primary(tokens)?;
                Ok(Expr::Function(Function::Log, Box::new(expr)))
            }
            // ... (handle other functions similarly)
            _ => Err("Unexpected token"),
        }
    } else {
        Err("Unexpected end of input")
    }
}

fn evaluate(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Operation(left, op, right) => {
            let left_val = evaluate(left);
            let right_val = evaluate(right);
            match op {
                Operator::Add => left_val + right_val,
                Operator::Subtract => left_val - right_val,
                Operator::Multiply => left_val * right_val,
                Operator::Divide => left_val / right_val,
                Operator::Exponentiate => left_val.powf(right_val),
            }
        }
        Expr::Function(function, argument) => match function {
            Function::Sin => evaluate(argument).sin(),
            Function::Cos => evaluate(argument).cos(),
            Function::Log => evaluate(argument).ln(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let expr = tokenize("3 + 2 * 2");
        let parsed = parse(&expr).unwrap();
        let result = evaluate(&parsed);
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_complex_expression() {
        let expr = tokenize("3 + 2 * (2 + 3) ^ 2");
        let parsed = parse(&expr).unwrap();
        let result = evaluate(&parsed);
        assert_eq!(result, 53.0);
    }

    #[test]
    fn test_functions() {
        let expr = tokenize("sin(0) + cos(0) + log(1)");
        let parsed = parse(&expr).unwrap();
        let result = evaluate(&parsed);
        assert!((result - 2.0).abs() < 1e-10);
    }
}

fn main() {
    let input = "3 + 2 * (2 + 3) ^ 2";
    let tokens = tokenize(input);
    match parse(&tokens) {
        Ok(ast) => {
            let result = evaluate(&ast);
            println!("Result of '{}': {}", input, result);
        }
        Err(e) => println!("Error: {}", e),
    }
}
