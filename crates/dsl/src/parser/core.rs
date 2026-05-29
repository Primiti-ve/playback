use crate::{ast::Value, error::ParseError};
use std::collections::HashMap;

pub struct Parser<'a> {
    pub input: &'a str,
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0, line: 1, column: 1 }
    }

    pub fn remaining(&self) -> &str {
        &self.input[self.pos..]
    }

    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;

        self.pos += ch.len_utf8();

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    pub fn skip_whitespace_inline(&mut self) {
        while matches!(self.peek(), Some(' ') | Some('\t')) {
            self.advance();
        }
    }

    pub fn skip_whitespace_and_newlines(&mut self) {
        while matches!(self.peek(), Some(' ') | Some('\t') | Some('\n') | Some('\r')) {
            self.advance();
        }
    }

    pub fn skip_comment(&mut self) {
        if self.peek() == Some('#') {
            while !matches!(self.peek(), Some('\n') | None) {
                self.advance();
            }
        }
    }

    pub fn skip_line_end(&mut self) {
        self.skip_whitespace_inline();
        self.skip_comment();

        if self.peek() == Some('\r') {
            self.advance();
        }

        if self.peek() == Some('\n') {
            self.advance();
        }
    }

    pub fn skip_ws_and_comments(&mut self) {
        loop {
            let before = self.pos;

            self.skip_whitespace_and_newlines();
            self.skip_comment();

            if self.pos == before {
                break;
            }
        }
    }

    pub fn err(&self, msg: &str) -> ParseError {
        ParseError {
            message: msg.to_string(),
            line: self.line,
            column: self.column,
            snippet: self.current_line(),
        }
    }

    pub fn current_line(&self) -> String {
        self.input.lines().nth(self.line - 1).unwrap_or("").to_string()
    }

    pub fn parse(&mut self) -> Result<HashMap<String, Value>, ParseError> {
        let mut root: HashMap<String, Value> = HashMap::new();
        let mut current_path: Vec<String> = vec![];
        let mut is_array_table = false;

        loop {
            self.skip_ws_and_comments();

            match self.peek() {
                None => break,

                Some('[') => {
                    self.advance();

                    is_array_table = self.peek() == Some('[');

                    if is_array_table {
                        self.advance();
                    }

                    self.skip_whitespace_inline();

                    let path = self.parse_key()?;

                    self.skip_whitespace_inline();

                    if is_array_table {
                        if self.peek() != Some(']') {
                            return Err(self.err("expected ']]'"));
                        }

                        self.advance();

                        if self.peek() != Some(']') {
                            return Err(self.err("expected ']]'"));
                        }

                        self.advance();
                    } else {
                        if self.peek() != Some(']') {
                            return Err(self.err("expected ']'"));
                        }

                        self.advance();
                    }

                    self.skip_line_end();
                    current_path = path.clone();

                    if is_array_table {
                        self.ensure_array(&mut root, &path).map_err(|e| self.err(&e))?;
                        self.push_array_entry(&mut root, &path).map_err(|e| self.err(&e))?;
                    }
                }

                _ => {
                    let keys = self.parse_key()?;

                    self.skip_whitespace_inline();

                    if self.peek() != Some('=') {
                        return Err(self.err("expected '='"));
                    }

                    self.advance();
                    self.skip_whitespace_inline();

                    let val = self.parse_value()?;

                    self.skip_line_end();

                    let mut full_path = current_path.clone();

                    full_path.extend(keys);

                    if is_array_table && !current_path.is_empty() {
                        self.insert_into_last_array_entry(&mut root, &current_path, &full_path[current_path.len()..], val)
                            .map_err(|e| self.err(&e))?;
                    } else {
                        self.insert_dotted(&mut root, &full_path, val).map_err(|e| self.err(&e))?;
                    }
                }
            }
        }

        Ok(root)
    }
}
