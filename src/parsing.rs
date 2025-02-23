use dioxus::prelude::*;

pub enum ParsingError {
    IncorrectNumber,
    IncorrectOperator,
    ZeroDivision,
    IncorrectInput,
}

fn calculate(input: Signal<String>) -> Result<f64, ParsingError> {
    let input = input.read();
    let evaluation = input.split_whitespace();
    let (numbers, operators): (Vec<&str>, Vec<&str>) =
        evaluation.partition(|element| match element.trim().parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        });

    if numbers.len() != operators.len() + 1 || operators.is_empty() || numbers.is_empty() {
        return Err(ParsingError::IncorrectInput);
    }

    let mut result = match numbers[0].parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err(ParsingError::IncorrectNumber),
    };

    for index in 0..operators.len() {
        let operator = operators[index];
        let number = match numbers[index + 1].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err(ParsingError::IncorrectNumber),
        };

        match operator {
            "+" => result += number,
            "-" => result -= number,
            "*" => result *= number,
            "/" => {
                if number == 0 as f64 {
                    return Err(ParsingError::ZeroDivision);
                }
                result /= number
            }
            _ => return Err(ParsingError::IncorrectOperator),
        }
    }

    Ok(result)
}

pub fn compare_input(input: Signal<String>) -> String {
    let result = calculate(input);
    match result {
        Ok(num) => num.to_string(),
        Err(ParsingError::IncorrectNumber) => "Incorrect number".to_string(),
        Err(ParsingError::IncorrectOperator) => "Incorrect operator".to_string(),
        Err(ParsingError::IncorrectInput) => "Incorrect input".to_string(),
        Err(ParsingError::ZeroDivision) => "Cannot divise by zero".to_string(),
    }
}
