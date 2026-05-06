use super::core::Parser;
use crate::ast::Value;
use crate::error::ParseError;
use std::collections::HashMap;

impl<'a> Parser<'a> {
    pub fn insert_dotted(
        &mut self,
        map: &mut HashMap<String, Value>,
        keys: &[String],
        val: Value,
    ) -> Result<(), String> {
        if keys.is_empty() {
            return Err("empty key".into());
        }

        if keys.len() == 1 {
            if map.contains_key(&keys[0]) {
                return Err(format!("duplicate key: {}", keys[0]));
            }

            map.insert(keys[0].clone(), val);

            return Ok(());
        }
        let entry = map
            .entry(keys[0].clone())
            .or_insert_with(|| Value::Table(HashMap::new()));

        match entry {
            Value::Table(inner) => self.insert_dotted(inner, &keys[1..], val),

            _ => Err(format!("key {} is not a table", keys[0])),
        }
    }

    pub fn ensure_array(
        &mut self,
        map: &mut HashMap<String, Value>,
        path: &[String],
    ) -> Result<(), String> {
        if path.is_empty() {
            return Err("empty path".into());
        }

        if path.len() == 1 {
            let entry = map
                .entry(path[0].clone())
                .or_insert_with(|| Value::Array(vec![]));

            if !matches!(entry, Value::Array(_)) {
                return Err(format!("{} is not an array", path[0]));
            }

            return Ok(());
        }

        let entry = map
            .entry(path[0].clone())
            .or_insert_with(|| Value::Table(HashMap::new()));

        match entry {
            Value::Table(inner) => self.ensure_array(inner, &path[1..]),

            Value::Array(arr) => {
                if let Some(Value::Table(last)) = arr.last_mut() {
                    self.ensure_array(last, &path[1..])
                } else {
                    Err(format!("{} has no table entries", path[0]))
                }
            }

            _ => Err(format!("{} is not a table", path[0])),
        }
    }

    pub fn push_array_entry(
        &mut self,
        map: &mut HashMap<String, Value>,
        path: &[String],
    ) -> Result<(), String> {
        if path.is_empty() {
            return Err("empty path".into());
        }

        if path.len() == 1 {
            match map.get_mut(&path[0]) {
                Some(Value::Array(arr)) => {
                    arr.push(Value::Table(HashMap::new()));
                    Ok(())
                }

                _ => Err(format!("{} is not an array", path[0])),
            }
        } else {
            match map.get_mut(&path[0]) {
                Some(Value::Table(inner)) => self.push_array_entry(inner, &path[1..]),

                Some(Value::Array(arr)) => {
                    if let Some(Value::Table(last)) = arr.last_mut() {
                        self.push_array_entry(last, &path[1..])
                    } else {
                        Err(format!("{} has no table entries", path[0]))
                    }
                }

                _ => Err(format!("{} is not a table", path[0])),
            }
        }
    }

    pub fn insert_into_last_array_entry(
        &mut self,
        root: &mut HashMap<String, Value>,
        array_path: &[String],
        rel_keys: &[String],
        val: Value,
    ) -> Result<(), String> {
        if array_path.is_empty() {
            return self.insert_dotted(root, rel_keys, val);
        }

        match root.get_mut(&array_path[0]) {
            Some(Value::Array(arr)) if array_path.len() == 1 => {
                if let Some(Value::Table(last)) = arr.last_mut() {
                    self.insert_dotted(last, rel_keys, val)
                } else {
                    Err("array has no table entry".into())
                }
            }

            Some(Value::Table(inner)) => {
                self.insert_into_last_array_entry(inner, &array_path[1..], rel_keys, val)
            }

            Some(Value::Array(arr)) => {
                if let Some(Value::Table(last)) = arr.last_mut() {
                    self.insert_into_last_array_entry(last, &array_path[1..], rel_keys, val)
                } else {
                    Err("array has no table entry".into())
                }
            }

            _ => Err(format!("{} not found", array_path[0])),
        }
    }

    pub fn parse_inline_table(&mut self) -> Result<Value, ParseError> {
        self.advance();

        let mut map = HashMap::new();

        self.skip_ws_and_comments();

        if self.peek() == Some('}') {
            self.advance();

            return Ok(Value::Table(map));
        }

        loop {
            self.skip_ws_and_comments();

            let keys = self.parse_key()?;

            self.skip_ws_and_comments();

            if self.peek() != Some('=') {
                return Err(self.err("expected '=' in inline table"));
            }

            self.advance();
            self.skip_ws_and_comments();

            let val = self.parse_value()?;

            self.insert_dotted(&mut map, &keys, val)
                .map_err(|e| self.err(&e))?;

            self.skip_ws_and_comments();

            match self.peek() {
                Some(',') => {
                    self.advance();
                    // Allow a trailing comma before '}'
                    self.skip_ws_and_comments();
                    if self.peek() == Some('}') {
                        self.advance();
                        break;
                    }
                }

                Some('}') => {
                    self.advance();

                    break;
                }

                _ => return Err(self.err("expected ',' or '}' in inline table")),
            }
        }

        Ok(Value::Table(map))
    }
}
