use std::collections::HashSet;

pub fn has_only_unique<T, I>(a: T) -> bool
where
    T: IntoIterator<Item = I>,
    I: Eq + std::hash::Hash,
{
    let mut items: HashSet<I> = HashSet::new();
    for n in a {
        if items.contains(&n) {
            return false;
        } else {
            items.insert(n);
        }
    }
    true
}

#[allow(dead_code)]
pub fn contains_all<T, I>(haystack: &HashSet<T>, needles: I) -> bool
where
    T: Eq + std::hash::Hash,
    I: IntoIterator<Item = T>,
{
    for n in needles {
        if !haystack.contains(&n) {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn get_first_not_in_set<T, I>(haystack: &HashSet<T>, needles: I) -> Option<T>
where
    T: Eq + std::hash::Hash,
    I: IntoIterator<Item = T>,
{
    for n in needles {
        if !haystack.contains(&n) {
            return Some(n);
        }
    }
    None
}
