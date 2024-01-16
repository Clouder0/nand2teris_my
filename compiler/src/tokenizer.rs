use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

#[derive(Debug, PartialEq)]
pub enum KeywordType {
    CLASS,
    METHOD,
    FUNCTION,
    CONSTRUCTOR,
    INT,
    BOOLEAN,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(KeywordType),
    Symbol(char),
    Identifier(String),
    IntConst(i64),
    StringConst(String),
}

#[derive(PartialEq)]
enum CommentState {
    None,
    Line,
    Block,
}

#[derive(PartialEq)]
enum TokenType {
    None,
    String,
    Others,
}

pub struct Tokenizer<T> {
    token_buffer: VecDeque<Token>,
    reader: BufReader<T>,
    comment_state: CommentState,
    token_type: TokenType,
    now_token: String,
}

fn parse_keyword(k_str: &str) -> Option<KeywordType> {
    if k_str == "class" {
        Some(KeywordType::CLASS)
    } else if k_str == "constructor" {
        Some(KeywordType::CONSTRUCTOR)
    } else if k_str == "function" {
        Some(KeywordType::FUNCTION)
    } else if k_str == "method" {
        Some(KeywordType::METHOD)
    } else if k_str == "field" {
        Some(KeywordType::FIELD)
    } else if k_str == "static" {
        Some(KeywordType::STATIC)
    } else if k_str == "var" {
        Some(KeywordType::VAR)
    } else if k_str == "int" {
        Some(KeywordType::INT)
    } else if k_str == "char" {
        Some(KeywordType::CHAR)
    } else if k_str == "boolean" {
        Some(KeywordType::BOOLEAN)
    } else if k_str == "void" {
        Some(KeywordType::VOID)
    } else if k_str == "true" {
        Some(KeywordType::TRUE)
    } else if k_str == "false" {
        Some(KeywordType::FALSE)
    } else if k_str == "null" {
        Some(KeywordType::NULL)
    } else if k_str == "this" {
        Some(KeywordType::THIS)
    } else if k_str == "let" {
        Some(KeywordType::LET)
    } else if k_str == "do" {
        Some(KeywordType::DO)
    } else if k_str == "if" {
        Some(KeywordType::IF)
    } else if k_str == "else" {
        Some(KeywordType::ELSE)
    } else if k_str == "while" {
        Some(KeywordType::WHILE)
    } else if k_str == "return" {
        Some(KeywordType::RETURN)
    } else {
        Option::None
    }
}

