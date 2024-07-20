#[derive(Debug, Clone, Eq, PartialEq)]

/// An error occurred while
pub struct UnescapeError;

type UnescapeResult<T> = Result<T, UnescapeError>;

// Used to collect output characters and queue u16 values for translation.
pub struct UnescapeState {
    // The accumulated characters
    out: Vec<u8>,
}

impl UnescapeState {
    fn with_capacity(capacity: usize) -> UnescapeState {
        UnescapeState {
            out: Vec::with_capacity(capacity),
        }
    }

    // Collect a new character
    fn push_char(&mut self, c: char) {
        let mut buff = [0; 8];
        self.out
            .extend_from_slice(c.encode_utf8(&mut buff).as_bytes());
    }

    fn push_u8(&mut self, c: u8) {
        self.out.push(c);
    }

    fn push_raw(&mut self, c: u32) -> UnescapeResult<()> {
        match std::char::from_u32(c) {
            Some(c) => {
                self.push_char(c);
                Ok(())
            }
            None => Err(UnescapeError),
        }
    }

    fn push_slice(&mut self, slice: &[u8]) {
        self.out.extend_from_slice(slice);
    }

    fn finalize(self) -> UnescapeResult<String> {
        String::from_utf8(self.out).map_err(|_| UnescapeError)
    }
}

fn parse_u32(
    s: &mut PeekableBytes,
    radix: u32,
    mut result: u32,
    max: Option<u8>,
) -> UnescapeResult<u32> {
    let mut max = max.unwrap_or(u8::MAX);
    while let Some(digit) = s.peek().and_then(|digit| (digit as char).to_digit(radix)) {
        let _ = s.next(); // consume the digit we peeked
        result = result.checked_mul(radix).ok_or(UnescapeError)?;
        result = result.checked_add(digit).ok_or(UnescapeError)?;
        max -= 1;
        if max == 0 {
            break;
        }
    }
    Ok(result)
}

pub trait EscapedString {
    fn handle_escape<'a>(bytes: &'a [u8], state: &mut UnescapeState) -> UnescapeResult<&'a [u8]>;
}

pub struct SingleQuoteString;

impl EscapedString for SingleQuoteString {
    fn handle_escape<'a>(bytes: &'a [u8], state: &mut UnescapeState) -> UnescapeResult<&'a [u8]> {
        let mut ins = PeekableBytes::new(bytes);
        let _slash = ins.next();
        debug_assert_eq!(_slash, Some(b'\\'));
        match ins.next() {
            None => {
                return Err(UnescapeError);
            }
            Some(d) => match d {
                b'\\' | b'\'' => state.push_u8(d),
                _ => {
                    state.push_u8(b'\\');
                    state.push_u8(d)
                }
            },
        }
        Ok(ins.as_slice())
    }
}

pub struct DoubleQuoteString;

impl EscapedString for DoubleQuoteString {
    fn handle_escape<'a>(bytes: &'a [u8], state: &mut UnescapeState) -> UnescapeResult<&'a [u8]> {
        let mut ins = PeekableBytes::new(bytes);
        let _next = ins.next();
        debug_assert_eq!(_next, Some(b'\\'));
        match ins.next() {
            None => {
                return Err(UnescapeError);
            }
            Some(d) => {
                match d {
                    b'$' | b'"' | b'\\' => state.push_u8(d),
                    b'n' => state.push_u8(b'\n'),   // linefeed
                    b'r' => state.push_u8(b'\r'),   // carriage return
                    b't' => state.push_u8(b'\t'),   // tab
                    b'v' => state.push_u8(b'\x0B'), // vertical tab
                    b'f' => state.push_u8(b'\x0C'), // form feed
                    b'x' => {
                        let val = parse_u32(&mut ins, 16, 0, Some(2))?;
                        state.push_raw(val)?;
                    }
                    b'u' => match ins.next() {
                        Some(b'{') => {
                            let val = parse_u32(&mut ins, 16, 0, None)?;
                            state.push_raw(val)?;
                            if !matches!(ins.next(), Some(b'}')) {
                                return Err(UnescapeError);
                            }
                        }
                        Some(d) => {
                            state.push_u8(b'\\');
                            state.push_u8(b'u');
                            state.push_u8(d);
                        }
                        None => {
                            state.push_u8(b'\\');
                            state.push_u8(d);
                        }
                    },
                    b'0'..=b'7' => {
                        let val =
                            parse_u32(&mut ins, 8, (d as char).to_digit(8).unwrap(), Some(3))?;
                        state.push_raw(val)?;
                    }
                    _ => {
                        state.push_u8(b'\\');
                        state.push_u8(d)
                    }
                }
            }
        }
        Ok(ins.as_slice())
    }
}

