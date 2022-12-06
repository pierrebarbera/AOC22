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
