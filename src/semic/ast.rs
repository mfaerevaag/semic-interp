use std::str::Chars;
use std::fmt::{Debug, Formatter, Error};


pub type CProg<'input> = Vec<CProgElem<'input>>;

#[derive(Clone)]
pub enum CProgElem<'input> {
    Decl(CLoc, CType, CIdent<'input>, Option<CExpr<'input>>),
    Proto(CLoc, CProto<'input>),
    Func(CLoc, CFunc<'input>),
    Error,
}

#[derive(Clone, Debug)]
pub struct CProto<'input> {
    pub ret: Option<CType>,
    pub name: CIdent<'input>,
    pub params: Vec<(CType, CIdent<'input>)>,
}

#[derive(Clone, Debug)]
pub struct CFunc<'input> {
    pub proto: CProto<'input>,
    pub body: CStmt<'input>,
}

#[derive(Clone)]
pub enum CStmt<'input> {
    Decl(CLoc, CType, CIdent<'input>, Option<CExpr<'input>>),
    Assign(CLoc, CIdent<'input>, Option<CExpr<'input>>, CExpr<'input>),
    Call(CLoc, CIdent<'input>, Vec<Box<CExpr<'input>>>),
    Return(CLoc, Option<CExpr<'input>>),
    Block(CLoc, Vec<Box<CStmt<'input>>>),
    If(CLoc, CExpr<'input>, Box<CStmt<'input>>, Option<Box<CStmt<'input>>>),
    While(CLoc, CExpr<'input>, Box<CStmt<'input>>),
    Print(CLoc, Option<CString<'input>>, CExpr<'input>),
    Error,
}

#[derive(Clone)]
pub enum CExpr<'input> {
    Int(CLoc, CInt),
    Float(CLoc, CFloat),
    Str(CLoc, CString<'input>),
    Char(CLoc, CChar),
    Ident(CLoc, CIdent<'input>),
    UnOp(CLoc, COp, Box<CExpr<'input>>),
    BinOp(CLoc, COp, Box<CExpr<'input>>, Box<CExpr<'input>>),
    Call(CLoc, CIdent<'input>, Vec<Box<CExpr<'input>>>),
    Index(CLoc, CIdent<'input>, Box<CExpr<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum COp {
    // arith
    Mul,
    Div,
    Add,
    Sub,
    // rel
    Neq,
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    // logical
    And,
    Or,
    // unary
    Neg,
    Not,
}

#[derive(PartialEq, Clone)]
pub enum CType {
    Int,
    Char,
    Float,
    Ref(Box<CType>),
}

pub type CLoc = (usize, usize);

pub type CInt = i32;
pub type CFloat = f32;
pub type CString<'input> = Chars<'input>;
pub type CChar = char;

pub type CIdent<'input> = &'input str;


// debug trait

impl<'input> Debug for CProgElem<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CProgElem::*;
        match *self {
            Decl(_, ref t, id, ref eo) => match eo {
                &Some(ref e) => write!(fmt, "{:?} {}[{:?}]", t, id, e),
                &None => write!(fmt, "{:?} {}", t, id),
            },
            Proto(_, ref x) => write!(fmt, "{:?}", x),
            Func(_, ref x) => write!(fmt, "{:#?}", x),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for CStmt<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CStmt::*;
        match *self {
            Decl(_, ref t, id, ref eo) => match *eo {
                Some(ref e) => write!(fmt, "{:?} {}[{:?}]", t, id, e),
                None => write!(fmt, "{:?} {}", t, id),
            },
            Assign(_, ref l, ref eo, ref r) => match *eo {
                Some(ref e) => write!(fmt, "{}[{:?}] = {:?}", l, e, r),
                None => write!(fmt, "{} = {:?}", l, r),
            },
            Call(_, ref i, ref p) => {
                let mut s: String = String::new();
                for (i, e) in p.iter().enumerate() {
                    if i > 0 { s.push_str(", ") }
                    s.push_str(&format!("{:?}", e));
                }
                write!(fmt, "{}({})", i, s)
            },
            Return(_, ref o) => {
                match *o {
                    Some(ref e) => write!(fmt, "return {:?}", e),
                    None => write!(fmt, "return"),
                }
            }
            Block(_, ref stmts) => write!(fmt, "{:#?}", stmts),
            If(_, ref cond, ref stmt, ref opt) => match opt.clone() {
                Some(ref stmt2) => write!(fmt, "if {:?} {:?} else {:?}", cond, stmt, stmt2),
                None => write!(fmt, "if {:?} {:?}", cond, stmt),
            },
            While(_, ref cond, ref stmt) =>
                write!(fmt, "while {:?} {:?}", cond, stmt),
            Print(_, ref fmto, ref e) => match *fmto {
                Some(ref s) => write!(fmt, "printf({:?}, {:?})", s, e),
                None => write!(fmt, "printf({:?})", e),
            },
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for CExpr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CExpr::*;
        match *self {
            Int(_, i) => write!(fmt, "{:?}", i),
            Float(_, f) => write!(fmt, "{:.2}", f),
            Str(_, ref s) => write!(fmt, "\"{}\"", s.as_str()),
            Char(_, c) => write!(fmt, "{:?}", c),
            Ident(_, ref s) => write!(fmt, "{}", &s),
            UnOp(_, op, ref l) => write!(fmt, "({:?}{:?})", op, l),
            BinOp(_, op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Call(_, ref i, ref p) => {
                let mut s: String = String::new();
                for (i, e) in p.iter().enumerate() {
                    if i > 0 { s.push_str(", ") }
                    s.push_str(&format!("{:?}", e));
                }
                write!(fmt, "{}({})", i, s)
            },
            Index(_, ref i, ref e) => {
                write!(fmt, "{}[{:?}]", i, e)
            },
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
            Eq  => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            Lt  => write!(fmt, "<"),
            Lte => write!(fmt, "<="),
            Gt  => write!(fmt, ">"),
            Gte => write!(fmt, ">="),
            And => write!(fmt, "&&"),
            Or  => write!(fmt, "||"),
            Neg => write!(fmt, "-"),
            Not => write!(fmt, "!"),
        }
    }
}

impl Debug for CType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CType::*;
        match *self {
            Char => write!(fmt, "char"),
            Int => write!(fmt, "int"),
            Float => write!(fmt, "float"),
            Ref(ref t) => write!(fmt, "{:?}*", t),
        }
    }
}
