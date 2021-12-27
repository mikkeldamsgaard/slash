use std::collections::HashMap;
use crate::value::Value;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ClosureData {
    variables: HashMap<String, Value>,
    exports: Vec<String>,
    parent: Option<Rc<RefCell<ClosureData>>>
}

#[derive(Debug,Clone)]
pub struct Closure(Rc<RefCell<ClosureData>>);

impl Closure {
    fn c(parent: Option<Rc<RefCell<ClosureData>>>) -> Closure {
        Closure(Rc::new(RefCell::new(
            ClosureData {
                parent,
                variables: HashMap::new(),
                exports: Vec::new()
            })))
    }

    pub fn new() -> Closure { Closure::c(None) }
    pub fn derived(&self) -> Closure { Closure::c(Some(self.0.clone())) }

    fn find_closure(&self, var_name: &str) -> Option<Rc<RefCell<ClosureData>>> {
        Closure::i_find_closure(&self.0, var_name)
    }

    fn i_find_closure(data: &Rc<RefCell<ClosureData>>, var_name: &str) -> Option<Rc<RefCell<ClosureData>>> {
        let cl_data = data.borrow();
        if cl_data.variables.contains_key(var_name) {
            Some(data.clone())
        } else if cl_data.parent.is_none() {
            None
        } else {
            Closure::i_find_closure(&cl_data.parent.clone().unwrap(), var_name)
        }
    }

    pub fn declare(&self, var_name: &str, value: Value) {
        self.0.borrow_mut().variables.insert(String::from(var_name), value);
    }

    pub fn assign(&mut self, var_name: &str, value: Value) {
        if let Some(closure) = self.find_closure(var_name) {
            closure.borrow_mut().variables.insert(String::from(var_name), value);
        } else {
            panic!("Variable {} not defined", var_name)
        }
    }

    pub fn has_var(&self, var_name: &str) -> bool {
        return self.find_closure(var_name).is_some();
    }

    pub fn lookup(&self, var_name: &str) -> Value {
        Closure::i_lookup(&self.0, var_name)
    }

    fn i_lookup(data: &Rc<RefCell<ClosureData>>, var_name: &str) -> Value {
        if let Some(closure) = Closure::i_find_closure(data, var_name) {
            return closure.borrow().variables.get(var_name).unwrap().clone();
        } else {
            // Check for built-in functions

            panic!("Variable {} not defined", var_name)
        }
    }

    pub fn add_export(&self, var_name: &str) {
        self.0.borrow_mut().exports.push(var_name.to_owned())
    }

    pub fn exports(&self) -> HashMap<String,String> {
        let mut res =  HashMap::new();
        Closure::i_export(&self.0,&mut res);
        res
    }

    fn i_export(data: &Rc<RefCell<ClosureData>>, res: &mut HashMap<String,String>) {
        let _ = &data.borrow().exports.iter().for_each(|v| {res.insert(v.clone(), Closure::i_lookup(data,&v).to_string());});
        if let Some(p) = &data.borrow().parent {
            Closure::i_export(p, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::closure::Closure;

    #[test]
    fn ref_test() {
        let ch = Closure::new();
        ch.derived();
        ch.derived();

        //i1.has_var("a");
    }
}