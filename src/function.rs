// Code to handle built in and user function calls

use pest::iterators::Pair;
use crate::value::Value;
use crate::{Rule, Slash, ExecuteResult};
use crate::evaluate::{evaluate_to_value, lookup_variable_or_environment, evaluate_to_args};
use crate::closure::Closure;
use crate::function::FunctionCallResult::NoValue;
use crate::pest::Parser;
use crate::error::SlashError;
use pest::Span;
use std::str::FromStr;
use std::{fs, env, fmt};
use std::ffi::OsStr;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

pub enum FunctionCallResult {
    NoValue(String),
    Value(Value),
}

#[derive(Clone)]
pub struct Builtin {
    name: String,
    function: Rc<dyn Fn(Vec<Value>, Vec<Span>, &mut Closure, &Slash) -> Result<FunctionCallResult, SlashError>>,
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Built In Function")
            .field(&self.name)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(Builtin),
    User(Rc<Vec<String>>, String, Closure),
}

pub fn add_builtin_to_closure(closure: &mut Closure) {
    vec!(
        Builtin {
            name: "print".to_owned(),
            function: Rc::new(|args, _spans, _closure, slash| {
                print(args, slash);
                Ok(NoValue(String::from("print")))
            }),
        },
        Builtin {
            name: "println".to_owned(),
            function: Rc::new(|args, _spans, _closure, slash| {
                print(args, slash);
                slash.write_stdout("\n");
                Ok(NoValue(String::from("println")))
            }),
        },
        Builtin {
            name: "eprint".to_owned(),
            function: Rc::new(|args, _spans, _closure, slash| {
                eprint(args, slash);
                Ok(NoValue(String::from("eprint")))
            }),
        },
        Builtin {
            name: "eprintln".to_owned(),
            function: Rc::new(|args, _spans, _closure, slash| {
                eprint(args, slash);
                slash.write_stderr("\n");
                Ok(NoValue(String::from("eprintln")))
            }),
        },
        Builtin {
            name: "len".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                match &args[0] {
                    Value::List(l) => Ok(FunctionCallResult::Value(Value::Number(l.borrow().len() as f64))),
                    Value::Table(t) => Ok(FunctionCallResult::Value(Value::Number(t.borrow().len() as f64))),
                    Value::String(s) => Ok(FunctionCallResult::Value(Value::Number(s.len() as f64))),
                    _ => Err(invalid_type(&spans[1], &args[0]))
                }
            }),
        },
        Builtin {
            name: "to_str".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::String(args[0].to_string())))
            }),
        },
        Builtin {
            name: "parse_number".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                if let Value::String(s) = &args[0] {
                    match f64::from_str(&s) {
                        Err(_) => Err(SlashError::new(&spans[1], &format!("Parse error for value {}", s))),
                        Ok(f) => Ok(FunctionCallResult::Value(Value::Number(f)))
                    }
                } else {
                    Err(invalid_type(&spans[1], &args[0]))
                }
            }),
        },
        Builtin {
            name: "is_number".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::Number(if let Value::Number(_) = &args[0] { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "is_list".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::Number(if let Value::List(_) = &args[0] { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "is_table".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::Number(if let Value::Table(_) = &args[0] { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "is_string".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::Number(if let Value::String(_) = &args[0] { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "is_process_result".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::Number(if let Value::ProcessResult(_, _, _) = &args[0] { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "is_function".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                Ok(FunctionCallResult::Value(Value::Number(if let Value::Function(_) = &args[0] { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "stdout".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                if let Value::ProcessResult(_, stdout, _) = &args[0] {
                    Ok(FunctionCallResult::Value(Value::String(stdout.clone())))
                } else {
                    Err(invalid_type_with_expected(&spans[1], &args[0], "ProcessResult"))
                }
            }),
        },
        Builtin {
            name: "stderr".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                if let Value::ProcessResult(_, _, stderr) = &args[0] {
                    Ok(FunctionCallResult::Value(Value::String(stderr.clone())))
                } else {
                    Err(invalid_type_with_expected(&spans[1], &args[0], "ProcessResult"))
                }
            }),
        },
        Builtin {
            name: "exit_code".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                if let Value::ProcessResult(exitcode, _, _) = &args[0] {
                    if let Some(e) = exitcode {
                        Ok(FunctionCallResult::Value(Value::Number(*e as f64)))
                    } else {
                        Err(SlashError::new(&spans[1], "Process exited abnormally"))
                    }
                } else {
                    Err(invalid_type_with_expected(&spans[1], &args[0], "ProcessResult"))
                }
            }),
        },
        Builtin {
            name: "exit".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;
                if let Value::Number(n) = &args[0] {
                    std::process::exit(*n as i32)
                } else {
                    Err(invalid_type_with_expected(&spans[1], &args[0], "Number"))
                }
            }),
        },
        Builtin {
            name: "include".to_owned(),
            function: Rc::new(|args, spans, closure, slash| {
                verify_formal_args(&args, &spans, 1)?;
                match &args[0] {
                    Value::String(file) => {
                        let mut file = String::from(file);
                        if !file.starts_with("/") {
                            let cd = slash.include_dir.to_str().ok_or::<Result<&OsStr, SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir during include"))))?;
                            //let cd = dbg!(cd).to_str().ok_or::<Result<&str,SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir during include"))))?;
                            file = cd.to_owned() + "/" + &file;
                        }

                        let src = fs::read_to_string(&file).or(Err(SlashError::new(&spans[1], &format!("Failed to load content of file {}", &file))))?;
                        let mut pairs = crate::SlashParser::parse(Rule::file, &src)?;
                        slash.execute(pairs.next().unwrap(), closure)?;

                        Ok(FunctionCallResult::NoValue(String::from("include")))
                    }
                    _ => Err(invalid_type_with_expected(&spans[1], &args[0], "Number")),
                }
            }),
        },
        Builtin {
            name: "cwd".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 0)?;
                let cwd = env::current_dir().or(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir"))))?;
                let cwd = cwd.to_str().ok_or::<Result<&str, SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir"))))?;
                Ok(FunctionCallResult::Value(Value::String(String::from(cwd))))
            }),
        },
        Builtin {
            name: "split".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 2)?;
                let s = get_string(&args[0], &spans[1])?;
                let p = get_string(&args[1], &spans[2])?;

                let mut res = Vec::new();
                for e in s.split(&p) {
                    res.push(Value::String(e.to_owned()));
                }

                Ok(FunctionCallResult::Value(Value::List(Rc::new(RefCell::new(res)))))
            }),
        },
        Builtin {
            name: "starts_with".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 2)?;
                let s = get_string(&args[0], &spans[1])?;
                let p = get_string(&args[1], &spans[2])?;
                Ok(FunctionCallResult::Value(Value::Number(if s.starts_with(&p) { 1.0 } else { 0.0 })))
            }),
        },
        Builtin {
            name: "join".to_owned(),
            function: Rc::new(|args, spans, _closure, _slash| {
                verify_formal_args(&args, &spans, 2)?;
                let l = get_list(&args[0], &spans[1])?;
                let c = get_string(&args[1], &spans[2])?;
                let mut s_vec = Vec::new();
                for sv in l.borrow().iter() {
                    match sv {
                        Value::String(s) => s_vec.push(s.clone()),
                        _ => return Err(SlashError::new(&spans[1], &format!("Expected a list of strings, but found a {} in the list", sv.value_type())))
                    }
                }

                Ok(FunctionCallResult::Value(Value::String(s_vec.join(&c))))
            }),
        },
        Builtin {
            name: "path_of_script".to_owned(),
            function: Rc::new(|args, spans, _closure, slash| {
                verify_formal_args(&args, &spans, 0)?;
                let include_dir = slash.include_dir.to_str().ok_or::<Result<&OsStr, SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir during include"))))?;
                Ok(FunctionCallResult::Value(Value::String(String::from(include_dir))))
            }),
        },
        Builtin {
            name: "args".to_owned(),
            function: Rc::new(|args, spans, _closure, slash| {
                verify_formal_args(&args, &spans, 0)?;
                Ok(FunctionCallResult::Value(Value::List(Rc::new(RefCell::new(slash.args.iter().map(|s| Value::String(s.clone())).collect())))))
            }),
        },
        Builtin {
            name: "lookup_env_var".to_owned(),
            function: Rc::new(|args, spans, closure, _slash| {
                verify_formal_args(&args, &spans, 1)?;

                let var_name = get_string(&args[0], &spans[1])?;

                let val = lookup_variable_or_environment(&var_name, closure, &spans[1])?;
                Ok(FunctionCallResult::Value(val))
            }),
        },
    ).iter().for_each(|bi| closure.declare(&bi.name, Value::Function(Function::Builtin(bi.clone()))));
}