impl<T> Tokenizer<T>
where
    T: std::io::Read,
{
    pub fn new(path: &str) -> Tokenizer<File> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        Tokenizer {
            token_buffer: VecDeque::new(),
            reader: reader,
            comment_state: CommentState::None,
            token_type: TokenType::None,
            now_token: String::new(),
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        // read until there exists some tokens
        while self.token_buffer.len() == 0 && self.read_line() != Option::None {
            self.read_line();
        }
        self.token_buffer.pop_front()
    }

    fn read_line(&mut self) -> Option<usize> {
        let mut buf = String::new();
        let ret = self.reader.read_line(&mut buf).unwrap();
        let mut token_parsed: usize = 0;
        let mut escaping = false;
        if ret == 0 {
            // indicates EOF
            return Option::None;
        }
        buf.push('\n');
        let mut last_char: Option<char> = None;
        for (idx, c) in buf.char_indices() {
            'end: loop {
                match self.comment_state {
                    CommentState::Line => {
                        if c == '\n' {
                            self.comment_state = CommentState::None;
                        }
                        break 'end;
                    }
                    CommentState::Block => {
                        if c == '/' && last_char == Some('*') {
                            // block comment end
                            if self.comment_state != CommentState::Block {
                                panic!("Invalid comment");
                            }
                            self.comment_state = CommentState::None;
                        }
                        break 'end;
                    }
                    CommentState::None => {}
                }
                match self.token_type {
                    TokenType::String => {
                        if escaping {
                            if c == 'n' {
                                self.now_token.push('\n');
                            } else if c == 't' {
                                self.now_token.push('\t');
                            } else if c == 'r' {
                                self.now_token.push('\r');
                            } else {
                                self.now_token.push(c);
                            }
                            escaping = false;
                        } else if c == '"' {
                            // string end
                            self.token_type = TokenType::None;
                            token_parsed += 1;
                            self.token_buffer
                                .push_back(Token::StringConst(mem::take(&mut self.now_token)));
                        } else if c == '\\' {
                            // escape, do nothing
                            escaping = true;
                        } else {
                            // normal char
                            self.now_token.push(c);
                        }
                    }
                    TokenType::Others => {
                        if c == ' ' || c == '\n' {
                            // end of token
                            token_parsed += 1;
                            self.token_buffer.push_back(
                                parse_keyword(&self.now_token)
                                    .map(|x| Token::Keyword(x))
                                    .or_else(|| {
                                        // try parse as int
                                        self.now_token
                                            .parse::<i64>()
                                            .ok()
                                            .map(|x| Token::IntConst(x))
                                            .or_else(|| {
                                                Some(Token::Identifier(mem::take(
                                                    &mut self.now_token,
                                                )))
                                            })
                                    })
                                    .unwrap(),
                            );
                            self.now_token = String::new();
                            self.token_type = TokenType::None;
                        } else {
                            // normal character
                            self.now_token.push(c);
                        }
                    }
                    TokenType::None => {
                        if c == '"' {
                            // string start
                            self.token_type = TokenType::String;
                        } else if c == ' ' || c == '\n' {
                            // ignore
                        } else if c == '/' && last_char == Some('/') {
                            // inline comment start
                            self.comment_state = CommentState::Line;
                        } else if c == '*' && last_char == Some('/') {
                            // block comment start
                            self.comment_state = CommentState::Block;
                        } else if c == '{'
                            || c == '}'
                            || c == '('
                            || c == ')'
                            || c == '['
                            || c == ']'
                            || c == '.'
                            || c == ','
                            || c == ';'
                            || c == '+'
                            || c == '-'
                            || c == '*'
                            || (c == '/'
                                && buf
                                    .chars()
                                    .nth(idx + 1)
                                    .and_then(|x| if x == '/' || x == '*' { Some(x) } else { None })
                                    .is_none())
                            || c == '&'
                            || c == '|'
                            || c == '<'
                            || c == '>'
                            || c == '='
                            || c == '~'
                        {
                            token_parsed += 1;
                            self.token_buffer.push_back(Token::Symbol(c));
                            self.token_type = TokenType::None;
                        } else if c == '/' {
                            // ignore
                        } else {
                            // normal character
                            self.token_type = TokenType::Others;
                            self.now_token.push(c);
                        }
                    }
                }
                break 'end;
            }
            last_char = Some(c);
        }
        if self.comment_state == CommentState::Line {
            // line level comment should set to none at the end of line
            self.comment_state = CommentState::None;
        }
        Some(token_parsed)
    }
}

pub fn tokenize(path: &str) -> Vec<Token> {
    let mut tokenizer: Tokenizer<File> = Tokenizer::<File>::new(path);
    let mut tokens = Vec::new();
    loop {
        let c = tokenizer.advance();
        if c.is_none() {
            break tokens;
        }
        tokens.push(c.unwrap());
    }
}

