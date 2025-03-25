fn check_brackets(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => { if stack.pop() != Some('(') { return false; } }
            ']' => { if stack.pop() != Some('[') { return false; } }
            '}' => { if stack.pop() != Some('{') { return false; } }
            _ => {}
        }
    }
    stack.is_empty()
}

fn clean_phone_number(phone_number: &str) -> String {
    let mut result = String::new();
    for c in phone_number.chars() {
        if c.is_ascii_digit() {
            result.push(c);
        }
    }
    if result.starts_with("3") {
        result = result[1..].to_string();
    }
    result
}

fn main() {
    println!("Task 1:");

    let test_str1 = "([]{})[]";
    let test_str2 = "([]]";
    println!("Test 1: {}", check_brackets(test_str1));
    println!("Test 2: {}", check_brackets(test_str2));

    println!("\nTask 2");

    let phone_number1 = "+3 (050)-995-0253";
    let phone_number2 = "050-995-0253";
    let phone_number3 = "3 050 995 0253";
    let phone_number4 = "050.995.0253";
    println!("Cleaned phone number 1: {}", clean_phone_number(phone_number1));
    println!("Cleaned phone number 2: {}",clean_phone_number(phone_number2));
    println!("Cleaned phone number 3: {}", clean_phone_number(phone_number3));
    println!("Cleaned phone number 4: {}", clean_phone_number(phone_number4));
}
