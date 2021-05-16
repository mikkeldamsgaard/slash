use pest::Span;
use pest::error::{Error, LineColLocation};
use crate::Rule;
use std::fmt::{Display, Formatter};

pub struct SlashError {
    err: String,
    error_line: String,
    line: usize,
    column: usize,
    parse: bool
}

impl SlashError {
    pub fn new(span: &Span, err: &str) -> SlashError {
        let (line,column) = span.start_pos().line_col();
        SlashError { err: String::from(err), line, column, error_line: String::from(span.start_pos().line_of()), parse: false}
    }
}

impl From<Error<Rule>> for SlashError {
    fn from(e: Error<Rule>) -> Self {
        let line: usize;
        let column: usize;
        match e.line_col {
            LineColLocation::Pos((l,c)) => { line =l; column = c},
            LineColLocation::Span((l,c),_) => { line =l; column = c}
        }
        SlashError { err: e.to_string(), line, column, error_line: String::from(""), parse: true }
    }
}
impl<T> From<std::result::Result<T, SlashError>> for SlashError  {
    fn from(e: Result<T, SlashError>) -> Self {
        if let Err(e) = e {
            e
        } else { panic!("Internal error");}
    }
}
// impl ToString for SlashError {
//     fn to_string(&self) -> String {
//         if self.parse { return self.err.to_owned() };
//
//         return format!("{}\nAt line {} column {}:\n===>   {}", self.err, self.line, self.column, self.error_line) // TODO: Add a ^ marker on a new line
//     }
// }

impl Display for SlashError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),std::fmt::Error> {
        if self.parse {
            f.write_str(&self.err)?;
        } else {
            f.write_str(&format!("{}\nAt line {} column {}:\n===>   {}", self.err, self.line, self.column, self.error_line))?; // TODO: Add a ^ marker on a new line
        }
        Ok(())
    }
}