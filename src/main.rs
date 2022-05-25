#![feature(map_try_insert)]

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

mod ast;

use ast::*;

pub struct System {
    mappings: HashMap<String, Val>,
}
impl System {
    pub fn new() -> Self {
        Self { mappings: HashMap::new() }
    }

    pub fn eval(&mut self, expr: &Expr) -> anyhow::Result<Val> {
        use Expr::*;
        match expr {
            Atom(val) => Ok(val.clone()),
            Sym(sym) => self.get(sym.clone()),
            _ => todo!()
        }
    }

    pub fn set(&mut self, sym: String, val: Val) -> anyhow::Result<()> {
        match self.mappings.try_insert(sym.clone(), val) {
            Ok(_) => Ok(()),
            Err(_) => Err(anyhow::Error::msg(format!("cannot reassign name `{}`", sym)))
        }
    }

    pub fn get(&self, sym: String) -> anyhow::Result<Val> {
        match self.mappings.get(&sym) {
            Some(val) => Ok(val.clone()),
            None => Err(anyhow::Error::msg(format!("name `{}` is undefined", sym)))
        }
    }
}



fn main() {
    println!("Hello, world!");
}

pub fn err_to_string<T: std::error::Error>(err: T) -> String {
    format!("{:?}", err)
}

#[cfg(test)]
mod tests {
    use super::*;

    use Expr::*;
    use Val::*;

    #[test]
    fn test_eval_atom() {
        let input = vec![Atom(Int(10)), Atom(Float(0.0)), Atom(Str(String::from("test")))];
        let mut sys = System::new();
        let result = input
            .iter()
            .map(|val| sys.eval(val).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![Int(10), Float(0.0), Str(String::from("test"))];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_eval_sym() {
        let input = vec![Sym(String::from("ldaslidhis")), Sym(String::from("hdlhahdhiualid"))];
        let mut sys = System::new();
        sys.set(String::from("ldaslidhis"), Int(87973003)).unwrap();
        sys.set(String::from("hdlhahdhiualid"), Str(String::from("hhidy98y"))).unwrap();

        let result = input
            .iter()
            .map(|val| sys.eval(val).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![Int(87973003), Str(String::from("hhidy98y"))];
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_eval_sym_undefined() {
        let input = Sym(String::from("aldsdhasdj"));
        let mut sys = System::new();
        sys.eval(&input).unwrap();
    }
}
