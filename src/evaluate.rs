use pest::iterators::Pair;
use crate::closure::Closure;
use crate::{Rule, value::Value, Slash, function};
use lazy_static::lazy_static;
use pest::prec_climber::{Assoc, PrecClimber, Operator};
use crate::function::FunctionCallResult;
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::SlashError;
use std::cell::RefCell;
use std::env;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrecClimber::new(vec![
            Operator::new(or, Left),
            Operator::new(and, Left),
            Operator::new(equals, Left) | Operator::new(not_equals, Left),
            Operator::new(less_than, Left) | Operator::new(greater_than, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right) |
            Operator::new(indexer, Left)
        ])
    };
}

pub fn evaluate(expression: Pair<Rule>, closure: &mut Closure, slash: &Slash) -> Result<Value, SlashError> {

    //dbg!(&expression);
    PREC_CLIMBER.climb(
        expression.into_inner(),
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::literal => {
                let literal = pair.into_inner().next().unwrap();
                match literal.as_rule() {
                    Rule::numeric_literal => Ok(Value::Number(literal.as_str().parse::<f64>().unwrap())),
                    Rule::string_literal => Ok(Value::string_from_syntax(literal.as_str())),
                    Rule::list_literal => {
                        let mut res = Vec::new();
                        for p in literal.into_inner() {
                            res.push(evaluate(p,closure,slash)?)
                        }
                        Ok(Value::List(Rc::new(RefCell::new(res))))
                    },
                    Rule::map_literal => {
                        let mut res = HashMap::new();
                        let mut pairs = literal.into_inner();
                        loop {
                            if let Some(pair) = pairs.next() {
                                match pair.as_rule() {
                                    Rule::var_name => {
                                        res.insert(String::from(pair.as_str().trim()), evaluate(pairs.next().unwrap(), closure,slash)?);
                                    },
                                    Rule::string_literal => {
                                        res.insert(Value::convert_parsed_string(pair.as_str()), evaluate(pairs.next().unwrap(),closure,slash)?);
                                    },
                                    _ => unreachable!()
                                }
                            } else {
                                break;
                            }
                        }
                        Ok(Value::Table(Rc::new(RefCell::new(res))))
                    }
                    _ => unreachable!("{:?}: |{}|", literal.as_rule(),literal.as_str()),
                }

            },
            Rule::expression => evaluate(pair, closure, slash),
            Rule::var_name => Ok(closure.lookup(pair.as_str())),
            Rule::env_var => {
                evaluate_env_var(closure, pair)
            }
            Rule::function_call => {
                let span = pair.as_span();
                match function::function_call(pair, closure, slash)? {
                    FunctionCallResult::NoValue(function_name) => Err(SlashError::new(&span,&format!("Function {} used in expression, but does not return a result", function_name))),
                    FunctionCallResult::Value(value) => Ok(value)
                }
            }
            _ => {
                unreachable!("{:?}\n{}",pair.as_rule(), SlashError::new(&pair.as_span(), "Unreachable"));
            },
        },
        |lhs: Result<Value, SlashError>, op: Pair<Rule>, rhs: Result<Value, SlashError> | {
            let s = op.as_span();
            match op.as_rule() {
                Rule::add => Ok(lhs?.add(rhs?, s)?),
                Rule::subtract => Ok(lhs?.sub(rhs?,s)?),
                Rule::multiply => Ok(lhs?.mul(rhs?,s)?),
                Rule::divide => Ok(lhs?.div(rhs?,s)?),
                Rule::power => Ok(lhs?.powf(rhs?,s)?),
                Rule::or => Ok(lhs?.or(rhs?)),
                Rule::and => Ok(lhs?.and(rhs?)),
                Rule::equals => Ok(lhs?.equals(rhs?,s)?),
                Rule::not_equals => Ok(lhs?.not_equals(rhs?,s)?),
                Rule::greater_than => Ok(lhs?.greater_than(rhs?,s)?),
                Rule::less_than => Ok(lhs?.less_than(rhs?,s)?),
                Rule::indexer => Ok(lhs?.lookup_by_index(rhs?, s)?),
                _ => unreachable!(),
            }
        },
    )
}

pub fn evaluate_env_var(closure: &mut Closure, pair: Pair<Rule>) -> Result<Value, SlashError> {
    let var_pair = pair.into_inner().next().unwrap();
    let var_name = var_pair.as_str();
    if closure.has_var(var_name) {
        Ok(closure.lookup(var_name))
    } else {
        match env::var(var_name) {
            Ok(s) => Ok(Value::String(s)),
            Err(_) => Err(SlashError::new(&var_pair.as_span(), &format!("Environment variable {} not defined", var_name)))
        }
    }
}
