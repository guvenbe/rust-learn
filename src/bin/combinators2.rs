fn main() {
    let a = Some(1);
    dbg!(a);
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    let a_mapped = a.map(|number| number + 1);
    dbg!(a_mapped);
    let a_filter = a.filter(|num| num == &1);
    dbg!(a_filter);
    let a_or_else = a.or_else(||Some(5));
    dbg!(a_or_else);
    let a_unwrapped = a.unwrap_or_else(|| 0);
    dbg!(a_unwrapped);
}