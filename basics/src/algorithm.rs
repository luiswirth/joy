use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    if a.len() == b.len() {
        if *a == *b {
            return Equal;
        }
    } else if a.len() > b.len() {
        if b.len() == 0 || a.windows(b.len()).any(|s| s == b) {
            return Superlist;
        }
    } else if a.len() < b.len() {
        if a.len() == 0 || b.windows(a.len()).any(|s| s == a) {
            return Sublist;
        }
    }
    Unequal
}

#[derive(Debug, PartialEq)]
enum Occurences {
    OnlyA,
    OnlyB,
    Both,
}

pub fn subset<T: Copy + PartialEq + Eq + std::hash::Hash>(a: &[T], b: &[T]) -> Comparison {
    let mut map: HashMap<T, Occurences> = a.iter().map(|t| (*t, Occurences::OnlyA)).collect();

    b.iter().for_each(|t| {
        let e = map.entry(*t).or_insert(Occurences::OnlyB);
        *e = Occurences::Both;
    });

    let could_be_sublist = !map.values().any(|o| *o == Occurences::OnlyA);
    let could_be_superlist = !map.values().any(|o| *o == Occurences::OnlyB);
    match (could_be_sublist, could_be_superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
