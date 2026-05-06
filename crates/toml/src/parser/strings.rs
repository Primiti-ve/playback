use super::core::Parser;
use crate::error::ParseError;

impl<'a> Parser<'a> {
    pub fn parse_basic_string(&mut self) -> Result<String, ParseError> {
        self.advance();

        if self.remaining().starts_with("\"\"") {
            self.advance();
            self.advance();

            return self.parse_multiline_basic_string();
        }

        let mut s = String::new();

        loop {
            match self.peek() {
                None | Some('\n') => return Err(self.err("unterminated string")),
                Some('"') => {
                    self.advance();
                    return Ok(s);
                }
                Some('\\') => s.push(self.parse_escape()?),

                _ => s.push(self.advance().unwrap()),
            }
        }
    }

    pub fn parse_multiline_basic_string(&mut self) -> Result<String, ParseError> {
        if self.peek() == Some('\n') {
            self.advance();
        } else if self.remaining().starts_with("\r\n") {
            self.advance();
            self.advance();
        }

        let mut s = String::new();

        loop {
            if self.remaining().starts_with("\"\"\"") {
                self.advance();
                self.advance();
                self.advance();

                return Ok(s);
            }

            match self.peek() {
                None => return Err(self.err("unterminated multi-line string")),

                Some('\\') => {
                    self.advance();

                    if matches!(
                        self.peek(),
                        Some(' ') | Some('\t') | Some('\n') | Some('\r')
                    ) {
                        self.skip_whitespace_and_newlines();
                    } else {
                        s.push(self.parse_escape_char()?);
                    }
                }

                _ => s.push(self.advance().unwrap()),
            }
        }
    }

    pub fn parse_literal_string(&mut self) -> Result<String, ParseError> {
        self.advance();

        if self.remaining().starts_with("''") {
            self.advance();
            self.advance();

            return self.parse_multiline_literal_string();
        }

        let mut s = String::new();

        loop {
            match self.peek() {
                None | Some('\n') => return Err(self.err("unterminated literal string")),
                Some('\'') => {
                    self.advance();
                    return Ok(s);
                }

                _ => s.push(self.advance().unwrap()),
            }
        }
    }

    pub fn parse_multiline_literal_string(&mut self) -> Result<String, ParseError> {
        if self.peek() == Some('\n') {
            self.advance();
        } else if self.remaining().starts_with("\r\n") {
            self.advance();
            self.advance();
        }

        let mut s = String::new();

        loop {
            if self.remaining().starts_with("'''") {
                self.advance();
                self.advance();
                self.advance();

                return Ok(s);
            }

            match self.peek() {
                None => return Err(self.err("unterminated multi-line literal string")),

                _ => s.push(self.advance().unwrap()),
            }
        }
    }
}
