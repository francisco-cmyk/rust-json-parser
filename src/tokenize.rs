use std::num::ParseFloatError;

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
  let chars: Vec<char> = input.chars().collect();
  let mut index = 0;

  let mut tokens = Vec::new();
  while index < chars.len() {
    let token = make_token(&chars, &mut index)?;
    tokens.push(token);
    index += 1;
  }

  Ok(tokens)
}

fn make_token(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
  let ch = chars[*index];
  let token = match ch {
    '[' => Token::LeftBracket,
    ']' => Token::RightBracket,
    '{' => Token::LeftBrace,
    '}' => Token::RightBrace,
    ',' => Token::Comma,
    ':' => Token::Colon,
    'n' => tokenize_null(chars, index)?,
    'f' => tokenize_false(chars, index)?,
    't' => tokenize_true(chars, index)?,
    c if c.is_ascii_digit() => tokenize_float(chars, index)?,
    '"' => tokenize_string(chars, index)?,
    _ => todo!("implement rest")
  };

  Ok(token)
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
  UnifinishedLiteralValue,
  ParseNumberError(ParseFloatError),
}

fn tokenize_null(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
  for expected_car in "null".chars() {
    if expected_car != chars[*index] {
      return Err(TokenizeError::UnifinishedLiteralValue)
    }
    *index += 1;
  }
  *index -= 1;
  Ok(Token::Null)
}

fn tokenize_true(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
  for expected_char in "true".chars() {
    if expected_char != chars[*index] {
      return Err(TokenizeError::UnifinishedLiteralValue)
    }
    *index += 1;
  }
  *index -= 1;
  Ok(Token::True)
}

fn tokenize_false(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
  for expected_char in "false".chars() {
    if expected_char != chars[*index] {
      return Err(TokenizeError::UnifinishedLiteralValue)
    }
    *index += 1;
  }
  *index -= 1;
  Ok(Token::False)
}

fn tokenize_float(chars: &Vec<char>, curr_index: &mut usize) -> Result<Token, TokenizeError> {
  let mut unparsed_num = String::new();
  let mut has_demical = false;

  while *curr_index < chars.len() {
    let ch = chars[*curr_index];
    match ch {
      c if c.is_ascii_digit() => unparsed_num.push(c),
      c if c == '.' && !has_demical => {
        unparsed_num.push('.');
        has_demical = true;
      }
      _ => break,
    }
    *curr_index += 1;
  }

  match unparsed_num.parse() {
    Ok(f) => Ok(Token::Number(f)),
    Err(err) => Err(TokenizeError::ParseNumberError(err))
  }
}

fn tokenize_string(chars: &Vec<char>, curr_index: &mut usize) -> Result<Token, TokenizeError> {
  let mut string = String::new();

  while *curr_index < chars.len() {
    *curr_index += 1;
    let ch = chars[*curr_index];
    if ch == '"' {
      break;
    }
    string.push(ch)
  }

  Ok(Token::String(string))
}

// fn tokenize_literal(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {

// }

#[derive(Debug,  PartialEq)]
pub enum Token {
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,
  Comma,
  Colon,
  Null,
  False,
  True,
  Number(f64),
  String(String),
}

#[cfg(test)]
mod tests {
  use super::{tokenize, Token, TokenizeError};

  #[test]
  fn just_comma() {
    let input = String::from(",");
    let expected = [Token::Comma];

    let actual = tokenize(input).unwrap();

    assert_eq!(actual, expected)
  }
  #[test]
  fn all_punctuation() {
    let input = String::from("[{]},:");
    let expected = [
      Token::LeftBracket,
      Token::LeftBrace,
      Token::RightBracket,
      Token::RightBrace,
      Token::Comma,
      Token::Colon,
    ];
    let actual = tokenize(input).unwrap();
    assert_eq!(actual, expected)
  }

  #[test]
  fn just_null() {
    let input = String::from("null");
    let expected = [Token::Null];

    let actual = tokenize(input).unwrap();
    assert_eq!(actual, expected)
  }

  #[test]
  fn just_false() {
    let input = String::from("false");
    let expected = [Token::False];

    let actual = tokenize(input).unwrap();
    assert_eq!(actual, expected)
  }
  #[test]
  fn just_true() {
    let input = String::from("true");
    let expected = [Token::True];

    let actual = tokenize(input).unwrap();

    assert_eq!(actual, expected)
  }

  #[test]
  fn true_comma() {
    let input = String::from("true,");
    let expected = [Token::True, Token::Comma];

    let actual = tokenize(input).unwrap();
    assert_eq!(actual, expected)
  }

  #[test]
  fn integer() {
    let input = String::from("123");
    let expected = [Token::Number(123.0)];

    let actual = tokenize(input).unwrap();
    assert_eq!(actual, expected)
  }

  #[test]
  fn floating_point() {
    let input = String::from("1.23");
    let expected = [Token::Number(1.23)];

    let actual = tokenize(input).unwrap();
    assert_eq!(actual, expected)
  }
}