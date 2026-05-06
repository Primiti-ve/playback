use super::core::Parser;
use crate::ast::Expr;
use crate::error::ParseError;

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.skip_ws();

        let left = self.parse_expr_atom()?;

        self.skip_ws();

        if self.remaining().starts_with("!=") {
            return self.parse_binary_op(left, "!=", Expr::NotEquals);
        }

        if self.remaining().starts_with("==") {
            return self.parse_binary_op(left, "==", Expr::Equals);
        }

        Ok(left)
    }

    fn parse_binary_op<F>(&mut self, left: Expr, op: &str, ctor: F) -> Result<Expr, ParseError>
    where
        F: Fn(Box<Expr>, Box<Expr>) -> Expr,
    {
        for _ in 0..op.len() {
            self.advance();
        }

        self.skip_ws();

        let right = self.parse_expr_atom()?;

        Ok(ctor(Box::new(left), Box::new(right)))
    }

    pub fn parse_expr_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some('"') => Ok(Expr::String(self.parse_basic_string()?)),

            Some(c) if c.is_alphanumeric() || c == '_' => {
                let mut ident = String::new();

                while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
                    ident.push(self.advance().unwrap());
                }

                Ok(Expr::Identifier(ident))
            }

            _ => Err(self.err("invalid expression")),
        }
    }

    pub fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ') | Some('\t')) {
            self.advance();
        }
    }
}
