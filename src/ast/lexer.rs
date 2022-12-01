use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Map,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Field(String),
    Type(ValueType),
    Whitespace,
    Pipe,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
enum ValueType {
    Str,
    Int,
    Timestamp(Option<String>),
}

struct Lexer {
    pos: usize,
    program: String,
}

impl Lexer {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            pos: 0,
            program: program.into(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut peekable = self.program.chars().peekable();

        while let Some(token) = self.next_token(&mut peekable) {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&self, peekable: &mut Peekable<Chars>) -> Option<Token> {
        match peekable.peek() {
            Some(ch) if ch.is_whitespace() => self.consume_and_return(peekable, Token::Whitespace),
            Some(ch) if ch.is_alphabetic() => {
                let mut word = String::new();
                while let Some(c) = peekable.peek() {
                    if c.is_whitespace() {
                        let tok = match word.as_str() {
                            "map" => Token::Map,
                            _ => Token::Field(word),
                        };
                        return Some(tok);
                    } else {
                        word.push_str(&c.to_string());
                        peekable.next();
                    }
                }
                None
            }
            Some('{') => self.consume_and_return(peekable, Token::LBracket),
            Some('}') => self.consume_and_return(peekable, Token::RBracket),
            Some(':') => self.consume_and_return(peekable, Token::Colon),
            Some(',') => self.consume_and_return(peekable, Token::Comma),
            Some('|') => self.consume_and_return(peekable, Token::Pipe),
            _ => None,
        }
    }

    fn consume_and_return(&self, chars: &mut Peekable<Chars>, t: Token) -> Option<Token> {
        chars.next();
        Some(t)
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn lex_program() {
        let mut lexer = Lexer::new("map { timestamp bytes }:,");
        let tokens = lexer.lex();

        let expected = vec![
            Token::Map,
            Token::Whitespace,
            Token::LBracket,
            Token::Whitespace,
            Token::Field("timestamp".into()),
            Token::Whitespace,
            Token::Field("bytes".into()),
            Token::Whitespace,
            Token::RBracket,
            Token::Colon,
            Token::Comma,
        ];
        assert_eq!(expected, tokens);
    }

    // #[test]
    // fn parse_map_function() {
    //     let lexer = Lexer::new(
    //         r#"map { ts:ts:"%Y-%m-%d", ip:str, method:str, bytes:int, status_code:int }"#,
    //     );
    //     let tokens = lexer.lex();

    //     let expected = vec![
    //         Token::Map,
    //         Token::LBracket,
    //         Token::Field("ts".into()),
    //         Token::Colon,
    //         Token::Type(ValueType::Timestamp(Some("%Y-%m-%d".into()))),
    //         Token::Comma,
    //         Token::Field("ip".into()),
    //         Token::Colon,
    //         Token::Type(ValueType::Str),
    //         Token::Comma,
    //         Token::Field("bytes".into()),
    //         Token::Colon,
    //         Token::Type(ValueType::Int),
    //         Token::Comma,
    //         Token::Field("status_code".into()),
    //         Token::Colon,
    //         Token::Type(ValueType::Int),
    //         Token::RBracket,
    //     ];

    //     assert_eq!(expected, tokens);
    // }
}