impl Function {
    pub fn invoke(&self, name: &str, args: Vec<Value>, spans: Vec<Span>, closure: &mut Closure, slash: &Slash) -> Result<FunctionCallResult, SlashError> {
        match self {
            Function::Builtin(b) => (b.function)(args,spans,closure,slash),
            Function::User(formal_args, body, closure) => {
                if args.len() != formal_args.len() {
                    return Err(SlashError::new(&spans[0], &format!("Parameter mismatch for function call {}, expected {} arguments but got {}", name, formal_args.len(), args.len())));
                }

                let mut execution_closure = closure.derived();
                for i in 0..args.len() {
                    execution_closure.declare(&formal_args[i][..], args[i].clone());
                }

                let mut pairs = crate::SlashParser::parse(Rule::block, &body).unwrap();
                let res = slash.execute(pairs.next().unwrap(), &mut execution_closure)?;
                if let ExecuteResult::Return(v, _) = res {
                    Ok(FunctionCallResult::Value(v))
                } else {
                    Ok(FunctionCallResult::NoValue("".to_owned()))
                }
            }
        }
    }
}

pub fn function_call(pair: Pair<Rule>, closure: &mut Closure, slash: &Slash) -> Result<FunctionCallResult, SlashError> {
    let mut pairs = pair.into_inner();
    let function_pair = pairs.next().unwrap();
    let function_span = function_pair.as_span();
    let args_pair = pairs.next().unwrap();
    let function = evaluate_to_value(function_pair, closure, slash)?;
    let (args,mut spans) = evaluate_to_args(args_pair, closure, slash)?;

    let mut func_spans = vec!(function_span);
    func_spans.append(&mut spans);
    function.invoke(args,func_spans,closure, slash)
}

