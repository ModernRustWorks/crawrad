//! **This file is yours.** CrawRAD does not regenerate it, and it
//! never removes code you wrote.
//!
//! It does write to the file, though — better you know than be
//! surprised:
//!
//! - wiring a new event in the Object Inspector inserts the
//!   handler at the marker below, leaving the rest alone;
//! - generating takes empty event handlers back out, and
//!   saving generates — so saving takes them out too. It does
//!   not matter whether the event is still wired: the wiring
//!   is dropped in the same step, so the form still compiles.
//!   This only ever reaches a handler that is still the empty
//!   one CrawRAD wrote, under the very name CrawRAD gave it.
//!   Type one character into the body, even a comment; rename
//!   the function; or write a line of your own right above it,
//!   such as a doc comment or an attribute — any of these and
//!   it is yours: it stays for good.
//!   Methods you wrote yourself are never looked at, whatever
//!   you name them and whatever you write above them;
//! - renaming the form moves this file along with it and
//!   replaces the form's type name throughout — in your code
//!   AND in your comments — because otherwise it would not
//!   compile.

// One glob, on purpose: it brings in the form's type, which
// the `impl` below is written on, and the event payload types
// (`KeyEvent`, `MouseEvent`) that a handler signature names
// with no path.
use crate::form_main_designer::*;

impl FormMain {
    /// `btn_7.on_click`
    pub fn btn_7_click(&mut self) {
        self.press("7");
    }

    /// `btn_8.on_click`
    pub fn btn_8_click(&mut self) {
        self.press("8");
    }

    /// `btn_9.on_click`
    pub fn btn_9_click(&mut self) {
        self.press("9");
    }

    /// `btn_divide.on_click`
    pub fn btn_divide_click(&mut self) {
        self.press("/");
    }

    /// `btn_4.on_click`
    pub fn btn_4_click(&mut self) {
        self.press("4");
    }

    /// `btn_5.on_click`
    pub fn btn_5_click(&mut self) {
        self.press("5");
    }

    /// `btn_6.on_click`
    pub fn btn_6_click(&mut self) {
        self.press("6");
    }

    /// `btn_multiply.on_click`
    pub fn btn_multiply_click(&mut self) {
        self.press("*");
    }

    /// `btn_1.on_click`
    pub fn btn_1_click(&mut self) {
        self.press("1");
    }

    /// `btn_2.on_click`
    pub fn btn_2_click(&mut self) {
        self.press("2");
    }

    /// `btn_3.on_click`
    pub fn btn_3_click(&mut self) {
        self.press("3");
    }

    /// `btn_subtract.on_click`
    pub fn btn_subtract_click(&mut self) {
        self.press("-");
    }

    /// `btn_clear.on_click`
    pub fn btn_clear_click(&mut self) {
        self.display.text = "0".into();
    }

    /// `btn_0.on_click`
    pub fn btn_0_click(&mut self) {
        self.press("0");
    }

    /// `btn_equals.on_click`
    pub fn btn_equals_click(&mut self) {
        self.display.text = match evaluate(&self.display.text) {
            Some(value) => format_number(value),
            None => "Error".into(),
        };
    }

    /// `btn_add.on_click`
    pub fn btn_add_click(&mut self) {
        self.press("+");
    }

    // New event handlers go here.

    /// Appends a digit or an operator to the expression on the display.
    fn press(&mut self, key: &str) {
        let shown = &self.display.text;
        if shown == "0" || shown == "Error" {
            self.display.text = key.into();
        } else {
            self.display.text.push_str(key);
        }
    }
}

/// Evaluates `7+8*2` with the usual precedence: `*` and `/` before `+` and `-`.
/// `None` for a malformed expression or a division by zero.
fn evaluate(expression: &str) -> Option<f64> {
    let mut sum = 0.0;
    let mut sign = 1.0;
    let mut term: Option<f64> = None;
    let mut pending: Option<char> = None;
    let mut number = String::new();
    for c in expression.chars().chain(std::iter::once('+')) {
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
            continue;
        }
        let value: f64 = number.parse().ok()?;
        number.clear();
        term = Some(match (term, pending) {
            (Some(t), Some('*')) => t * value,
            (Some(t), Some('/')) if value != 0.0 => t / value,
            (Some(_), Some('/')) => return None,
            _ => value,
        });
        match c {
            '*' | '/' => pending = Some(c),
            '+' | '-' => {
                sum += sign * term.take()?;
                sign = if c == '-' { -1.0 } else { 1.0 };
                pending = None;
            }
            _ => return None,
        }
    }
    Some(sum)
}

/// `14` rather than `14.0`; at most ten decimals otherwise.
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        let text = format!("{value:.10}");
        text.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
