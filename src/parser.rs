pub struct Parser<'a> {
    // Assume that Parser parses ASCII.
    state: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { state: input }
    }

    /// Read a first character in the state and advance state.
    pub fn read(&mut self) -> Option<char> {
        self.state.split_first().map(|(&b, tail)| {
            self.state = tail;
            char::from(b)
        })
    }

    /// Read a first character in the state, but the state is not modified.
    pub fn peek(&self) -> Option<char> {
        self.state.split_first().map(|(&b, _)| char::from(b))
    }

    /// Read until `target` appears and return string composed of bytes read so far.
    /// It does not include `target`.
    pub fn read_until(&mut self, target: char) -> String {
        let mut read_bytes = Vec::new();
        while let Some(c) = self.read() {
            if c == target {
                break;
            }
            read_bytes.push(c);
        }
        read_bytes.iter().collect()
    }

    /// Read until whitespace(' ') appears.
    pub fn read_until_whitespace(&mut self) -> String {
        self.read_until(' ')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_char() {
        let mut p = Parser::new(&[42, 43]);
        assert_eq!(Some('*'), p.read());
        assert_eq!(Some('+'), p.read());
        assert_eq!(None, p.read());
    }

    #[test]
    fn peek_char() {
        let p = Parser::new(&[42, 43]);
        assert_eq!(Some('*'), p.peek());
    }

    #[test]
    fn read_until_delim() {
        let bytes = "GET /index.html HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        assert_eq!("GET", p.read_until(' '));
        assert_eq!("/index.html", p.read_until(' '));
        assert_eq!("HTTP/1.1", p.read_until('\r'));
    }
}