fn format_args(args: Vec<Value>) -> String {
    if args.is_empty() { return "".to_owned()}
    let mut s = String::new();
    args.iter().for_each(|a| s.push_str(&format!(" {}", &a.to_string())));
    return s[1..].to_owned();
}

fn print(args: Vec<Value>, slash: &Slash) {
    slash.write_stdout(&format_args(args));
}

fn eprint(args: Vec<Value>, slash: &Slash) {
    slash.write_stderr(&format_args(args));
}

fn verify_formal_args(args: &Vec<Value>, spans: &Vec<Span>, num: usize) -> Result<(), SlashError> {
    if args.len() != num {
        Err(SlashError::new(&spans[0], &format!("Expected {} arguments, but got {}", num, args.len())))
    } else {
        Ok(())
    }
}

fn invalid_type(span: &Span, value: &Value) -> SlashError {
    SlashError::new(span, &format!("Type {} of value is invalid", value.value_type()))
}

fn invalid_type_with_expected(span: &Span, value: &Value, expected: &str) -> SlashError {
    SlashError::new(span, &format!("Expected value to be of type {} but it was of type {}", expected, value.value_type()))
}

fn get_string(arg: &Value, span: &Span) -> Result<String, SlashError> {
    match arg {
        Value::String(s) => Ok(s.clone()),
        _ => Err(invalid_type_with_expected(span, arg, "String"))
    }
}

fn get_list(arg: &Value, span: &Span) -> Result<Rc<RefCell<Vec<Value>>>, SlashError> {
    match arg {
        Value::List(l) => Ok(l.clone()),
        _ => Err(invalid_type_with_expected(span, arg, "List"))
    }
}