pub fn parse_string(literal: &str) -> Result<String, UnescapeError> {
    let inner = &literal[1..(literal.len()) - 1];
    if literal.bytes().next().unwrap() == b'\'' {
        unescape::<SingleQuoteString>(inner)
    } else {
        unescape::<DoubleQuoteString>(inner)
    }
}

pub fn unescape<S: EscapedString>(s: &str) -> UnescapeResult<String> {
    let mut state = UnescapeState::with_capacity(s.len());
    let mut bytes = s.as_bytes();
    while let Some(escape_index) = memchr::memchr(b'\\', bytes) {
        state.push_slice(&bytes[0..escape_index]);
        bytes = &bytes[escape_index..];
        bytes = S::handle_escape(bytes, &mut state)?;
    }

    state.push_slice(&bytes[0..]);

    state.finalize()
}

struct PeekableBytes<'a> {
    slice: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for PeekableBytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.slice.get(self.pos)?;
        self.pos += 1;
        Some(*byte)
    }
}

impl<'a> PeekableBytes<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        PeekableBytes { slice, pos: 0 }
    }

    pub fn peek(&self) -> Option<u8> {
        self.slice.get(self.pos).copied()
    }

    pub fn as_slice(&self) -> &'a [u8] {
        &self.slice[self.pos..]
    }
}

pub fn is_array_key_numeric(string: &str) -> bool {
    let mut bytes = string.bytes();
    if !matches!(
        (bytes.next(), string.len()),
        (Some(b'-'), _) | (Some(b'0'..=b'9'), 1) | (Some(b'1'..=b'9'), _)
    ) {
        return false;
    }

    bytes.all(|byte| byte.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_numeric() {
        assert!(is_array_key_numeric("123"));
        assert!(is_array_key_numeric("-123"));
        assert!(is_array_key_numeric("0"));
        assert!(!is_array_key_numeric("0123"));
        assert!(!is_array_key_numeric("123asd"));
        assert!(!is_array_key_numeric("+123"));
    }

    #[test]
    fn test_unescape_single() {
        assert_eq!(unescape::<SingleQuoteString>(r#"abc"#), Ok("abc".into()));
        assert_eq!(
            unescape::<SingleQuoteString>(r#"ab\nc"#),
            Ok("ab\\nc".into())
        );
        assert_eq!(
            unescape::<SingleQuoteString>(r#"ab\zc"#),
            Ok("ab\\zc".into())
        );
        assert_eq!(
            unescape::<SingleQuoteString>(r#" \"abc\" "#),
            Ok(" \\\"abc\\\" ".into())
        );
        assert_eq!(unescape::<SingleQuoteString>(r#"𝄞"#), Ok("𝄞".into()));
        assert_eq!(unescape::<SingleQuoteString>(r#"\𝄞"#), Ok("\\𝄞".into()));
        assert_eq!(
            unescape::<SingleQuoteString>(r#"\xD834\xDD1E"#),
            Ok("\\xD834\\xDD1E".into())
        );
        assert_eq!(
            unescape::<SingleQuoteString>(r#"\xD834"#),
            Ok("\\xD834".into())
        );
        assert_eq!(
            unescape::<SingleQuoteString>(r#"\xDD1E"#),
            Ok("\\xDD1E".into())
        );
        assert_eq!(unescape::<SingleQuoteString>("\t"), Ok("\t".into()));
    }

    #[test]
    fn test_unescape_double() {
        assert_eq!(unescape::<DoubleQuoteString>(r#"abc"#), Ok("abc".into()));
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"ab\nc"#),
            Ok("ab\nc".into())
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"ab\zc"#),
            Ok("ab\\zc".into())
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#" \"abc\" "#),
            Ok(" \"abc\" ".into())
        );
        assert_eq!(unescape::<DoubleQuoteString>(r#"𝄞"#), Ok("𝄞".into()));
        assert_eq!(unescape::<DoubleQuoteString>(r#"\𝄞"#), Ok("\\𝄞".into()));
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\u{1D11E}"#),
            Ok("𝄞".into())
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\xD834"#),
            Ok("\u{D8}34".into())
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\xDD1E"#),
            Ok("\u{DD}1E".into())
        );
        assert_eq!(unescape::<DoubleQuoteString>(r#"\xD"#), Ok("\u{D}".into()));
        assert_eq!(unescape::<DoubleQuoteString>("\t"), Ok("\t".into()));
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\u{D834"#),
            Err(UnescapeError)
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\uD834"#),
            Ok("\\uD834".into())
        );
        assert_eq!(unescape::<DoubleQuoteString>(r#"\u"#), Ok("\\u".into()));
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\47foo"#),
            Ok("'foo".into())
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\48foo"#),
            Ok("\u{4}8foo".into())
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\87foo"#),
            Ok("\\87foo".into())
        );

        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\u{999999}"#),
            Err(UnescapeError)
        );
        assert_eq!(
            unescape::<DoubleQuoteString>(r#"\u{999999999999999999}"#),
            Err(UnescapeError)
        );
    }
}