#[allow(dead_code)]
pub fn tokenize_str(s: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer {
        token_buffer: VecDeque::new(),
        reader: BufReader::new(s.as_bytes()),
        comment_state: CommentState::None,
        token_type: TokenType::None,
        now_token: String::new(),
    };
    let mut tokens = Vec::new();
    loop {
        let c = tokenizer.advance();
        if c.is_none() {
            break tokens;
        }
        tokens.push(c.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::*;
    // use super::*;

    #[test]
    fn single_identifier() {
        let source = "abc";
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::Identifier("abc".to_string()));
    }

    #[test]
    fn multiple_identifiers() {
        let source = "abc def ghi\naaa\nbbb";
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::Identifier("abc".to_string()));
        assert_eq!(ret[1], Token::Identifier("def".to_string()));
        assert_eq!(ret[2], Token::Identifier("ghi".to_string()));
        assert_eq!(ret[3], Token::Identifier("aaa".to_string()));
        assert_eq!(ret[4], Token::Identifier("bbb".to_string()));
    }

    #[test]
    fn single_symbol() {
        let source = "+";
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::Symbol('+'));
    }

    #[test]
    fn int_const() {
        let source = "123567";
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::IntConst(123567));
    }

    #[test]
    fn string_const() {
        let source = r#""This is a string""#;
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::StringConst("This is a string".to_string()));
    }

    #[test]
    fn string_const_with_symbols() {
        let source = r#""This is a string with symbols: {}[]()<>.,;+-*/&|~""#;
        let ret = tokenize_str(source);
        assert_eq!(
            ret[0],
            Token::StringConst("This is a string with symbols: {}[]()<>.,;+-*/&|~".to_string())
        );
    }

    #[test]
    fn string_const_with_escapes() {
        let source = r#""a\\b""#;
        println!("source: {}", source);
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::StringConst(r#"a\b"#.to_string()));
    }

    #[test]
    fn string_const_with_escapes2() {
        let source = r#""a\nb""#;
        println!("source: {}", source);
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::StringConst("a\nb".to_string()));
    }

    #[test]
    fn string_const_with_escape_slash() {
        let source = r#""b\\\\b""#;
        println!("source: {}", source);
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::StringConst(r#"b\\b"#.to_string()));
    }

    #[test]
    fn string_const_number() {
        let source = r#""123""#;
        println!("source: {}", source);
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::StringConst("123".to_string()));
    }

    #[test]
    fn keywords() {
        let source = "class method function constructor int boolean char void var static field let do if else while return true false null this";
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::Keyword(KeywordType::CLASS));
        assert_eq!(ret[1], Token::Keyword(KeywordType::METHOD));
        assert_eq!(ret[2], Token::Keyword(KeywordType::FUNCTION));
        assert_eq!(ret[3], Token::Keyword(KeywordType::CONSTRUCTOR));
        assert_eq!(ret[4], Token::Keyword(KeywordType::INT));
        assert_eq!(ret[5], Token::Keyword(KeywordType::BOOLEAN));
        assert_eq!(ret[6], Token::Keyword(KeywordType::CHAR));
        assert_eq!(ret[7], Token::Keyword(KeywordType::VOID));
        assert_eq!(ret[8], Token::Keyword(KeywordType::VAR));
        assert_eq!(ret[9], Token::Keyword(KeywordType::STATIC));
        assert_eq!(ret[10], Token::Keyword(KeywordType::FIELD));
        assert_eq!(ret[11], Token::Keyword(KeywordType::LET));
        assert_eq!(ret[12], Token::Keyword(KeywordType::DO));
        assert_eq!(ret[13], Token::Keyword(KeywordType::IF));
        assert_eq!(ret[14], Token::Keyword(KeywordType::ELSE));
        assert_eq!(ret[15], Token::Keyword(KeywordType::WHILE));
        assert_eq!(ret[16], Token::Keyword(KeywordType::RETURN));
        assert_eq!(ret[17], Token::Keyword(KeywordType::TRUE));
        assert_eq!(ret[18], Token::Keyword(KeywordType::FALSE));
        assert_eq!(ret[19], Token::Keyword(KeywordType::NULL));
        assert_eq!(ret[20], Token::Keyword(KeywordType::THIS));
    }

    #[test]
    fn symbols() {
        let source = "{}[]().,;+-*/&|~<>=";
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::Symbol('{'));
        assert_eq!(ret[1], Token::Symbol('}'));
        assert_eq!(ret[2], Token::Symbol('['));
        assert_eq!(ret[3], Token::Symbol(']'));
        assert_eq!(ret[4], Token::Symbol('('));
        assert_eq!(ret[5], Token::Symbol(')'));
        assert_eq!(ret[6], Token::Symbol('.'));
        assert_eq!(ret[7], Token::Symbol(','));
        assert_eq!(ret[8], Token::Symbol(';'));
        assert_eq!(ret[9], Token::Symbol('+'));
        assert_eq!(ret[10], Token::Symbol('-'));
        assert_eq!(ret[11], Token::Symbol('*'));
        assert_eq!(ret[12], Token::Symbol('/'));
        assert_eq!(ret[13], Token::Symbol('&'));
        assert_eq!(ret[14], Token::Symbol('|'));
        assert_eq!(ret[15], Token::Symbol('~'));
        assert_eq!(ret[16], Token::Symbol('<'));
        assert_eq!(ret[17], Token::Symbol('>'));
        assert_eq!(ret[18], Token::Symbol('='));
    }

    #[test]
    fn comments() {
        let source = r#"// this is a comment
        // this is another comment
        a
        b /* this is a block comment */ c
        d /* this is a block comment with 
        multiple lines comment */ e
        f "#;
        let ret = tokenize_str(source);
        assert_eq!(ret[0], Token::Identifier("a".to_string()));
        assert_eq!(ret[1], Token::Identifier("b".to_string()));
        assert_eq!(ret[2], Token::Identifier("c".to_string()));
        assert_eq!(ret[3], Token::Identifier("d".to_string()));
        assert_eq!(ret[4], Token::Identifier("e".to_string()));
        assert_eq!(ret[5], Token::Identifier("f".to_string()));
    }
}
