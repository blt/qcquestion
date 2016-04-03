#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
use quickcheck::{Arbitrary,Gen};

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct C<'a> {
    s: &'a str,
    b: bool
}

#[cfg(test)]
impl<'a> Arbitrary for C<'a> {
    fn arbitrary<G: Gen>(g: &mut G) -> C<'a> {
        let s = g.gen::<&str>();
        C{s: s, b: (s.len() > 0)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn len_checks_out(c: C) -> bool {
        (c.s.len() > 0) == c.b
    }
}
