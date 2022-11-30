#[derive(Debug, PartialEq, Eq)]
enum Token {
    Map,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Field(String),
    Type(ValueType),
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

        let mut chars = self.program.chars();

        while let Some(ch) = chars.next() {
            match ch {
                '{' => tokens.push(Token::LBracket),
                '}' => tokens.push(Token::RBracket),
                ':' => tokens.push(Token::Colon),
                ',' => tokens.push(Token::Comma),
                _ => {}
            }

            self.pos += 1;
        }

        tokens
    }

    fn next_token(&self) -> Option<Token> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn lex_program() {
        let mut lexer = Lexer::new("{}:,");
        let tokens = lexer.lex();

        let expected = vec![Token::LBracket, Token::RBracket, Token::Colon, Token::Comma];
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
