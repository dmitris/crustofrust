pub struct StrSplit<'a, 'b> {
    haystack: &'a str,
    delimiter: &'b str,
    pos: usize,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        if delimiter.len() == 0 {
            panic!("delimiter must be non-empty");
        }
        Self {
            haystack: haystack,
            delimiter: delimiter,
            pos: 0,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.haystack.len() {
            return None;
        }

        let new_pos = match skip_delims_prefix(&self.haystack[self.pos..], self.delimiter) {
            None => {
                // only delimiters left
                self.pos = self.haystack.len();
                return None;
            }
            Some(n) => self.pos + n,
        };
        self.pos = new_pos; // advance to skip past prefix out of [<delimiter>]*.
        let remainder = &self.haystack[self.pos..];
        match remainder.find(self.delimiter) {
            None => {
                // return the rest of the string
                self.pos = self.haystack.len();
                Some(&remainder)
            }
            Some(next_delimiter) => {
                self.pos += next_delimiter + self.delimiter.len(); // point past the delimiter
                Some(&remainder[..next_delimiter])
            }
        }
    }
}

/// Skips the (optional) prefix from one or multiple
/// delimiters and returns Some<n> with n pointing
/// to the first char past the delimiter prefix,
/// Some(0) if the haystack does not start with delim,
/// or None if the haystack is empty or consists of only the delimiters.
fn skip_delims_prefix(haystack: &str, delim: &str) -> Option<usize> {
    let mut p: usize = 0;
    while p < haystack.len() {
        if haystack[p..].starts_with(delim) {
            p += delim.len();
        } else {
            return Some(p);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_skip_delims_prefix() {
        let tests: Vec<(&str, &str, Option<usize>)> = vec![
            ("", "x", None),
            ("xxxxxxxxxxx", "x", None),
            ("abc d", "x", Some(0)),
            ("a b c", " ", Some(0)),
            (" a b", " ", Some(1)),
            ("  a", " ", Some(2)),
            ("  a a a", " ", Some(2)),
            (".......", ".", None),
            ("<<<<", "<<", None),
            ("<<<", "<<", Some(2)),
        ];
        for (i, (inp, del, res)) in tests.iter().enumerate() {
            let actual = skip_delims_prefix(inp, del);
            assert_eq!(
                actual, *res,
                "test #{i} failed: input={inp:?}, delimiter={del:?}, expected={res:?}, got={actual:?}"
            );
        }
    }

    #[test]
    fn it_works() {
        let haystack = "a bc def h";
        let v: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(vec!["a", "bc", "def", "h"], v);
    }

    #[test]
    fn strsplit() {
        let tests: Vec<(&str, &str, Vec<&str>)> = vec![
            ("abcd", "x", vec!["abcd"]),
            ("a|b||c|d|", "|", vec!["a", "b", "c", "d"]),
            ("foo::bar::", "::", vec!["foo", "bar"]),
            ("::::::foo::bar::", "::", vec!["foo", "bar"]),
            (":::foo::bar:::", "::", vec![":foo", "bar", ":"]),
            ("a::b", ":", vec!["a", "b"]),
        ];
        for (i, (inp, del, expected)) in tests.iter().enumerate() {
            let actual: Vec<&str> = StrSplit::new(inp, del).collect();
            assert_eq!(
                actual, *expected,
                "test #{i} failed: input={inp:?}, delimiter={del:?}, expected={expected:?}, got={actual:?}"
            );
        }
    }

    #[test]
    fn empty_str() {
        let haystack = "";
        let v: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(Vec::<&str>::new(), v);
    }

    #[test]
    fn only_delims() {
        let haystack = ".....";
        let delim = ".";
        let v: Vec<&str> = StrSplit::new(haystack, delim).collect();
        assert_eq!(Vec::<&str>::new(), v);
    }

    #[test]
    #[should_panic(expected = "delimiter must be non-empty")]
    fn empty_delimiter() {
        _ = StrSplit::new("abc", "");
    }

    #[test]
    fn lifetimes() {
        let haystack = ".....";
        let v: Vec<&str>;
        {
            let delim = ".".to_string();
            v = StrSplit::new(haystack, &delim).collect();
        }
        assert_eq!(Vec::<&str>::new(), v);
    }
}
