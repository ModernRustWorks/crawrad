use {
    crate::{
        Token,
        document::{DocumentLayout, IndentState},
        inlays::{BlockInlay, InlineInlay},
        selection::Affinity,
        session::SessionLayout,
        str::StrExt,
        text::{Position, Text},
        widgets::{BlockWidget, InlineWidget},
        wrap::WrapData,
    },
    std::{cell::Ref, slice::Iter},
};

#[derive(Debug)]
pub struct Layout<'a> {
    pub text: Ref<'a, Text>,
    pub document_layout: Ref<'a, DocumentLayout>,
    pub session_layout: Ref<'a, SessionLayout>,
}

impl<'a> Layout<'a> {
    pub fn as_text(&self) -> &Text {
        &self.text
    }

    pub fn width(&self) -> f64 {
        let mut width: f64 = 0.0;
        for line in self.lines(0, self.as_text().as_lines().len()) {
            width = width.max(line.width());
        }
        width
    }

    pub fn height(&self) -> f64 {
        *self.session_layout.y.last().unwrap_or(&0.0)
    }

    pub fn find_first_line_ending_after_y(&self, y: f64) -> usize {
        if self.session_layout.y.len() <= 1 {
            return 0;
        }
        match self.session_layout.y[..self.session_layout.y.len() - 1].binary_search_by(
            |current_y| {
                current_y
                    .partial_cmp(&y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            },
        ) {
            Ok(line) => line,
            Err(line) => line.saturating_sub(1),
        }
    }

    pub fn find_first_line_starting_after_y(&self, y: f64) -> usize {
        if self.session_layout.y.len() <= 1 {
            return 0;
        }
        match self.session_layout.y[..self.session_layout.y.len() - 1].binary_search_by(
            |current_y| {
                current_y
                    .partial_cmp(&y)
                    .unwrap_or(std::cmp::Ordering::Equal)
            },
        ) {
            Ok(line) => line + 1,
            Err(line) => line,
        }
    }

    /// Where the blocks of `line` start: above the lenses or widgets that
    /// come before it, which `line(line).y()` does not count.
    pub fn top_of(&self, line: usize) -> f64 {
        match line.checked_sub(1) {
            Some(prev) => {
                let prev = self.line(prev);
                prev.y() + prev.height()
            }
            None => 0.0,
        }
    }

    /// The line whose lens band holds `y`, if any.
    pub fn lens_at_y(&self, y: f64) -> Option<usize> {
        if y < 0.0 || y >= self.height() {
            return None;
        }
        let total_lines = self.text.as_lines().len();
        let line_start = self.find_first_line_ending_after_y(y);
        let mut line_index = line_start;
        let mut origin_y = self.top_of(line_start);
        for block in self.block_elements(line_start, total_lines) {
            if origin_y > y {
                break;
            }
            let end_y = origin_y + block.height();
            match block {
                BlockElement::Line { .. } => line_index += 1,
                BlockElement::Lens { .. } if y >= origin_y && y < end_y => {
                    return Some(line_index);
                }
                _ => {}
            }
            origin_y = end_y;
        }
        None
    }

    pub fn logical_to_normalized_position(
        &self,
        position: Position,
        affinity: Affinity,
    ) -> (f64, f64) {
        let line = self.line(position.line_index);
        let (row_index, column_index) =
            line.logical_to_grid_position(position.byte_index, affinity);
        let (x, y) = line.grid_to_normalized_position(row_index, column_index);
        (x, line.y() + y)
    }

    pub fn line(&self, index: usize) -> Line<'_> {
        let lines = self.text.as_lines();
        let safe_index = if lines.is_empty() {
            0
        } else {
            index.min(lines.len() - 1)
        };
        Line {
            y: self.session_layout.y.get(safe_index).copied(),
            column_count: self
                .session_layout
                .column_count
                .get(safe_index)
                .copied()
                .flatten(),
            fold: self
                .session_layout
                .fold_column
                .get(safe_index)
                .copied()
                .unwrap_or(0),
            scale: self
                .session_layout
                .scale
                .get(safe_index)
                .copied()
                .unwrap_or(1.0),
            is_collapsed: self.session_layout.collapsed.contains(&safe_index),
            text: if safe_index < lines.len() {
                &lines[safe_index]
            } else {
                ""
            },
            indent_state: self
                .document_layout
                .indent_state
                .get(safe_index)
                .copied()
                .flatten(),
            tokens: self
                .document_layout
                .tokens
                .get(safe_index)
                .map(|v| v.as_slice())
                .unwrap_or(&[]),
            inlays: self
                .document_layout
                .inline_inlays
                .get(safe_index)
                .map(|v| v.as_slice())
                .unwrap_or(&[]),
            wrap_data: self
                .session_layout
                .wrap_data
                .get(safe_index)
                .and_then(|v| v.as_ref()),
        }
    }

