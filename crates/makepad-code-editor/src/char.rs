pub trait CharExt {
    fn is_opening_delimiter(&self) -> bool;
    fn is_closing_delimiter(&self) -> bool;
    fn column_count(self) -> usize;
    fn opposite_delimiter(&self) -> Option<char>;
}

impl CharExt for char {
    fn is_opening_delimiter(&self) -> bool {
        matches!(self, '(' | '[' | '{')
    }

    fn is_closing_delimiter(&self) -> bool {
        matches!(self, ')' | ']' | '}')
    }

    fn column_count(self) -> usize {
        1
    }

    fn opposite_delimiter(&self) -> Option<char> {
        Some(match self {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            _ => return None,
        })
    }
}
