#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    let mut shorter = _first_list;
    let mut longer = _second_list;
    let mut sub_list =  Comparison::Sublist;

    if _first_list.len() > _second_list.len() {
        (shorter,longer) = (longer, shorter);
        sub_list = Comparison::Superlist;
    }

    if shorter.is_empty() || longer.windows(shorter.len()).any(|window| window == shorter) {
        sub_list
    }else {
         Comparison::Unequal
    }
}