    pub fn lines(&self, start: usize, end: usize) -> Lines<'_> {
        let text_lines = self.text.as_lines();
        let total = text_lines.len();
        let s = start.min(total);
        let e = end.min(total).max(s);
        let y_len = self.session_layout.y.len();
        let y_s = start.min(y_len);
        let y_e = end.min(y_len).max(y_s);

        let col_len = self.session_layout.column_count.len();
        let col_s = start.min(col_len);
        let col_e = end.min(col_len).max(col_s);

        let fold_len = self.session_layout.fold_column.len();
        let fold_s = start.min(fold_len);
        let fold_e = end.min(fold_len).max(fold_s);

        let scale_len = self.session_layout.scale.len();
        let scale_s = start.min(scale_len);
        let scale_e = end.min(scale_len).max(scale_s);

        let indent_len = self.document_layout.indent_state.len();
        let indent_s = start.min(indent_len);
        let indent_e = end.min(indent_len).max(indent_s);

        let tokens_len = self.document_layout.tokens.len();
        let tokens_s = start.min(tokens_len);
        let tokens_e = end.min(tokens_len).max(tokens_s);

        let inlays_len = self.document_layout.inline_inlays.len();
        let inlays_s = start.min(inlays_len);
        let inlays_e = end.min(inlays_len).max(inlays_s);

        let wrap_len = self.session_layout.wrap_data.len();
        let wrap_s = start.min(wrap_len);
        let wrap_e = end.min(wrap_len).max(wrap_s);

