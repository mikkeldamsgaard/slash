extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate lazy_static;

mod closure;
mod evaluate;
mod value;
mod function;
mod error;

use pest::{Parser, Span};
use pest::iterators::Pair;
use duct;
use std::ffi::OsString;
use crate::closure::{Closure};
use crate::evaluate::{evaluate, evaluate_env_var};
use std::io::Write;
use crate::function::function_call;
use crate::value::Value;
use std::rc::Rc;
use crate::error::SlashError;
use std::cell::RefCell;
use std::path::Path;
use std::env;

#[derive(Debug, Clone)]
pub enum ExecuteResult<'a> {
    Return(Value, Span<'a>),
    Break(Span<'a>),
    Continue(Span<'a>),
    None,
}

impl ExecuteResult<'_> {
    fn is_none(&self) -> bool {
        if let ExecuteResult::None = self { true } else { false }
    }
}

#[derive(Parser)]
#[grammar = "slash.pest"]
pub struct SlashParser;

pub struct Slash<'a> {
    source: &'a str,
    stdout: Box<RefCell<dyn Write>>,
    stderr: Box<RefCell<dyn Write>>,
    cur_dir: &'a Path,
}

impl Slash<'_> {
    pub fn new<'a>(source: &'a str, stdout: Box<RefCell<dyn Write>>,
                   stderr: Box<RefCell<dyn Write>>, cur_dir: &'a Path) -> Slash<'a> {
        Slash { source, stdout, stderr, cur_dir }
    }

    pub fn run(&self) -> Result<(), SlashError> {
        let mut root = Closure::new();
        let mut pairs = SlashParser::parse(Rule::file, self.source)?;
        self.execute(pairs.next().unwrap(), &mut root)?;
        Ok(())
    }

    fn execute<'a>(&self, pair: Pair<'a, Rule>, closure: &mut Closure) -> Result<ExecuteResult<'a>, SlashError> {
        match pair.as_rule() {
            Rule::file => {
                for p in pair.into_inner() {
                    let res = self.execute(p, closure)?;
                    match res {
                        ExecuteResult::None => {}
                        ExecuteResult::Continue(s) => return Err(SlashError::new(&s, "Unexpected continue")),
                        ExecuteResult::Break(s) => return Err(SlashError::new(&s, "Unexpected break")),
                        ExecuteResult::Return(_, s) => return Err(SlashError::new(&s, "Unexpected return"))
                    }
                }
            }
            Rule::block => {
                let mut inner_closure = closure.derived();
                for p in pair.into_inner() {
                    let res = self.execute(p, &mut inner_closure)?;
                    if !res.is_none() { return Ok(res); }
                }
            }
            Rule::function_call => { function_call(pair, closure, self)?; }
            Rule::var_declaration => {
                let mut pairs = pair.into_inner();
                let var_name = pairs.next().unwrap().as_str();
                let expression = pairs.next().unwrap();
                let value = evaluate(expression, closure, self)?;
                closure.declare(var_name, value);
            }
            Rule::var_assignment => {
                let mut pairs = pair.into_inner();
                let var_pair = pairs.next().unwrap();
                let var_name = var_pair.as_str().trim();
                if closure.has_var(var_name) {
                    let expression = pairs.next().unwrap();
                    let value = evaluate(expression, closure, self)?;
                    closure.assign(var_name, value);
                } else {
                    return Err(SlashError::new(&var_pair.as_span(), &format!("Variable {} not defined.", var_name)));
                }
            }
            Rule::indexed_var_assignment => {
                let mut pairs = pair.into_inner();
                let var_pair = pairs.next().unwrap();
                let var_span = var_pair.as_span();
                let var_name = var_pair.as_str().trim();

                if closure.has_var(var_name) {
                    let expr_pair = pairs.next().unwrap();
                    let expr_span = expr_pair.as_span();
                    let index = evaluate(expr_pair, closure, &self)?;
                    let expression = pairs.next().unwrap();
                    let value = evaluate(expression, closure, self)?;

                    let lhs_val = closure.lookup(var_name);
                    let lhs_val_type = lhs_val.value_type();
                    match &lhs_val {
                        Value::List(l) => {
                            let i_index;
                            if let Value::Number(raw) = index {
                                if 0.0 <= raw && raw < l.borrow().len() as f64 {
                                    i_index = usize::from(raw as u16);
                                    l.borrow_mut()[i_index] = value;
                                } else {
                                    return Err(SlashError::new(&expr_span, &format!("Index out of bounds. Value length is {} index was {}", l.borrow().len(), raw)));
                                }
                            } else {
                                return Err(SlashError::new(&expr_span, &format!("Index value not a number, but a {}", lhs_val_type)));
                            }
                        }
                        Value::Table(t) => {
                            if let Value::String(s) = index {
                                t.borrow_mut().insert(s, value);
                            } else {
                                return Err(SlashError::new(&expr_span, &format!("Index value not a string, but a {}", lhs_val_type)));
                            }
                        }
                        _ => return Err(SlashError::new(&var_span, &format!("Left hand side variables value is not table or list, but {}", lhs_val.value_type())))
                    }
                } else {
                    return Err(SlashError::new(&var_pair.as_span(), &format!("Variable {} not defined.", var_name)));
                }
            }
            Rule::chain => {
                let mut pairs = pair.into_inner();
                let command = pairs.next().unwrap();

                let mut vec = Vec::new();
                let mut redirection = None;
                let mut capture = None;
                loop {
                    match pairs.next() {
                        Some(p) => {
                            match p.as_rule() {
                                Rule::pipe => { vec.push(p.into_inner().next().unwrap()); }
                                Rule::redirection => { redirection = Some(p); }
                                Rule::capture => { capture = Some(p); }
                                _ => {}
                            }
                        }
                        None => break
                    }
                }

                self.run_chain(command, vec, redirection, capture, closure)?;
            }
            Rule::for_in_statement => {
                let mut pairs = pair.into_inner();
                let var_name = pairs.next().unwrap().as_str();
                let expression = pairs.next().unwrap();
                let expression_span = expression.as_span();
                if let Value::List(list) = evaluate(expression, closure, self)? {
                    let block = pairs.next().unwrap();
                    let mut inner_closure = closure.derived();

                    for v in list.borrow().iter() {
                        inner_closure.declare(var_name, v.clone());
                        let mut break_loop = false;
                        for p in block.clone().into_inner() {
                            let res = self.execute(p, &mut inner_closure)?;
                            match res {
                                ExecuteResult::Break(_) => {
                                    break_loop = true;
                                    break;
                                }
                                ExecuteResult::Continue(_) => { break; }
                                ExecuteResult::Return(v, s) => return Ok(ExecuteResult::Return(v, s)),
                                ExecuteResult::None => {}
                            }
                        }

                        if break_loop { break; }
                    }
                } else {
                    return Err(SlashError::new(&expression_span, &format!("Expected list value")));
                }
            }
            Rule::for_std_statement => {
                let mut pairs = pair.into_inner();
                let var_name = pairs.next().unwrap().as_str();
                let init_expression = pairs.next().unwrap();
                let continue_expression = pairs.next().unwrap();
                let update_assignment = pairs.next().unwrap();
                let update_assignment_span = update_assignment.as_span();
                let mut update_assignment_pairs = update_assignment.into_inner();
                let update_var_name = update_assignment_pairs.next().unwrap();
                if var_name != update_var_name.as_str() {
                    return Err(SlashError::new(&update_assignment_span, &format!("Expected update term to update loop variable {}, but it updated variable {}", var_name, update_var_name.as_str())));
                }
                let update_expression = update_assignment_pairs.next().unwrap();
                let block = pairs.next().unwrap();
                let mut inner_closure = closure.derived();
                let loop_value = evaluate(init_expression, &mut inner_closure, self)?;
                inner_closure.declare(var_name, loop_value);
                loop {
                    let val = evaluate(continue_expression.clone(), &mut inner_closure, self)?;
                    if !val.is_true() { break; }
                    let mut break_loop = false;
                    for p in block.clone().into_inner() {
                        let res = self.execute(p, &mut inner_closure)?;
                        match res {
                            ExecuteResult::Break(_) => {
                                break_loop = true;
                                break;
                            }
                            ExecuteResult::Continue(_) => { break; }
                            ExecuteResult::Return(v, s) => return Ok(ExecuteResult::Return(v, s)),
                            ExecuteResult::None => {}
                        }
                    }
                    if break_loop { break; }
                    let loop_value = evaluate(update_expression.clone(), &mut inner_closure, self)?;
                    inner_closure.assign(var_name, loop_value);
                }
            }
            Rule::if_statement => {
                let mut pairs = pair.into_inner();
                loop {
                    match pairs.next() {
                        None => break,
                        Some(p) => {
                            match p.as_rule() {
                                Rule::expression => { // if p branch
                                    let branch = pairs.next().unwrap();
                                    if evaluate(p, closure, self)?.is_true() {
                                        let res = self.execute(branch, closure)?;
                                        if !res.is_none() { return Ok(res); }
                                        break;
                                    }
                                }
                                _ => {
                                    let res = self.execute(p, closure)?;
                                    if !res.is_none() { return Ok(res); }
                                } // else p
                            }
                        }
                    }
                }
            }
            Rule::function_declaration => {
                let mut pairs = pair.into_inner();
                let function_name = pairs.next().unwrap().as_str();
                let mut formal_args = Vec::new();
                loop {
                    let p = pairs.next().unwrap();
                    match p.as_rule() {
                        Rule::var_name => formal_args.push(String::from(p.as_str())),
                        Rule::block => {
                            closure.declare(function_name,
                                            Value::Function(
                                                Rc::new(formal_args),
                                                String::from(p.as_str()),
                                                closure.clone(),
                                            ));
                            break;
                        }
                        _ => unreachable!()
                    }
                }
            }
            Rule::return_statement => {
                let span = pair.as_span();
                let expression = pair.into_inner().next().unwrap();
                let value = evaluate(expression, closure, self)?;
                return Ok(ExecuteResult::Return(value, span));
            }
            Rule::break_statement => { return Ok(ExecuteResult::Break(pair.as_span())); }
            Rule::continue_statement => { return Ok(ExecuteResult::Continue(pair.as_span())); }
            Rule::export_statement => {
                let mut pairs = pair.into_inner();
                let var_pair = pairs.next().unwrap();
                let var_name = var_pair.as_str().trim();
                if let Some(expr_pair) = pairs.next() {
                    let value = evaluate(expr_pair, closure, self)?;
                    closure.declare(var_name, value);
                } else {
                    if !closure.has_var(var_name) {
                        return Err(SlashError::new(&var_pair.as_span(), &format!("Exported variable {} not defined.", var_name)));
                    }
                }
                closure.add_export(var_name);
            }
            Rule::match_statement => {
                let mut pairs = pair.into_inner();
                let match_value = evaluate(pairs.next().unwrap(), closure, self)?;
                loop {
                    match pairs.next() {
                        None => break,
                        Some(match_term) => {
                            let mut mt: Vec<_> = match_term.into_inner().collect();
                            let block = mt.remove(mt.len() - 1);
                            if self.matches(&match_value, mt, closure)? {
                                self.execute(block, closure)?;
                            }
                        }
                    }
                }
            }
            Rule::EOI => {}
            _ => { unreachable!("Rule not handled {:?}", pair.as_rule()) }
        }
        Ok(ExecuteResult::None)
    }

    fn run_chain(&self, command: Pair<Rule>, pipes: Vec<Pair<Rule>>, redirect: Option<Pair<Rule>>, capture: Option<Pair<Rule>>, closure: &mut Closure) -> Result<(), SlashError> {
        let command_span = command.as_span();
        let mut cmd = self.create_cmd(command, closure)?;

        for x in pipes {
            cmd = cmd.pipe(self.create_cmd(x, closure)?);
        }

        cmd = cmd.stdout_capture().stderr_capture();

        if let Some(pair) = redirect {
            // Handle append ">>"
            let out_file = self.parse_prg_or_arg(pair.into_inner().next().unwrap(), closure)?;
            cmd = cmd.stdout_file(std::fs::File::create(&out_file[..]).unwrap());
        }

        let out = cmd.unchecked().run().or_else(|e| { Err(SlashError::new(&command_span, &e.to_string())) })?;

        if let Some(pair) = capture {
            let var_name = pair.into_inner().next().unwrap().as_str();
            closure.declare(var_name, Value::ProcessResult(out.status.code(), String::from_utf8(out.stdout).unwrap(), String::from_utf8(out.stderr).unwrap()));
        } else {
            self.stdout.borrow_mut().write(&out.stdout).expect("Failed to write to stdout");
            self.stderr.borrow_mut().write(&out.stderr).expect("Failed to write to stderr");
        }
        Ok(())
    }

    pub fn write_stdout(&self, msg: &str) {
        self.stdout.borrow_mut().write_fmt(format_args!("{}", msg)).expect("Failed to write to stdout");
    }

    fn create_cmd(&self, ast: Pair<Rule>, closure: &mut Closure) -> Result<duct::Expression, SlashError> {
        //println!("to_cmd ast={:?}", ast);
        let mut pairs = ast.into_inner();
        let program = self.parse_prg_or_arg(pairs.next().unwrap(), closure)?;

        let mut args = Vec::new();
        for r in pairs {
            args.push(self.parse_prg_or_arg(r, closure)?);
        }
        let expr = duct::cmd(program, args.clone().iter().map(|i| Into::<OsString>::into(i)));
        let mut full_env = closure.exports();
        env::vars().for_each(|f| {
            if !full_env.contains_key(&f.0) {
                full_env.insert(f.0, f.1);
            }
        });

        let expr = expr.full_env(full_env);

        Ok(expr)
    }

    fn parse_prg_or_arg(&self, term: Pair<Rule>, closure: &mut Closure) -> Result<String, SlashError> {
        match term.as_rule() {
            Rule::word => Ok(String::from(term.as_str())),
            Rule::string_literal => Ok(Value::convert_parsed_string(term.as_str())),
            _ => {
                let t_str = term.as_str();
                let t_sp = term.as_span();
                let v = match term.as_rule() {
                    Rule::env_var => evaluate_env_var(closure, term)?,
                    Rule::expression => evaluate(term, closure, self)?,
                    _ => unreachable!()
                };
                if let Value::String(str) = v {
                    Ok(str)
                } else {
                    Err(SlashError::new(&t_sp, &format!("Term must evaluate to a string {}", t_str)))
                }
            }
        }
    }

    fn matches(&self, value: &Value, match_expressions: Vec<Pair<Rule>>, closure: &mut Closure) -> Result<bool, SlashError> {
        if match_expressions.is_empty() {
            // If there are no match expressions, it is the catch_all rule "_ => {}"
            return Ok(true);
        }

        for match_expression_pair in match_expressions {
            let mut expressions = match_expression_pair.into_inner();
            let first_expression_pair = expressions.next().unwrap();
            let first_expression_pair_span = first_expression_pair.as_span();
            let first = evaluate(first_expression_pair, closure, self)?;

            if let Some(second) = expressions.next() {
                let second_expression_pair_span = second.as_span();
                let second = evaluate(second, closure, self)?;
                if first._less_than_or_equals(value,&first_expression_pair_span)? &&
                    second._greater_than_or_equals(value, &second_expression_pair_span)? {
                    return Ok(true);
                }
            } else {
                if value._equals(&first, &first_expression_pair_span)? {
                    return Ok(true);
                }
            }
        }

        return Ok(false);
    }
}



