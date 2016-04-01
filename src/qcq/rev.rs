use nom::IResult::*;
use std;

pub fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for x in xs {
        rev.insert(0, x.clone())
    }
    rev
}

pub fn parse(data:&[u8]) -> Option<bool> {
    match aaabccc(data) {
        Done(_, _) => Some(true),
        _ => None
    }
}

named!(aaabccc <&[u8], &str>,
    map_res!(
        recognize!(
            chain!(
                take_while!(is_a) ~
                tag!("b") ~
                take_while!(is_c),
                || {}
            )
        ),
        std::str::from_utf8
   )
);

fn is_a(l: u8) -> bool {
    match l {
        b'a' => true,
        _ => false,
    }
}

fn is_c(l: u8) -> bool {
    match l {
        b'c' => true,
        _ => false,
    }
    }

#[cfg(test)]
mod tests {
    #[quickcheck]
    fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
        xs == super::reverse(&super::reverse(&xs))
    }
}
