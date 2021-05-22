// Code to handle built in and user function calls

use pest::iterators::Pair;
use crate::value::Value;
use crate::{Rule, Slash, ExecuteResult};
use crate::evaluate::{evaluate, lookup_variable_or_environment};
use crate::closure::Closure;
use crate::function::FunctionCallResult::NoValue;
use crate::pest::Parser;
use crate::error::SlashError;
use pest::Span;
use std::str::FromStr;
use std::{fs, env};
use std::ffi::OsStr;
use std::rc::Rc;
use std::cell::RefCell;

pub enum FunctionCallResult {
    NoValue(String),
    Value(Value),
}

pub fn function_call(pair: Pair<Rule>, closure: &mut Closure, slash: &Slash) -> Result<FunctionCallResult, SlashError> {
    let mut pairs = pair.into_inner();
    let function_pair = pairs.next().unwrap();
    let function = function_pair.as_str().trim();
    let mut args = Vec::new();
    let mut spans = Vec::new();
    spans.push(function_pair.as_span());
    for p in pairs {
        spans.push(p.as_span());
        args.push(evaluate(p, closure, slash)?);
    }

    match function {
        "print" => { print(args, slash); }
        "println" => {
            print(args, slash);
            slash.write_stdout("\n");
        }
        "len" => { return len(args, spans); }
        "to_str" => { return to_str(args, spans); }
        "parse_float" => { return parse_float(args, spans); }
        "is_float" => { return is_float(args, spans); }
        "is_list" => { return is_list(args, spans); }
        "is_table" => { return is_table(args, spans); }
        "is_string" => { return is_string(args, spans); }
        "is_process_result" => { return is_process_result(args, spans); }
        "stdout" => { return stdout(args, spans); }
        "stderr" => { return stderr(args, spans); }
        "exit_code" => { return exit_code(args, spans); }
        "exit" => { return exit(args, spans); }
        "include" => { include(args, spans, closure, slash)?; }
        "cwd" => { return cwd(args,spans); }
        "split" => { return split(args,spans); }
        "starts_with" => { return starts_with(args,spans); }
        "join" => { return join(args,spans); }
        "path_of_script" => { return path_of_script(args,spans,slash); }
        "args" => { return script_args(args,spans,slash); }
        "lookup_env_var" => { return lookup_env_var(args,spans,closure); }

        _ => {
            if !closure.has_var(function) {
                return Err(SlashError::new(&function_pair.as_span(), &format!("Function not found {}", function)));
            }
            if let Value::Function(formal_args, body, closure) = closure.lookup(function) {
                if args.len() != formal_args.len() {
                    return Err(SlashError::new(&function_pair.as_span(), &format!("Parameter mismatch for function call {}, expected {} arguments but got {}", function, formal_args.len(), args.len())));
                }

                let mut execution_closure = closure.derived();
                for i in 0..args.len() {
                    execution_closure.declare(&formal_args[i][..], args[i].clone());
                }

                let mut pairs = crate::SlashParser::parse(Rule::block, &body).unwrap();
                let res = slash.execute(pairs.next().unwrap(), &mut execution_closure)?;
                if let ExecuteResult::Return(v, _) = res {
                    return Ok(FunctionCallResult::Value(v));
                }
            } else {
                return Err(SlashError::new(&function_pair.as_span(), &format!("Variable {} used as function", function)));
            }
        }
    }

    Ok(NoValue(String::from(function)))
}

