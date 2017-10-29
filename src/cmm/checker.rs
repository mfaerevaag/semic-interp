use std::error::Error;
use std::fmt;

use ast::*;
use env::{FuncTab, SymTab};

// error type

#[derive(Debug)]
pub struct CheckErr {
    message: String,
}

impl CheckErr {
    pub fn new (message: String) -> CheckErr {
        CheckErr { message }
    }
}

impl Error for CheckErr {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CheckErr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", &self.message)
    }
}

// checker functions

pub fn check_prog<'input, 'err>(
    errors: &'err mut Vec<CheckErr>,
    ast: &'input CProg
) -> Result<(), ()>
{
    // global function table
    let mut funcs = FuncTab::new();
    funcs.push_frame();
    // symbol table
    let mut syms = SymTab::new();
    syms.push_frame();

    // check each element
    for elem in ast.iter() {
        match *elem {
            CProgElem::VarDecl(ref decl) => {
                let (_, ref name, _) = *decl;
                let val = (decl, None);

                match syms.insert(*name, val) {
                    Ok(Some(_)) => {
                        let msg = format!("variable {:?} already declared", name);
                        errors.push(CheckErr::new(msg));
                    },
                    Ok(None) => (),
                    Err(_) => panic!("symbol table empty"),
                };
            },

            CProgElem::Func(ref func) => {
                let CFunc { ref proto, .. } = *func;
                let CProto { ref name, .. } = *proto;
                let val = (proto, Some(func));

                match funcs.insert(*name, val) {
                    Ok(Some(x)) => match x {
                        (_, None) => (),
                        (_, Some(_)) => {
                            let msg = format!("function {:?} already declared", name);
                            errors.push(CheckErr::new(msg));
                        },
                    },
                    Ok(None) => (),
                    Err(_) => panic!("function table empty"),
                };
            },

            CProgElem::Proto(ref proto) => {
                let CProto { ref name, .. } = *proto;
                let val = (proto, None);

                match funcs.insert(*name, val) {
                    Ok(Some(_)) => {
                        let msg = format!("function {:?} already defined", name);
                        errors.push(CheckErr::new(msg));
                    },
                    Ok(None) => (),
                    Err(_) => panic!("function table empty"),
                };
            },

            CProgElem::Error => (),
        };
    };

    match errors.len() {
        0 => Ok(()),
        _ => Err(()),
    }
}