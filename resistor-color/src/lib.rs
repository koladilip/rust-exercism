use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoEnumIterator)]
pub enum ResistorColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::into_enum_iter()
        .find(|r| (*r) as usize == value)
        .map(|r| format!("{:?}", r))
        .unwrap_or(String::from("value out of range"))
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
