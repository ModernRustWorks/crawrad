//! Expression parser and evaluator for Grid conditional style rules.

use std::fmt;

/// Comparison operator against a literal value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Contains,
}

impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "="),
            Self::Ne => write!(f, "<>"),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::Contains => write!(f, "contains"),
        }
    }
}

/// A literal value compared against a row's cell value.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    Text(String),
    Bool(bool),
}

/// AST expression for a style rule's `When` condition.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Compare {
        field: String,
        op: CompareOp,
        value: Literal,
    },
    IsNull {
        field: String,
        negated: bool,
    },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

/// Parse error with 0-based character/byte offset and descriptive message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub at: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (at {})", self.message, self.at)
    }
}

impl std::error::Error for ParseError {}

impl Expr {
    /// Returns all field identifiers referenced by this expression.
    pub fn fields(&self) -> Vec<&str> {
        let mut list = Vec::new();
        self.collect_fields(&mut list);
        list
    }

    fn collect_fields<'a>(&'a self, out: &mut Vec<&'a str>) {
        match self {
            Self::Compare { field, .. } | Self::IsNull { field, .. } => {
                if !out.contains(&field.as_str()) {
                    out.push(field.as_str());
                }
            }
            Self::And(a, b) | Self::Or(a, b) => {
                a.collect_fields(out);
                b.collect_fields(out);
            }
        }
    }

    /// Evaluates this expression using a field getter function.
    ///
    /// Unknown or missing fields evaluate to `false` for all comparison operations.
    /// For `is null`, a missing/NULL value evaluates to `true` (`false` if negated).
    pub fn eval(&self, get: &dyn Fn(&str) -> Option<String>) -> bool {
        match self {
            Self::IsNull { field, negated } => {
                let is_null = get(field).is_none();
                if *negated { !is_null } else { is_null }
            }
            Self::Compare { field, op, value } => {
                let Some(cell_text) = get(field) else {
                    return false;
                };
                match value {
                    Literal::Number(target) => {
                        let Ok(num) = cell_text.trim().parse::<f64>() else {
                            return false;
                        };
                        match op {
                            CompareOp::Eq => (num - target).abs() < f64::EPSILON,
                            CompareOp::Ne => (num - target).abs() >= f64::EPSILON,
                            CompareOp::Lt => num < *target,
                            CompareOp::Le => num <= *target,
                            CompareOp::Gt => num > *target,
                            CompareOp::Ge => num >= *target,
                            CompareOp::Contains => false,
                        }
                    }
                    Literal::Text(target) => match op {
                        CompareOp::Eq => cell_text == *target,
                        CompareOp::Ne => cell_text != *target,
                        CompareOp::Lt => cell_text < *target,
                        CompareOp::Le => cell_text <= *target,
                        CompareOp::Gt => cell_text > *target,
                        CompareOp::Ge => cell_text >= *target,
                        CompareOp::Contains => cell_text.contains(target.as_str()),
                    },
                    Literal::Bool(target) => {
                        let cell_bool =
                            if cell_text == "1" || cell_text.eq_ignore_ascii_case("true") {
                                Some(true)
                            } else if cell_text == "0" || cell_text.eq_ignore_ascii_case("false") {
                                Some(false)
                            } else {
                                None
                            };
                        let Some(b) = cell_bool else {
                            return false;
                        };
                        match op {
                            CompareOp::Eq => b == *target,
                            CompareOp::Ne => b != *target,
                            _ => false,
                        }
                    }
                }
            }
            Self::And(left, right) => left.eval(get) && right.eval(get),
            Self::Or(left, right) => left.eval(get) || right.eval(get),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Ident(String),
    Number(f64),
    StringLit(String),
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LParen,
    RParen,
    UnknownOp(String),
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    kind: TokenKind,
    at: usize,
    len: usize,
}

fn tokenize(src: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let bytes = src.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }

        let at = i;
        if b == b'(' {
            tokens.push(Token {
                kind: TokenKind::LParen,
                at,
                len: 1,
            });
            i += 1;
        } else if b == b')' {
            tokens.push(Token {
                kind: TokenKind::RParen,
                at,
                len: 1,
            });
            i += 1;
        } else if b == b'=' {
            if i + 1 < len && bytes[i + 1] == b'=' {
                tokens.push(Token {
                    kind: TokenKind::UnknownOp("==".to_string()),
                    at,
                    len: 2,
                });
                i += 2;
            } else {
                tokens.push(Token {
                    kind: TokenKind::Eq,
                    at,
                    len: 1,
                });
                i += 1;
            }
        } else if b == b'!' {
            if i + 1 < len && bytes[i + 1] == b'=' {
                tokens.push(Token {
                    kind: TokenKind::UnknownOp("!=".to_string()),
                    at,
                    len: 2,
                });
                i += 2;
            } else {
                tokens.push(Token {
                    kind: TokenKind::UnknownOp("!".to_string()),
                    at,
                    len: 1,
                });
                i += 1;
            }
        } else if b == b'<' {
            if i + 1 < len && bytes[i + 1] == b'>' {
                tokens.push(Token {
                    kind: TokenKind::Ne,
                    at,
                    len: 2,
                });
                i += 2;
            } else if i + 1 < len && bytes[i + 1] == b'=' {
                tokens.push(Token {
                    kind: TokenKind::Le,
                    at,
                    len: 2,
                });
                i += 2;
            } else {
                tokens.push(Token {
                    kind: TokenKind::Lt,
                    at,
                    len: 1,
                });
                i += 1;
            }
        } else if b == b'>' {
            if i + 1 < len && bytes[i + 1] == b'=' {
                tokens.push(Token {
                    kind: TokenKind::Ge,
                    at,
                    len: 2,
                });
                i += 2;
            } else {
                tokens.push(Token {
                    kind: TokenKind::Gt,
                    at,
                    len: 1,
                });
                i += 1;
            }
        } else if b == b'"' {
            // Walk CHARS, not bytes: `"São Paulo"` must stay text, not one byte per char.
            let mut text = String::new();
            let mut closed = false;
            let mut chars = src[i + 1..].char_indices();
            let mut end = len;
            while let Some((off, c)) = chars.next() {
                match c {
                    '"' => {
                        closed = true;
                        end = i + 1 + off + 1;
                        break;
                    }
                    '\\' => match chars.next() {
                        Some((_, 'n')) => text.push('\n'),
                        Some((_, 't')) => text.push('\t'),
                        Some((_, 'r')) => text.push('\r'),
                        Some((_, other)) => text.push(other),
                        None => break,
                    },
                    other => text.push(other),
                }
            }
            if !closed {
                return Err(ParseError {
                    at,
                    message: "unclosed string literal".to_string(),
                });
            }
            tokens.push(Token {
                kind: TokenKind::StringLit(text),
                at,
                len: end - at,
            });
            i = end;
        } else if b == b'-' && i + 1 < len && bytes[i + 1].is_ascii_digit() {
            // Negative number literal
            let start = i;
            i += 1;
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            let text = &src[start..i];
            let num = text.parse::<f64>().map_err(|_| ParseError {
                at: start,
                message: format!("invalid number \"{text}\""),
            })?;
            tokens.push(Token {
                kind: TokenKind::Number(num),
                at: start,
                len: i - start,
            });
        } else if b.is_ascii_digit() {
            let start = i;
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            let text = &src[start..i];
            let num = text.parse::<f64>().map_err(|_| ParseError {
                at: start,
                message: format!("invalid number \"{text}\""),
            })?;
            tokens.push(Token {
                kind: TokenKind::Number(num),
                at: start,
                len: i - start,
            });
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let text = &src[start..i];
            tokens.push(Token {
                kind: TokenKind::Ident(text.to_string()),
                at: start,
                len: i - start,
            });
        } else {
            return Err(ParseError {
                at,
                message: format!("unexpected character '{}'", b as char),
            });
        }
    }

    Ok(tokens)
}

