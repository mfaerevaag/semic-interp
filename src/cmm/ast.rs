use std::fmt::{Debug, Formatter, Error};

pub type CFunc = (Option<CType>, CIdent, Vec<CType>, Vec<Box<CDecl>>, Vec<Box<CStmt>>);

pub type CDecl = (CType, Vec<CIdent>);

pub enum CStmt {
    Assign(CIdent, Box<CExpr>),
    Error,
}

pub enum CExpr {
    Number(CNum),
    Ident(CIdent),
    BinOp(Box<CExpr>, COp, Box<CExpr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum COp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Copy, Clone)]
pub enum CType {
    Char,
    Int,
}

pub type CNum = i32;

pub type CIdent = String;

impl Debug for CStmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CStmt::*;
        match *self {
            Assign(ref l, ref r) => write!(fmt, "{:?} = {:?}", l, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for CExpr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CExpr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Ident(ref s) => write!(fmt, "{}", &s),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for COp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::COp::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Debug for CType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CType::*;
        match *self {
            Char => write!(fmt, "char"),
            Int => write!(fmt, "int"),
        }
    }
}