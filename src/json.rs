use core::panic;

#[derive(Debug)]
pub enum JSONValue {
    Object(Vec<(String, JSONValue)>),
    Array(Vec<JSONValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn parse_from_string(input: String) -> JSONValue {
    let characters = input.chars().collect::<Vec<char>>();

    let mut offset = 0;
    return parse(&characters, &mut offset);
}

fn parse(input: &[char], offset: &mut usize) -> JSONValue {
    skip_spaces(input, offset);
    match input[*offset] {
        '{' => parse_object(input, offset),
        '"' => parse_string(input, offset),
        '+' | '-' | '0'..='9' => parse_number(input, offset),
        '[' => parse_array(input, offset),
        't' | 'f' | 'n' => parse_special(input, offset),
        _ => {
            todo!("unhandled character : {}", input[*offset]);
        }
    }
}

fn parse_object(input: &[char], offset: &mut usize) -> JSONValue {
    // skip the opening brace
    *offset += 1;

    let mut obj: Vec<(String, JSONValue)> = Vec::new();

    // parse the key / value pairs
    loop {
        skip_spaces(input, offset);

        // skip the quote
        if input[*offset] != '\"' {
            panic!("key for json object should be quoted (offset {offset}");
        }
        *offset += 1;

        skip_spaces(input, offset);

        // parse the key
        let mut key = String::new();
        while input[*offset] != '\"' {
            key.push(input[*offset]);
            *offset += 1;
        }
        *offset += 1;

        skip_spaces(input, offset);

        // skip the :
        if input[*offset] != ':' {
            panic!("missing colon separator for key/value pair in object (offset {offset}");
        }
        *offset += 1;

        skip_spaces(input, offset);

        // parse value and add to array
        let value = parse(input, offset);
        obj.push((key, value));

        // quit if there are no more members
        skip_spaces(input, offset);
        if input[*offset] == '}' {
            *offset += 1;
            break;
        }

        if input[*offset] != ',' {
            panic!("object members should be separated by a comma (offset : {offset})");
        }
        *offset += 1;
        skip_spaces(input, offset);
    }

    return JSONValue::Object(obj);
}

fn parse_array(input: &[char], offset: &mut usize) -> JSONValue {
    // skip the opening bracket
    *offset += 1;

    let mut arr: Vec<JSONValue> = Vec::new();

    // parse the values
    loop {
        skip_spaces(input, offset);

        let value = parse(input, offset);
        arr.push(value);

        // quit if there are no more members
        skip_spaces(input, offset);
        if input[*offset] == ']' {
            *offset += 1;
            break;
        }

        if input[*offset] != ',' {
            panic!("object members should be separated by a comma (offset : {offset})");
        }
        *offset += 1;
        skip_spaces(input, offset);
    }

    return JSONValue::Array(arr);
}

fn parse_string(input: &[char], offset: &mut usize) -> JSONValue {
    // skip the opening quote
    *offset += 1;

    // parse the string
    let mut string = String::new();
    while input[*offset] != '"' {
        string.push(input[*offset]);
        *offset += 1;
    }

    // skip the end quote
    *offset += 1;

    return JSONValue::String(string);
}

fn parse_number(input: &[char], offset: &mut usize) -> JSONValue {
    // guess how long the number is
    let start = *offset;
    while match input[*offset] {
        '+' | '-' | '.' | 'E' | 'e' | '0'..='9' => true,
        _ => false,
    } {
        *offset += 1;
    }

    // try to parse it
    let num_string: String = input[start..*offset].iter().collect();
    let number = num_string.parse::<f64>().unwrap();

    return JSONValue::Number(number);
}

fn parse_special(input: &[char], offset: &mut usize) -> JSONValue {
    match input[*offset..] {
        ['t', 'r', 'u', 'e', ..] => {
            *offset += 4;
            return JSONValue::Boolean(true);
        }
        ['f', 'a', 'l', 's', 'e', ..] => {
            *offset += 5;
            return JSONValue::Boolean(false);
        }
        ['n', 'u', 'l', 'l', ..] => {
            *offset += 4;
            return JSONValue::Null;
        }
        _ => panic!("unknown value at offset {offset}"),
    }
}

#[inline]
fn skip_spaces(input: &[char], offset: &mut usize) {
    while input[*offset].is_whitespace() {
        *offset += 1;
    }
}