struct Parser<'a> {
    src: &'a str,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str, tokens: Vec<Token>) -> Self {
        Self {
            src,
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn current_at(&self) -> usize {
        self.tokens
            .get(self.pos)
            .map_or(self.src.trim_end().len(), |t| t.at)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while let Some(tok) = self.peek() {
            if let TokenKind::Ident(s) = &tok.kind
                && s.eq_ignore_ascii_case("or")
            {
                self.next();
                let right = self.parse_and()?;
                left = Expr::Or(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_primary()?;
        while let Some(tok) = self.peek() {
            if let TokenKind::Ident(s) = &tok.kind
                && s.eq_ignore_ascii_case("and")
            {
                self.next();
                let right = self.parse_primary()?;
                left = Expr::And(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let Some(tok) = self.peek().cloned() else {
            return Err(ParseError {
                at: self.current_at(),
                message: "expected an expression".to_string(),
            });
        };

        if let TokenKind::LParen = tok.kind {
            let lparen_pos = tok.at;
            self.next();
            let inner = self.parse_expr()?;
            match self.next() {
                Some(Token {
                    kind: TokenKind::RParen,
                    ..
                }) => Ok(inner),
                _ => Err(ParseError {
                    at: lparen_pos,
                    message: "unclosed (".to_string(),
                }),
            }
        } else if let TokenKind::UnknownOp(op) = tok.kind {
            Err(ParseError {
                at: tok.at,
                message: format!("unknown operator \"{op}\""),
            })
        } else if let TokenKind::Ident(field) = tok.kind {
            // Check reserved keywords that cannot start a primary expression
            let lower = field.to_ascii_lowercase();
            if lower == "and" || lower == "or" {
                return Err(ParseError {
                    at: tok.at,
                    message: format!("unexpected keyword \"{field}\""),
                });
            }
            self.next();
            self.parse_condition(field, tok.at)
        } else {
            Err(ParseError {
                at: tok.at,
                message: "expected an expression".to_string(),
            })
        }
    }

    fn parse_condition(&mut self, field: String, field_at: usize) -> Result<Expr, ParseError> {
        let Some(tok) = self.peek().cloned() else {
            return Err(ParseError {
                at: field_at + field.len(),
                message: format!("expected a comparison after \"{field}\""),
            });
        };

        match tok.kind {
            TokenKind::Ident(ref s) if s.eq_ignore_ascii_case("is") => {
                self.next();
                let Some(next_tok) = self.next() else {
                    return Err(ParseError {
                        at: self.current_at(),
                        message: "expected \"null\" or \"not null\" after \"is\"".to_string(),
                    });
                };
                if let TokenKind::Ident(ref kw) = next_tok.kind {
                    if kw.eq_ignore_ascii_case("null") {
                        Ok(Expr::IsNull {
                            field,
                            negated: false,
                        })
                    } else if kw.eq_ignore_ascii_case("not") {
                        let Some(after_not) = self.next() else {
                            return Err(ParseError {
                                at: self.current_at(),
                                message: "expected \"null\" after \"is not\"".to_string(),
                            });
                        };
                        if let TokenKind::Ident(ref nkw) = after_not.kind
                            && nkw.eq_ignore_ascii_case("null")
                        {
                            Ok(Expr::IsNull {
                                field,
                                negated: true,
                            })
                        } else {
                            Err(ParseError {
                                at: after_not.at,
                                message: "expected \"null\" after \"is not\"".to_string(),
                            })
                        }
                    } else {
                        Err(ParseError {
                            at: next_tok.at,
                            message: "expected \"null\" or \"not null\" after \"is\"".to_string(),
                        })
                    }
                } else {
                    Err(ParseError {
                        at: next_tok.at,
                        message: "expected \"null\" or \"not null\" after \"is\"".to_string(),
                    })
                }
            }
            TokenKind::Ident(ref s) if s.eq_ignore_ascii_case("contains") => {
                self.next();
                let value = self.parse_literal("contains", tok.at)?;
                Ok(Expr::Compare {
                    field,
                    op: CompareOp::Contains,
                    value,
                })
            }
            TokenKind::Eq
            | TokenKind::Ne
            | TokenKind::Lt
            | TokenKind::Le
            | TokenKind::Gt
            | TokenKind::Ge => {
                let op = match tok.kind {
                    TokenKind::Eq => CompareOp::Eq,
                    TokenKind::Ne => CompareOp::Ne,
                    TokenKind::Lt => CompareOp::Lt,
                    TokenKind::Le => CompareOp::Le,
                    TokenKind::Gt => CompareOp::Gt,
                    _ => CompareOp::Ge,
                };
                self.next();
                let value = self.parse_literal(&op.to_string(), tok.at)?;
                Ok(Expr::Compare { field, op, value })
            }
            TokenKind::UnknownOp(ref op) => Err(ParseError {
                at: tok.at,
                message: format!("unknown operator \"{op}\""),
            }),
            _ => Err(ParseError {
                at: field_at + field.len(),
                message: format!("expected a comparison after \"{field}\""),
            }),
        }
    }

    fn parse_literal(&mut self, op_str: &str, op_at: usize) -> Result<Literal, ParseError> {
        let Some(tok) = self.next() else {
            return Err(ParseError {
                at: op_at + op_str.len(),
                message: format!("expected a value after \"{op_str}\""),
            });
        };

        match tok.kind {
            TokenKind::Number(n) => Ok(Literal::Number(n)),
            TokenKind::StringLit(s) => Ok(Literal::Text(s)),
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(Literal::Bool(true)),
            TokenKind::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(Literal::Bool(false)),
            _ => Err(ParseError {
                at: tok.at,
                message: format!("expected a value after \"{op_str}\""),
            }),
        }
    }
}

/// Parses a `When` condition into an expression tree.
pub fn parse(src: &str) -> Result<Expr, ParseError> {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return Err(ParseError {
            at: 0,
            message: "expression is empty".to_string(),
        });
    }

    let tokens = tokenize(src)?;
    let mut parser = Parser::new(src, tokens);
    let expr = parser.parse_expr()?;

    if let Some(tok) = parser.peek() {
        if let TokenKind::UnknownOp(op) = &tok.kind {
            return Err(ParseError {
                at: tok.at,
                message: format!("unknown operator \"{op}\""),
            });
        }
        return Err(ParseError {
            at: tok.at,
            message: "unexpected trailing tokens".to_string(),
        });
    }

    Ok(expr)
}
