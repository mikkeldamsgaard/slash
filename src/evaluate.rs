use pest::iterators::Pair;
use crate::closure::Closure;
use crate::{Rule, value::Value, Slash};
use lazy_static::lazy_static;
use pest::prec_climber::{Assoc, PrecClimber, Operator};
use crate::function::{FunctionCallResult, Function};
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::SlashError;
use std::cell::RefCell;
use std::env;
use pest::Span;

#[derive(Debug)]
enum EvalResult<'a> {
    Var(String, Span<'a>),
    Val(Value, Span<'a>),
    ArgList(Vec<(Value, Span<'a>)>),
    FieldMap(String, Value, Span<'a>),
    FieldList(Vec<(String, Value)>),
    Slice(Value, Value, Span<'a>)
}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(arg_list_constructor, Left) | Operator::new(slice_constructor, Left),
            Operator::new(map_field_constructor, Left),
            Operator::new(or, Left),
            Operator::new(and, Left),
            Operator::new(equals, Left) | Operator::new(not_equals, Left),
            Operator::new(less_than, Left) | Operator::new(greater_than, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
            Operator::new(infix_dot, Left),
            Operator::new(function_call_indicator, Left) | Operator::new(indexer, Left)
        ])
    };
}

pub fn evaluate_to_value(expression: Pair<Rule>, closure: &mut Closure, slash: &Slash) -> Result<Value, SlashError> {
    use EvalResult::*;
    let expression_span = expression.as_span();
    let res = do_climb(expression, closure, slash);
    match res? {
        Val(v, _) => Ok(v),
        Var(var, _span) => Ok(closure.lookup(&var)),
        _ => Err(SlashError::new(&expression_span, &format!("Syntax error, expected an expression that evaluates to a value")))
    }
}

pub fn evaluate_to_args<'a>(expression: Pair<'a, Rule>, closure: &mut Closure, slash: &Slash<'a>) -> Result<(Vec<Value>, Vec<Span<'a>>), SlashError> {
    use EvalResult::*;
    let expression_span = expression.as_span();
    let res = do_climb(expression, closure, slash);
    match res? {
        Val(v, span) => Ok((vec!(v), vec!(span))),
        Var(var, span) => Ok((vec!(closure.lookup(&var)), vec!(span))),
        ArgList(vals) => {
            let mut values = vec!();
            let mut spans = vec!();
            for (val, span) in vals {
                values.push(val);
                spans.push(span);
            }
            Ok((values, spans))
        }
        _ => Err(SlashError::new(&expression_span, &format!("Syntax error, expected an expression that evaluates to arguments")))
    }
}


fn do_climb<'a>(expression: Pair<'a, Rule>, closure: &mut Closure, slash: &Slash<'a>) -> Result<EvalResult<'a>, SlashError> {
    use EvalResult::*;
    let cl = RefCell::new(closure);
    let expression_span = expression.as_span();
    let infix_expression_span = expression.as_span();
    PREC_CLIMBER.climb(
        expression.into_inner(),
        |pair: Pair<Rule>| {
            let expression_span = expression_span.clone();
            match pair.as_rule() {
                Rule::literal => {
                    let literal = pair.into_inner().next().unwrap();
                    match literal.as_rule() {
                        Rule::numeric_literal => Ok(Val(Value::Number(literal.as_str().parse::<f64>().unwrap()), expression_span)),
                        Rule::string_literal => Ok(Val(Value::string_from_syntax(literal.as_str()), expression_span)),
                        Rule::list_literal => {
                            Ok(Val(Value::List(Rc::new(RefCell::new(
                                match do_climb(literal.into_inner().next().unwrap(), &mut cl.borrow_mut(), slash)? {
                                    Val(v, _) => vec!(v),
                                    Var(var_name, _span) => vec!(cl.borrow().lookup(&var_name)),
                                    ArgList(l) => l.into_iter().map(|(v, _s)| v).collect(),
                                    _ => return Err(SlashError::new(&expression_span, &format!("Expected value or list of values")))
                                }))), expression_span))
                        }
                        Rule::map_literal => {
                            let mut res = HashMap::new();
                            match do_climb(literal.into_inner().next().unwrap(), &mut cl.borrow_mut(), slash)? {
                                FieldMap(key, val, _span) => { res.insert(key, val); }
                                FieldList(v) => for (k, v) in v { res.insert(k, v); },
                                ArgList(v) => if !v.is_empty() {
                                    return Err(SlashError::new(&expression_span, "Expected a field definition"));
                                }
                                _ => return Err(SlashError::new(&expression_span, "Expected a field definition"))
                            }
                            Ok(Val(Value::Table(Rc::new(RefCell::new(res))), expression_span))
                        }
                        _ => unreachable!("{:?}: |{}|", literal.as_rule(), literal.as_str()),
                    }
                }
                Rule::expression => do_climb(pair, &mut cl.borrow_mut(), slash),
                Rule::not_expression => {
                    let expr = evaluate_to_value(pair.into_inner().next().unwrap(), &mut cl.borrow_mut(), slash)?;
                    Ok(Val(Value::Number(if expr.is_true() { 0.0 } else { 1.0 }), expression_span))
                }
                Rule::var_name => Ok(Var(pair.as_str().to_owned(), expression_span)),
                Rule::env_var => Ok(Val(evaluate_env_var(&mut cl.borrow_mut(), pair)?, expression_span)),
                Rule::empty_expression_list => Ok(ArgList(vec!())),
                Rule::anonymous_function => {
                    let children: Vec<_> = pair.into_inner().collect();
                    Ok(Val(Value::Function(Function::User(
                        Rc::new(children[0..children.len() - 1].iter().map(|p| p.as_str().to_owned()).collect()),
                        children[children.len() - 1].as_str().to_owned(),
                        cl.borrow().clone())
                    ), expression_span))
                }
                _ => {
                    unreachable!("{:?}\n{}", pair.as_rule(), SlashError::new(&pair.as_span(), "Unreachable"));
                }
            }
        },
        |lhs: Result<EvalResult, SlashError>, op: Pair<Rule>, rhs: Result<EvalResult, SlashError>| {
            let op_span = op.as_span();
            let infix_expression_span = infix_expression_span.clone();
            match op.as_rule() {
                Rule::function_call_indicator => {
                    let mut args = vec!();
                    let mut spans = vec!();
                    spans.push(op_span.clone());
                    match rhs? {
                        Val(v, span) => {
                            args.push(v);
                            spans.push(span)
                        }
                        ArgList(a) => {
                            for (val, span) in a {
                                args.push(val);
                                spans.push(span)
                            }
                        },
                        Var(var_name, span) => {
                            args.push(cl.borrow().lookup(&var_name));
                            spans.push(span)
                        }
                        _ => return Err(SlashError::new(&infix_expression_span, &format!("Expected value or list of values")))
                    }
                    let lhs = v(lhs, &op_span, &cl.borrow())?;
                    match lhs.invoke(args, spans, &mut cl.borrow_mut(), slash)? {
                        FunctionCallResult::Value(v) => Ok(Val(v, infix_expression_span)),
                        FunctionCallResult::NoValue(_st) => Err(SlashError::new(&op_span, "Expected function to return a value"))
                    }
                }
                Rule::arg_list_constructor => {
                    match rhs? {
                        Val(rhs_val, rhs_span) => {
                            match lhs? {
                                Val(lhs_val, lhs_span) => { Ok(ArgList(vec!((lhs_val, lhs_span), (rhs_val, rhs_span)))) },
                                Var(var_name, lhs_span) => { Ok(ArgList(vec!((cl.borrow().lookup(&var_name), lhs_span), (rhs_val, rhs_span)))) },
                                ArgList(mut v) => {
                                    v.push((rhs_val, rhs_span));
                                    Ok(ArgList(v))
                                }
                                _ => Err(SlashError::new(&op_span, "Expected a value or a list of values on left hand side"))
                            }
                        }
                        FieldMap(key, val, _span) => {
                            match lhs? {
                                FieldMap(lhs_key, lhs_val, _span) => Ok(FieldList(vec!((lhs_key, lhs_val), (key, val)))),
                                FieldList(mut v) => {
                                    v.push((key, val));
                                    Ok(FieldList(v))
                                }
                                _ => Err(SlashError::new(&op_span, "Expected a field or a list of fields on left hand side"))
                            }
                        }
                        _ => Err(SlashError::new(&op_span, "Expected a value or a field on right hand side"))
                    }
                }
                Rule::map_field_constructor => {
                    let lhs = v(lhs, &op_span, &cl.borrow())?;
                    let rhs = v(rhs, &op_span, &cl.borrow())?;
                    Ok(FieldMap(lhs.to_string().to_owned(), rhs, infix_expression_span))
                }
                Rule::slice_constructor => {
                    let lhs = v(lhs, &op_span, &cl.borrow())?;
                    let rhs = v(rhs, &op_span, &cl.borrow())?;
                    Ok(Slice(lhs, rhs, infix_expression_span))
                }
                Rule::infix_dot => {
                    let lhs = v(lhs, &op_span, &cl.borrow())?;
                    let rhs = rhs?;
                    if let EvalResult::Var(var_name, var_span) = rhs {
                        if let Value::Table(val) = lhs {
                            if let Some(field) = val.borrow().get(&var_name) {
                                return Ok(Val(field.clone(), infix_expression_span));
                            }
                        }

                        if cl.borrow().has_var(&var_name) {
                            // TODO: Partial resolved functions, ie len(str) === str.len()

                        }

                        Err(SlashError::new(&var_span, &format!("Identifier {} could not be resolved", &var_name)))
                    } else {
                        Err(SlashError::new(&op_span, "Right hand side of a . operator must be an identifier"))
                    }
                }
                Rule::indexer => {
                    let lhs = v(lhs, &op_span, &cl.borrow());
                    match rhs? {
                        Val(v, _) => Ok(Val(lhs?.lookup_by_index(&v, &op_span)?, infix_expression_span)),
                        Slice(from, to, _) => {
                            Ok(Val(lhs?.slice(&from, &to, &op_span)?, infix_expression_span))
                        }
                        _ => Err(SlashError::new(&infix_expression_span, "Expected slice operator or value"))
                    }
                }
                _ => {
                    let lhs = v(lhs, &op_span, &cl.borrow());
                    let rhs = v(rhs, &op_span, &cl.borrow());
                    match op.as_rule() {
                        Rule::add => Ok(Val(lhs?.add(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::subtract => Ok(Val(lhs?.sub(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::multiply => Ok(Val(lhs?.mul(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::divide => Ok(Val(lhs?.div(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::power => Ok(Val(lhs?.powf(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::or => Ok(Val(lhs?.or(&rhs?), infix_expression_span)),
                        Rule::and => Ok(Val(lhs?.and(&rhs?), infix_expression_span)),
                        Rule::equals => Ok(Val(lhs?.equals(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::not_equals => Ok(Val(lhs?.not_equals(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::greater_than => Ok(Val(lhs?.greater_than(&rhs?, &op_span)?, infix_expression_span)),
                        Rule::less_than => Ok(Val(lhs?.less_than(&rhs?, &op_span)?, infix_expression_span)),

                        _ => unreachable!()
                    }
                }
            }
        },
    )
}

fn v(r: Result<EvalResult, SlashError>, span: &Span, closure: &Closure) -> Result<Value, SlashError> {
    match r? {
        EvalResult::Var(var, _span) => Ok(closure.lookup(&var)),
        EvalResult::Val(value, _span) => Ok(value),
        _ => Err(SlashError::new(span, &format!("Syntax error, expected expression to evaluate to a value")))
    }
}

pub fn evaluate_env_var(closure: &mut Closure, pair: Pair<Rule>) -> Result<Value, SlashError> {
    let var_pair = pair.into_inner().next().unwrap();
    let var_name = var_pair.as_str();
    let span = &var_pair.as_span();
    lookup_variable_or_environment(var_name, closure, span)
}

pub fn lookup_variable_or_environment(var_name: &str, closure: &mut Closure, span: &Span) -> Result<Value, SlashError> {
    if closure.has_var(var_name) {
        Ok(closure.lookup(var_name))
    } else {
        match env::var(var_name) {
            Ok(s) => Ok(Value::String(s)),
            Err(_) => Err(SlashError::new(span, &format!("Environment variable {} not defined", var_name)))
        }
    }
}