fn print(args: Vec<Value>, slash: &Slash) {
    let mut s = String::new();
    args.iter().for_each(|a| s.push_str(&format!(" {}", &a.to_string())));
    slash.write_stdout(&s[1..]);
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

fn len(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    match &args[0] {
        Value::List(l) => Ok(FunctionCallResult::Value(Value::Number(l.borrow().len() as f64))),
        Value::Table(t) => Ok(FunctionCallResult::Value(Value::Number(t.borrow().len() as f64))),
        Value::String(s) => Ok(FunctionCallResult::Value(Value::Number(s.len() as f64))),
        _ => Err(invalid_type(&spans[1], &args[0]))
    }
}

fn to_str(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    Ok(FunctionCallResult::Value(Value::String(args[0].to_string())))
}

fn parse_float(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::String(s) = &args[0] {
        match f64::from_str(&s) {
            Err(_) => Err(SlashError::new(&spans[1], &format!("Parse error for value {}", s))),
            Ok(f) => Ok(FunctionCallResult::Value(Value::Number(f)))
        }
    } else {
        Err(invalid_type(&spans[1], &args[0]))
    }
}

fn is_float(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::Number(_) = &args[0] {
        Ok(FunctionCallResult::Value(Value::Number(1.0)))
    } else {
        Ok(FunctionCallResult::Value(Value::Number(0.0)))
    }
}

fn is_string(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::String(_) = &args[0] {
        Ok(FunctionCallResult::Value(Value::Number(1.0)))
    } else {
        Ok(FunctionCallResult::Value(Value::Number(0.0)))
    }
}

fn is_list(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::List(_) = &args[0] {
        Ok(FunctionCallResult::Value(Value::Number(1.0)))
    } else {
        Ok(FunctionCallResult::Value(Value::Number(0.0)))
    }
}


fn is_table(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::Table(_) = &args[0] {
        Ok(FunctionCallResult::Value(Value::Number(1.0)))
    } else {
        Ok(FunctionCallResult::Value(Value::Number(0.0)))
    }
}

fn is_process_result(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::ProcessResult(_, _, _) = &args[0] {
        Ok(FunctionCallResult::Value(Value::Number(1.0)))
    } else {
        Ok(FunctionCallResult::Value(Value::Number(0.0)))
    }
}

fn stdout(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::ProcessResult(_, stdout, _) = &args[0] {
        Ok(FunctionCallResult::Value(Value::String(stdout.clone())))
    } else {
        Err(invalid_type_with_expected(&spans[1], &args[0], "ProcessResult"))
    }
}

fn stderr(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::ProcessResult(_, _, stderr) = &args[0] {
        Ok(FunctionCallResult::Value(Value::String(stderr.clone())))
    } else {
        Err(invalid_type_with_expected(&spans[1], &args[0], "ProcessResult"))
    }
}

fn exit_code(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
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
}

fn exit(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    if let Value::Number(n) = &args[0] {
        std::process::exit(*n as i32)
    } else {
        Err(invalid_type_with_expected(&spans[1], &args[0], "Number"))
    }
}

fn include(args: Vec<Value>, spans: Vec<Span>, closure: &mut Closure, slash: &Slash) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;
    match &args[0] {
        Value::String(file) => {
            let mut file = String::from(file);
            if !file.starts_with("/") {
                let cd = slash.include_dir.to_str().ok_or::<Result<&OsStr,SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir during include"))))?;
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
}

fn cwd(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 0)?;
    let cwd = env::current_dir().or(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir"))))?;
    let cwd = cwd.to_str().ok_or::<Result<&str,SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir"))))?;
    Ok(FunctionCallResult::Value(Value::String(String::from(cwd))))
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

fn split(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 2)?;
    let s = get_string(&args[0], &spans[1])?;
    let p = get_string(&args[1], &spans[2])?;

    let mut res = Vec::new();
    for e in s.split(&p) {
        res.push(Value::String(e.to_owned()));
    }

    Ok(FunctionCallResult::Value(Value::List(Rc::new(RefCell::new(res)))))
}

fn starts_with(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 2)?;
    let s = get_string(&args[0], &spans[1])?;
    let p = get_string(&args[1], &spans[2])?;
    Ok(FunctionCallResult::Value(Value::Number(if s.starts_with(&p) {1.0} else {0.0})))
}

fn join(args: Vec<Value>, spans: Vec<Span>) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 2)?;
    let l = get_list(&args[0], &spans[1])?;
    let c = get_string(&args[1], &spans[2])?;
    let mut s_vec = Vec::new();
    for sv in l.borrow().iter() {
        match sv {
            Value::String(s) => s_vec.push(s.clone()),
            _=> return Err(SlashError::new(&spans[1], &format!("Expected a list of strings, but found a {} in the list", sv.value_type())))
        }
    }

    Ok(FunctionCallResult::Value(Value::String(s_vec.join(&c))))
}

fn path_of_script(args: Vec<Value>, spans: Vec<Span>, slash: &Slash) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 0)?;
    let include_dir = slash.include_dir.to_str().ok_or::<Result<&OsStr,SlashError>>(Err(SlashError::new(&spans[0], &format!("Could not retrieve current dir during include"))))?;
    Ok(FunctionCallResult::Value(Value::String(String::from(include_dir))))
}

fn script_args(args: Vec<Value>, spans: Vec<Span>, slash: &Slash) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 0)?;
    Ok(FunctionCallResult::Value(Value::List(Rc::new(RefCell::new(slash.args.iter().map(|s| Value::String(s.clone())).collect())))))
}

fn lookup_env_var(args: Vec<Value>, spans: Vec<Span>, closure: &mut Closure) -> Result<FunctionCallResult, SlashError> {
    verify_formal_args(&args, &spans, 1)?;

    let var_name = get_string(&args[0], &spans[1])?;

    let val= lookup_variable_or_environment(&var_name, closure, &spans[1])?;
    Ok(FunctionCallResult::Value(val))
}
