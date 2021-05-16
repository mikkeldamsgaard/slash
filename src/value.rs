use std::collections::HashMap;
use std::rc::Rc;
use crate::closure::Closure;
use pest::Span;
use crate::error::SlashError;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Value {
    Table(Rc<RefCell<HashMap<String, Value>>>),
    List(Rc<RefCell<Vec<Value>>>),
    Number(f64),
    String(String),
    Function(Rc<Vec<String>>, String, Closure),
    ProcessResult(Option<i32>, String, String),
}

impl Value {
    pub fn add(self, rhs: Self, span: Span) -> Result<Value, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(Number(lhs_val + rhs_val)),
                    _ => Err(SlashError::new(&span, "Add left hand side is number, expected number on right hand side"))
                }
            }
            String(lhs_val) => {
                match rhs {
                    String(rhs_val) => Ok(String(lhs_val + &rhs_val)),
                    _ => Err(SlashError::new(&span, "Add left hand side is string, expected string on right hand side"))
                }
            }
            _ => Err(SlashError::new(&span, &format!("Add not defined on left hand argument value {}", self.value_type())))
        }
    }

    pub fn sub(self, rhs: Self, span: Span) -> Result<Value, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(Number(lhs_val - rhs_val)),
                    _ => Err(SlashError::new(&span, "Subtraction left hand side is number, expected number on right hand side"))
                }
            }
            _ => Err(SlashError::new(&span, &format!("Subtraction not defined on left hand argument value {}", self.value_type())))
        }
    }

    pub fn mul(self, rhs: Value, span: Span) -> Result<Value, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(Number(lhs_val * rhs_val)),
                    _ => Err(SlashError::new(&span, "Multiplication left hand side is number, expected number on right hand side"))
                }
            }
            _ => Err(SlashError::new(&span, &format!("Multiplication not defined on left hand argument value {}", self.value_type())))
        }
    }

    pub fn div(self, rhs: Value, span: Span) -> Result<Value, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(Number(lhs_val / rhs_val)),
                    _ => Err(SlashError::new(&span, "Division left hand side is number, expected number on right hand side"))
                }
            }
            _ => Err(SlashError::new(&span, &format!("Division not defined on left hand argument value {}", self.value_type())))
        }
    }

    pub fn powf(&self, rhs: Value, span: Span) -> Result<Value, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(Number(lhs_val.powf(rhs_val))),
                    _ => Err(SlashError::new(&span, "Power left hand side is number, expected number on right hand side"))
                }
            }
            _ => Err(SlashError::new(&span, &format!("Power not defined on left hand argument value {}", self.value_type())))
        }
    }

    pub fn string_from_syntax(parsed: &str) -> Value {
        Value::String(Value::convert_parsed_string(parsed))
    }

    pub fn convert_parsed_string(parsed: &str) -> String {
        let expanded = String::from(parsed
            .replace("\\\"", "\"")
            .replace("\\\n", "\n")
            .replace("\\\t", "\t")
            .replace("\\\r", "\r")
        );
        String::from(&expanded[1..(expanded.len() - 1)])
    }

    fn escape_string(str: &str) -> String {
        String::from(str
            .replace("\"", "\\\"")
            .replace("\n", "\\\n")
            .replace("\t", "\\\t")
            .replace("\r", "\\\r")
        )
    }

    pub fn is_true(&self) -> bool {
        match self {
            Value::Number(n) => n != &0.0,
            Value::String(s) => s.len() != 0,
            Value::List(l) => l.borrow().len() != 0,
            Value::Table(t) => t.borrow().len() != 0,
            Value::Function(..) => true,
            Value::ProcessResult(exit_code, _, _) => if let Some(e) = exit_code { *e == 0 } else { false }
        }
    }


    pub fn or(&self, rhs: Value) -> Value {
        return bool_to_value(self.is_true() || rhs.is_true());
    }

    pub fn and(&self, rhs: Value) -> Value {
        return bool_to_value(self.is_true() && rhs.is_true());
    }

    fn _equals(&self, rhs: &Value, span: &Span) -> Result<bool, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(lhs_val == rhs_val),
                    _ => self.type_mismatch_error(&rhs, span)
                }
            }
            String(lhs_val) => {
                match rhs {
                    String(rhs_val) => Ok(lhs_val.eq(rhs_val)),
                    _ => self.type_mismatch_error(&rhs, span)
                }
            }
            List(lhs_val) => {
                match rhs {
                    List(rhs_val) => {
                        let lhs_val = lhs_val.borrow();
                        let rhs_val = rhs_val.borrow();
                        if rhs_val.len() != lhs_val.len() {
                            Ok(false)
                        } else {
                            for i in 0..rhs_val.len() {
                                if !rhs_val[i]._equals(&lhs_val[i], span)? {
                                    return Ok(false);
                                }
                            }
                            Ok(true)
                        }
                    }
                    _ => self.type_mismatch_error(&rhs, span)
                }
            }
            // Table(lhs_val) => {
            //     match rhs {
            //         Table(rhs_val) => lhs_val.eq(&rhs_val),
            //         _ => panic!("Type mismatch in comparison")
            //     }
            // }
            _ => self.type_mismatch_error(&rhs, span)
        }
    }

    fn type_mismatch_error(&self, rhs: &&Value, span: &Span) -> Result<bool, SlashError> {
        Err(SlashError::new(&span, &format!("Type mismatch in comparison. Cannot compare {} to {}", self.value_type(), rhs.value_type())))
    }

    pub fn equals(&self, rhs: Value, span: Span) -> Result<Value, SlashError> {
        Ok(bool_to_value(self._equals(&rhs, &span)?))
    }

    pub fn not_equals(&self, rhs: Value, span: Span) -> Result<Value, SlashError> {
        Ok(bool_to_value(!self._equals(&rhs, &span)?))
    }

    pub fn _less_than(&self, rhs: &Value, span: &Span) -> Result<bool, SlashError> {
        use Value::*;
        match self {
            Number(lhs_val) => {
                match rhs {
                    Number(rhs_val) => Ok(lhs_val < &rhs_val),
                    _ => self.type_mismatch_error(&rhs, span)
                }
            }
            String(lhs_val) => {
                match rhs {
                    String(rhs_val) => Ok(lhs_val.lt(&rhs_val)),
                    _ => self.type_mismatch_error(&rhs, span)
                }
            }
            // List(lhs_val) => {
            //     match rhs {
            //         List(rhs_val) => lhs_val.lt(&rhs_val),
            //         _ => panic!("Type mismatch in comparison")
            //     }
            // },
            _ => self.type_mismatch_error(&rhs, span)
        }
    }


    pub fn less_than(&self, rhs: Value, span: Span) -> Result<Value, SlashError> {
        Ok(bool_to_value(self._less_than(&rhs, &span)?))
    }

    pub fn greater_than(&self, rhs: Self, span: Span) -> Result<Value, SlashError> {
        Ok(bool_to_value(!self._less_than(&rhs, &span)? && !self._equals(&rhs, &span)?))
    }

    pub fn value_type(&self) -> &str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::List(_) => "List",
            Value::Table(_) => "Table",
            Value::ProcessResult(_, _, _) => "Process result",
            Value::Function(_, _, _) => "Function",
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            Value::Number(f) => format!("{}", f),
            Value::String(s) => format!("\"{}\"", Value::escape_string(&s)),
            Value::List(l) => {
                let mut s = String::from("");
                l.borrow().iter().for_each(|v| s.push_str(&format!(", {}", v.to_json())));
                format!("[{}]", &s[2..])
            }
            Value::Table(t_r) => {
                let t = t_r.borrow();
                let mut s = String::from("");
                t.keys().for_each(|k| s.push_str(&format!(", \"{}\": {}", Value::escape_string(k), t.get(k).unwrap().to_json())));
                format!("{{{}}}", &s[2..])
            }
            Value::ProcessResult(exitcode, stdout, stderr) => {
                let pre;
                if let Some(e) = exitcode {
                    pre = format!("{{ \"exit_code\": {}, ", e)
                } else {
                    pre = String::from("{{ ")
                }
                format!("{} \"stderr\": {}, \"stdout\": {} }}", pre, stderr, stdout)
            }
            Value::Function(_, _, _) => format!("\"<<function>>\"")
        }
    }


    pub fn lookup_by_index(&self, index: Value, span: Span) -> Result<Value, SlashError> {
        match self {
            Value::List(l) => {
                let i_index;
                if let Value::Number(raw) = index {
                    if 0.0 <= raw && raw < l.borrow().len() as f64 {
                        i_index = usize::from(raw as u16);
                        Ok(l.borrow()[i_index].clone())
                    } else {
                        Err(SlashError::new(&span, &format!("Index out of bounds. Value length is {} index was {}", l.borrow().len(), raw)))
                    }
                } else {
                    Err(SlashError::new(&span, &format!("Index value not a number, but a {}",index.value_type())))
                }
            }
            Value::Table(t) => {
                if let Value::String(s) = index {
                    if let Some(val) = t.borrow().get(&s) {
                        Ok(val.clone())
                    } else {
                        Err(SlashError::new(&span, &format!("Entry {} not found in table",&s)))
                    }
                } else {
                    Err(SlashError::new(&span, &format!("Index value not a string, but a {}",index.value_type())))
                }
            }
            _ => Err(SlashError::new(&span, &format!("Trying to index into non-indexable type {}, expected List or Table",self.value_type())))
        }
    }
}

fn bool_to_value(val: bool) -> Value {
    Value::Number(if val { 1.0 } else { 0.0 })
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String(val) => String::from(val),
            _ => self.to_json()
        }
    }
}