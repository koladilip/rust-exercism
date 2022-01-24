#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.len() >= _second_list.len() {
        return false;
    }
    return _second_list.windows(_first_list.len()).any(|l| l == _first_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    return match (_first_list, _second_list) {
        ([], []) => Equal,
        ([], _) => Sublist,
        (_, []) => Superlist,
        (x, y) if x == y => Equal,
        (x, y) if is_sublist(x, y) => Sublist,
        (x, y) if is_sublist(y, x) => Superlist,
        _ => Unequal
    }
}