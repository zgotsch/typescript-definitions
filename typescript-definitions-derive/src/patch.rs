// Copyright 2019 Ian Castleden
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Patch
//!
//! we are generating *typescript* from rust tokens so
//! the final result when rendered to a string has a typescript
//! formatting problem. This mod just applies a few patches
//! to make the final result a little more acceptable.
//!

use lazy_static::lazy_static;
use proc_macro2::Literal;
use regex::{Captures, Regex};
use std::borrow::Cow;

// In typescript '===' is a single token whereas
// for rust this would be two tokens '==' and '=',
// and fails to generate correct typescript/javascript.
// So we subsitute the operator with this identifier and then patch
// it back *after* we generate the string.
// The problem is that someone, somewhere might have
// an identifer that is this... We hope and pray.
//
// This is also the reason we prefer !(x === y) to x !== y ..
// too much patching.

// no field names have anything but ascii at the moment.

const TRIPLE_EQ: &str = "\"__============__\"";
const NL_PATCH: &str = "\"__nlnlnlnl__\"";
// type N = [(&'static str, &'static str); 10];
const PATCHES_SPEC: [(&str, &str, &str); 13] = [
    ("brack", r"\s+\[\s*\]", "[]"),
    ("brace", r"\{\s+\}", "{}"),
    ("colon", r"\s+[:]\s+", ": "),
    ("enl", r"\s*\n+\}", " }"),
    ("fnl", r"\{\n+\s*", "{ "),
    ("te", TRIPLE_EQ, "==="),
    ("lt", r"\s<\s", "<"),
    ("gt", r"\s>(\s|$)", ">"),
    ("semi", r"\s+;", ";"),
    ("call", r"\s\(\s+\)\s", " () "),
    ("dot", r"\s\.\s", "."),
    ("nl", r"\s*\n+\s*", " "), // need to do this before nl_patch, and after everything that cares about spaces
    ("nlpatch", NL_PATCH, "\n"), // for adding newlines to output string
];

lazy_static! {
    static ref PATCHES: Vec<(&'static str, Regex, &'static str)> = PATCHES_SPEC
        .iter()
        .map(|(n, re, s)| (*n, Regex::new(re).unwrap(), *s))
        .collect();
}

pub fn patch(s: &str) -> String {
    let mut s = s.to_owned();
    for (_n, re, replacement) in &*PATCHES {
        s = re.replace_all(&s, *replacement).into_owned();
    }
    s
}

#[inline]
pub fn eq() -> Literal {
    Literal::string(&TRIPLE_EQ[1..TRIPLE_EQ.len() - 1])
}

#[inline]
pub fn nl() -> Literal {
    Literal::string(&NL_PATCH[1..NL_PATCH.len() - 1])
}

// #[inline]
// pub fn vbar() -> Ident {
//     ident_from_str(RESULT_BAR)
// }