        Lines {
            y: self.session_layout.y[y_s..y_e].iter(),
            column_count: self.session_layout.column_count[col_s..col_e].iter(),
            fold: self.session_layout.fold_column[fold_s..fold_e].iter(),
            scale: self.session_layout.scale[scale_s..scale_e].iter(),
            text: text_lines[s..e].iter(),
            indent_state: self.document_layout.indent_state[indent_s..indent_e].iter(),
            tokens: self.document_layout.tokens[tokens_s..tokens_e].iter(),
            inline_inlays: self.document_layout.inline_inlays[inlays_s..inlays_e].iter(),
            wrap_data: self.session_layout.wrap_data[wrap_s..wrap_e].iter(),
            collapsed: &self.session_layout.collapsed,
            line_index: s,
        }
    }

    pub fn block_elements(&self, line_start: usize, line_end: usize) -> BlockElements<'_> {
        let mut block_inlays = self.document_layout.block_inlays.iter();
        while block_inlays
            .as_slice()
            .first()
            .is_some_and(|&(position, _)| position < line_start)
        {
            block_inlays.next();
        }
        BlockElements {
            lines: self.lines(line_start, line_end),
            block_inlays,
            position: line_start,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lines<'a> {
    y: Iter<'a, f64>,
    column_count: Iter<'a, Option<usize>>,
    fold: Iter<'a, usize>,
    scale: Iter<'a, f64>,
    text: Iter<'a, String>,
    indent_state: Iter<'a, Option<IndentState>>,
    tokens: Iter<'a, Vec<Token>>,
    inline_inlays: Iter<'a, Vec<(usize, InlineInlay)>>,
    wrap_data: Iter<'a, Option<WrapData>>,
    collapsed: &'a std::collections::HashSet<usize>,
    line_index: usize,
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.text.next()?;
        let is_collapsed = self.collapsed.contains(&self.line_index);
        self.line_index += 1;
        Some(Line {
            y: self.y.next().copied(),
            column_count: self.column_count.next().copied().flatten(),
            fold: self.fold.next().copied().unwrap_or(0),
            scale: self.scale.next().copied().unwrap_or(1.0),
            is_collapsed,
            text,
            indent_state: self.indent_state.next().copied().flatten(),
            tokens: self.tokens.next().map(|v| v.as_slice()).unwrap_or(&[]),
            inlays: self
                .inline_inlays
                .next()
                .map(|v| v.as_slice())
                .unwrap_or(&[]),
            wrap_data: self.wrap_data.next().and_then(|v| v.as_ref()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line<'a> {
    pub y: Option<f64>,
    pub column_count: Option<usize>,
    pub fold: usize,
    pub scale: f64,
    pub is_collapsed: bool,
    pub text: &'a str,
    pub indent_state: Option<IndentState>,
    pub tokens: &'a [Token],
    pub inlays: &'a [(usize, InlineInlay)],
    pub wrap_data: Option<&'a WrapData>,
}

impl<'a> Line<'a> {
    pub fn y(&self) -> f64 {
        self.y.unwrap_or(0.0)
    }

    pub fn row_count(&self) -> usize {
        self.wrap_data.map(|w| w.wraps.len() + 1).unwrap_or(1)
    }

    pub fn column_count(&self) -> usize {
        self.column_count.unwrap_or(0)
    }

    pub fn width(&self) -> f64 {
        let mut width: f64 = 0.0;
        for row_index in 0..self.row_count() {
            let (x, _) = self.grid_to_normalized_position(row_index, self.column_count());
            width = width.max(x);
        }
        width
    }

    pub fn is_collapsed(&self) -> bool {
        self.is_collapsed
    }

    pub fn height(&self) -> f64 {
        if self.is_collapsed {
            0.0
        } else {
            self.row_count() as f64 * self.scale
        }
    }

    pub fn logical_to_grid_position(
        &self,
        byte_index: usize,
        affinity: Affinity,
    ) -> (usize, usize) {
        let mut current_byte_index = 0;
        let mut current_row_index = 0;
        let mut current_column_index = 0;
        if current_byte_index == byte_index && affinity == Affinity::Before {
            return (current_row_index, current_column_index);
        }
        for element in self.wrapped_elements() {
            match element {
                WrappedElement::Text {
                    is_inlay: false,
                    text,
                } => {
                    for grapheme in text.graphemes() {
                        if current_byte_index == byte_index && affinity == Affinity::After {
                            return (current_row_index, current_column_index);
                        }
                        current_byte_index += grapheme.len();
                        current_column_index += grapheme.column_count();
                        if current_byte_index == byte_index && affinity == Affinity::Before {
                            return (current_row_index, current_column_index);
                        }
                    }
                }
                WrappedElement::Text {
                    is_inlay: true,
                    text,
                } => {
                    current_column_index += text.column_count();
                }
                WrappedElement::Widget(widget) => {
                    current_column_index += widget.column_count;
                }
                WrappedElement::Wrap => {
                    current_row_index += 1;
                    current_column_index = self.wrap_indent_column_count();
                }
            }
        }
        if current_byte_index == byte_index && affinity == Affinity::After {
            return (current_row_index, current_column_index);
        }
        panic!()
    }

    pub fn grid_to_logical_position(
        &self,
        row_index: usize,
        column_index: usize,
    ) -> (usize, Affinity) {
        let mut current_row_index = 0;
        let mut current_column_index = 0;
        let mut current_byte_index = 0;
        for element in self.wrapped_elements() {
            match element {
                WrappedElement::Text {
                    is_inlay: false,
                    text,
                } => {
                    for grapheme in text.graphemes() {
                        let next_column = current_column_index + grapheme.column_count();
                        if current_row_index == row_index
                            && (current_column_index..next_column).contains(&column_index)
                        {
                            return (current_byte_index, Affinity::After);
                        }
                        current_byte_index += grapheme.len();
                        current_column_index = next_column;
                    }
                }
                WrappedElement::Text {
                    is_inlay: true,
                    text,
                } => {
                    let next_column = current_column_index + text.column_count();
                    if current_row_index == row_index
                        && (current_column_index..next_column).contains(&column_index)
                    {
                        return (current_byte_index, Affinity::Before);
                    }
                    current_column_index = next_column;
                }
                WrappedElement::Widget(widget) => {
                    current_column_index += widget.column_count;
                }
                WrappedElement::Wrap => {
                    if current_row_index == row_index {
                        return (current_byte_index, Affinity::Before);
                    }
                    current_row_index += 1;
                    current_column_index = self.wrap_indent_column_count();
                }
            }
        }
        if current_row_index == row_index {
            return (current_byte_index, Affinity::After);
        }
        panic!()
    }

    pub fn grid_to_normalized_position(&self, row_index: usize, column_index: usize) -> (f64, f64) {
        let before_fold = column_index.min(self.fold);
        let after_fold = column_index - before_fold;
        (
            before_fold as f64 + after_fold as f64 * self.scale,
            row_index as f64 * self.scale,
        )
    }

    pub fn fold(&self) -> usize {
        self.fold
    }

    pub fn scale(&self) -> f64 {
        if self.is_collapsed { 0.0 } else { self.scale }
    }

    pub fn wrap_indent_column_count(self) -> usize {
        self.wrap_data.map(|w| w.indent_column_count).unwrap_or(0)
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn indent_column_count(&self) -> usize {
        match self.indent_state {
            Some(IndentState::Empty(indent_column_count)) => indent_column_count,
            Some(IndentState::NonEmpty(indent_column_count, _)) => indent_column_count,
            None => 0,
        }
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens
    }

    pub fn inline_elements(&self) -> InlineElements<'a> {
        InlineElements {
            text: self.text,
            inlays: self.inlays.iter(),
            position: 0,
        }
    }

    pub fn wrapped_elements(&self) -> WrappedElements<'a> {
        let mut elements = self.inline_elements();
        static EMPTY_WRAPS: [usize; 0] = [];
        let wraps = self
            .wrap_data
            .map(|w| w.wraps.as_slice())
            .unwrap_or(&EMPTY_WRAPS)
            .iter();
        WrappedElements {
            element: elements.next(),
            elements,
            wraps,
            position: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InlineElements<'a> {
    text: &'a str,
    inlays: Iter<'a, (usize, InlineInlay)>,
    position: usize,
}

impl<'a> Iterator for InlineElements<'a> {
    type Item = InlineElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .inlays
            .as_slice()
            .first()
            .is_some_and(|&(position, _)| position == self.position)
        {
            let (_, inline_inlay) = self.inlays.next().unwrap();
            return Some(match *inline_inlay {
                InlineInlay::Text(ref text) | InlineInlay::Hint(ref text) => InlineElement::Text {
                    is_inlay: true,
                    text,
                },
                InlineInlay::Widget(widget) => InlineElement::Widget(widget),
            });
        }
        if self.text.is_empty() {
            return None;
        }
        let mut len: usize = self.text.len();
        if let Some(&(position, _)) = self.inlays.as_slice().first() {
            len = len.min(position - self.position);
        }
        let (text_0, text_1) = self.text.split_at(len);
        self.text = text_1;
        self.position += text_0.len();
        Some(InlineElement::Text {
            is_inlay: false,
            text: text_0,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InlineElement<'a> {
    Text { is_inlay: bool, text: &'a str },
    Widget(InlineWidget),
}

#[derive(Clone, Debug)]
pub struct WrappedElements<'a> {
    element: Option<InlineElement<'a>>,
    elements: InlineElements<'a>,
    wraps: Iter<'a, usize>,
    position: usize,
}

impl<'a> Iterator for WrappedElements<'a> {
    type Item = WrappedElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .wraps
            .as_slice()
            .first()
            .is_some_and(|&position| position == self.position)
        {
            self.wraps.next();
            return Some(WrappedElement::Wrap);
        }
        Some(match self.element.take()? {
            InlineElement::Text { is_inlay, text } => {
                let mut len: usize = text.len();
                if let Some(&position) = self.wraps.as_slice().first() {
                    len = len.min(position - self.position);
                }
                let text = if len < text.len() {
                    let (text_0, text_1) = text.split_at(len);
                    self.element = Some(InlineElement::Text {
                        is_inlay,
                        text: text_1,
                    });
                    text_0
                } else {
                    self.element = self.elements.next();
                    text
                };
                self.position += text.len();
                WrappedElement::Text { is_inlay, text }
            }
            InlineElement::Widget(widget) => {
                self.position += 1;
                WrappedElement::Widget(widget)
            }
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WrappedElement<'a> {
    Text { is_inlay: bool, text: &'a str },
    Widget(InlineWidget),
    Wrap,
}

#[derive(Clone, Debug)]
pub struct BlockElements<'a> {
    lines: Lines<'a>,
    block_inlays: Iter<'a, (usize, BlockInlay)>,
    position: usize,
}

impl<'a> Iterator for BlockElements<'a> {
    type Item = BlockElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .block_inlays
            .as_slice()
            .first()
            .is_some_and(|&(line, _)| line == self.position)
        {
            let (_, block_inlay) = self.block_inlays.next()?;
            return Some(match block_inlay {
                BlockInlay::Widget(widget) => BlockElement::Widget(*widget),
                BlockInlay::Lens(text) => BlockElement::Lens {
                    text: text.as_str(),
                },
            });
        }
        let line = self.lines.next()?;
        self.position += 1;
        Some(BlockElement::Line {
            is_inlay: false,
            line,
        })
    }
}

/// The height of a code lens row: one row of text.
pub const LENS_HEIGHT: f64 = 1.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockElement<'a> {
    Line { is_inlay: bool, line: Line<'a> },
    Widget(BlockWidget),
    Lens { text: &'a str },
}

impl<'a> BlockElement<'a> {
    pub fn height(&self) -> f64 {
        match self {
            Self::Line { line, .. } => line.height(),
            Self::Widget(widget) => widget.height,
            Self::Lens { .. } => LENS_HEIGHT,
        }
    }
}
