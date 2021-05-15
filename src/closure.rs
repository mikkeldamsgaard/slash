use std::collections::HashMap;
use crate::value::Value;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ClosureData {
    variables: HashMap<String, Value>,
    parent: Option<Rc<RefCell<ClosureData>>>
}

#[derive(Debug,Clone)]
pub struct Closure(Rc<RefCell<ClosureData>>);

impl Closure {
    fn c(parent: Option<Rc<RefCell<ClosureData>>>) -> Closure { Closure(Rc::new(RefCell::new(ClosureData {parent,variables: HashMap::new()}))) }
    pub fn new() -> Closure { Closure::c(None) }
    pub fn derived(&self) -> Closure { Closure::c(Some(self.0.clone())) }

    fn find_closure(&mut self, var_name: &str) -> Option<Rc<RefCell<ClosureData>>> {
        Closure::fc(&self.0, var_name)
    }

    fn fc(data: &Rc<RefCell<ClosureData>>, var_name: &str) -> Option<Rc<RefCell<ClosureData>>> {
        let cl_data = data.borrow();
        if cl_data.variables.contains_key(var_name) {
            Some(data.clone())
        } else if cl_data.parent.is_none() {
            None
        } else {
            Closure::fc(&cl_data.parent.clone().unwrap(), var_name)
        }
    }

    pub fn declare(&mut self, var_name: &str, value: Value) {
        self.0.borrow_mut().variables.insert(String::from(var_name), value);
    }

    pub fn assign(&mut self, var_name: &str, value: Value) {
        if let Some(closure) = self.find_closure(var_name) {
            closure.borrow_mut().variables.insert(String::from(var_name), value);
        } else {
            panic!("Variable {} not defined", var_name)
        }
    }

    pub fn has_var(&mut self, var_name: &str) -> bool {
        return self.find_closure(var_name).is_some();
    }

    pub fn lookup(&mut self, var_name: &str) -> Value {
        if let Some(closure) = self.find_closure(var_name) {
            return closure.borrow().variables.get(var_name).unwrap().clone();
        } else {
            panic!("Variable {} not defined", var_name)
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::closure::Closure;

    #[test]
    fn ref_test() {
        let mut ch = Closure::new();
        ch.derived();
        ch.derived();

        //i1.has_var("a");
    }
}