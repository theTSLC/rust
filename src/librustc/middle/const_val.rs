// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax::symbol::InternedString;
use syntax::ast;
use std::rc::Rc;
use hir::def_id::DefId;
use ty::subst::Substs;
use rustc_const_math::*;

use self::ConstVal::*;
pub use rustc_const_math::ConstInt;

use std::collections::BTreeMap;

#[derive(Clone, Debug, Hash, RustcEncodable, RustcDecodable, Eq, PartialEq)]
pub enum ConstVal<'tcx> {
    Float(ConstFloat),
    Integral(ConstInt),
    Str(InternedString),
    ByteStr(Rc<Vec<u8>>),
    Bool(bool),
    Function(DefId, &'tcx Substs<'tcx>),
    Struct(BTreeMap<ast::Name, ConstVal<'tcx>>),
    Tuple(Vec<ConstVal<'tcx>>),
    Array(Vec<ConstVal<'tcx>>),
    Repeat(Box<ConstVal<'tcx>>, u64),
    Char(char),
}

impl<'tcx> ConstVal<'tcx> {
    pub fn description(&self) -> &'static str {
        match *self {
            Float(f) => f.description(),
            Integral(i) => i.description(),
            Str(_) => "string literal",
            ByteStr(_) => "byte string literal",
            Bool(_) => "boolean",
            Struct(_) => "struct",
            Tuple(_) => "tuple",
            Function(..) => "function definition",
            Array(..) => "array",
            Repeat(..) => "repeat",
            Char(..) => "char",
        }
    }

    pub fn to_const_int(&self) -> Option<ConstInt> {
        match *self {
            ConstVal::Integral(i) => Some(i),
            ConstVal::Bool(b) => Some(ConstInt::U8(b as u8)),
            ConstVal::Char(ch) => Some(ConstInt::U32(ch as u32)),
            _ => None
        }
    }
}
