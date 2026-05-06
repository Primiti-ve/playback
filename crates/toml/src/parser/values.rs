use super::core::Parser;
use crate::ast::Value;
use crate::error::ParseError;

impl<'a> Parser<'a> {
    pub fn parse_key(&mut self) -> Result<Vec<String>, ParseError> {
        let mut parts = vec![self.parse_simple_key()?];

        while self.peek() == Some('.') {
            self.advance();
            parts.push(self.parse_simple_key()?);
        }

        Ok(parts)
    }

    pub fn parse_simple_key(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Some('"') => self.parse_basic_string(),
            Some('\'') => self.parse_literal_string(),

            Some(c) if c.is_alphanumeric() || c == '-' || c == '_' => {
                let mut key = String::new();

                while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '-' || c == '_')
                {
                    key.push(self.advance().unwrap());
                }

                Ok(key)
            }

            _ => Err(self.err("expected a key")),
        }
    }

    pub fn parse_escape(&mut self) -> Result<char, ParseError> {
        self.advance();
        self.parse_escape_char()
    }

    pub fn parse_escape_char(&mut self) -> Result<char, ParseError> {
        match self.advance() {
            Some('b') => Ok('\x08'),
            Some('t') => Ok('\t'),
            Some('n') => Ok('\n'),
            Some('f') => Ok('\x0C'),
            Some('r') => Ok('\r'),
            Some('"') => Ok('"'),
            Some('\\') => Ok('\\'),
            Some('u') => self.parse_unicode(4),
            Some('U') => self.parse_unicode(8),

            other => Err(self.err(&format!("invalid escape: {:?}", other))),
        }
    }

    pub fn parse_unicode(&mut self, digits: usize) -> Result<char, ParseError> {
        let mut hex = String::new();

        for _ in 0..digits {
            match self.advance() {
                Some(c) if c.is_ascii_hexdigit() => hex.push(c),

                _ => return Err(self.err("invalid unicode escape")),
            }
        }

        let code =
            u32::from_str_radix(&hex, 16).map_err(|_| self.err("invalid unicode codepoint"))?;

        char::from_u32(code).ok_or_else(|| self.err("invalid unicode codepoint"))
    }

    pub fn parse_value(&mut self) -> Result<Value, ParseError> {
        if self.remaining().starts_with("if") {
            return self.parse_conditional();
        }

        match self.peek() {
            Some('"') => self.parse_basic_string().map(Value::String),
            Some('\'') => self.parse_literal_string().map(Value::String),
            Some('[') => self.parse_array(),

            Some('{') => {
                if self.is_block_value()? {
                    self.parse_block_value()
                } else {
                    self.parse_inline_table()
                }
            }

            _ => self.parse_number_or_bool_or_date(),
        }
    }

    pub fn is_block_value(&mut self) -> Result<bool, ParseError> {
        // Skip the opening '{' and any whitespace/newlines to peek at what follows.
        let rest = self
            .remaining()
            .chars()
            .skip(1)
            .skip_while(|c| matches!(c, ' ' | '\t' | '\n' | '\r'));

        let peeked: String = rest.take(2).collect();

        Ok(peeked.starts_with("if") || peeked.starts_with('['))
    }

    pub fn expect_keyword(&mut self, kw: &str) -> Result<(), ParseError> {
        if self.remaining().starts_with(kw) {
            for _ in 0..kw.len() {
                self.advance();
            }

            Ok(())
        } else {
            Err(self.err(&format!("expected '{}'", kw)))
        }
    }

    pub fn parse_array(&mut self) -> Result<Value, ParseError> {
        self.advance();

        let mut arr = Vec::new();

        loop {
            self.skip_whitespace_and_newlines();
            self.skip_comment();
            self.skip_whitespace_and_newlines();

            if self.peek() == Some(']') {
                self.advance();

                break;
            }

            arr.push(self.parse_value()?);

            self.skip_whitespace_and_newlines();
            self.skip_comment();
            self.skip_whitespace_and_newlines();

            match self.peek() {
                Some(',') => {
                    self.advance();
                }

                Some(']') => {
                    self.advance();

                    break;
                }

                _ => return Err(self.err("expected ',' or ']' in array")),
            }
        }

        Ok(Value::Array(arr))
    }

    pub fn parse_conditional(&mut self) -> Result<Value, ParseError> {
        self.expect_keyword("if")?;
        self.skip_whitespace_inline();

        let condition = self.parse_expr()?;

        self.skip_whitespace_inline();

        let then_branch = self.parse_block_value()?;

        self.skip_whitespace_inline();

        let else_branch = if self.remaining().starts_with("else") {
            self.expect_keyword("else")?;
            self.skip_whitespace_inline();

            Some(Box::new(self.parse_block_value()?))
        } else {
            None
        };

        Ok(Value::Conditional {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    pub fn parse_block_value(&mut self) -> Result<Value, ParseError> {
        if self.peek() != Some('{') {
            return Err(self.err("expected '{'"));
        }

        self.advance();
        self.skip_whitespace_and_newlines();

        let val = self.parse_value()?;

        self.skip_whitespace_and_newlines();

        if self.peek() != Some('}') {
            return Err(self.err("expected '}'"));
        }

        self.advance();

        Ok(val)
    }

    pub fn parse_number_or_bool_or_date(&mut self) -> Result<Value, ParseError> {
        if self.remaining().starts_with("true")
            && !self.remaining()[4..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            for _ in 0..4 {
                self.advance();
            }

            return Ok(Value::Boolean(true));
        }

        if self.remaining().starts_with("false")
            && !self.remaining()[5..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        {
            for _ in 0..5 {
                self.advance();
            }

            return Ok(Value::Boolean(false));
        }

        let mut raw = String::new();

        while matches!(self.peek(), Some(c) if !matches!(c, ',' | ']' | '}' | '\n' | '\r' | '#')) {
            raw.push(self.advance().unwrap());
        }

        let raw = raw.trim_end().to_string();
        let clean = raw.replace('_', "");

        if clean.contains('.')
            || clean.contains('e')
            || clean.contains('E')
            || clean == "inf"
            || clean == "+inf"
            || clean == "-inf"
            || clean == "nan"
            || clean == "+nan"
            || clean == "-nan"
        {
            return match clean.as_str() {
                "inf" | "+inf" => Ok(Value::Float(f64::INFINITY)),
                "-inf" => Ok(Value::Float(f64::NEG_INFINITY)),
                "nan" | "+nan" | "-nan" => Ok(Value::Float(f64::NAN)),

                _ => clean
                    .parse::<f64>()
                    .map(Value::Float)
                    .map_err(|_| self.err(&format!("invalid float: {}", raw))),
            };
        }

        if clean.starts_with("0x") {
            return i64::from_str_radix(&clean[2..], 16)
                .map(Value::Integer)
                .map_err(|_| self.err(&format!("invalid hex: {}", raw)));
        }

        if clean.starts_with("0o") {
            return i64::from_str_radix(&clean[2..], 8)
                .map(Value::Integer)
                .map_err(|_| self.err(&format!("invalid octal: {}", raw)));
        }

        if clean.starts_with("0b") {
            return i64::from_str_radix(&clean[2..], 2)
                .map(Value::Integer)
                .map_err(|_| self.err(&format!("invalid binary: {}", raw)));
        }

        clean
            .parse::<i64>()
            .map(Value::Integer)
            .map_err(|_| self.err(&format!("invalid value: {}", raw)))
    }
}
