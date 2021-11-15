#![doc = include_str!("../README.md")]

use regex::Regex;

/// Indent text
///
/// # Examples
///
/// ```rust
/// fn hello_newline_world() {
///     assert_eq!("    hello\n    world", indentasy::indent("hello\nworld", 1, 4));
/// }
///
/// fn newline_hello_newline_world() {
///     assert_eq!(
///         "\n    hello\n    world",
///         indentasy::indent("\nhello\nworld", 1, 4)
///     );
/// }
///
/// fn hello_newline_world_indent_with_tab() {
///     assert_eq!("\thello\n\tworld", indentasy::indent("hello\nworld", 1, 0));
/// }
///
/// fn hello_newline_world_with_String() {
///     assert_eq!(
///         "    hello\n    world",
///         indentasy::indent("hello\nworld".to_string(), 1, 4)
///     );
/// }
/// ```
pub fn indent<S: AsRef<str>>(s: S, num_of_indents: usize, spaces_per_indent: usize) -> String {
    let s = s
        .as_ref()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let tmp;
            [
                if i > 0 { "\n" } else { "" },
                if !line.is_empty() {
                    tmp = vec![""; num_of_indents + 1].join("\t");
                    &tmp
                } else {
                    ""
                },
                line,
            ]
            .concat()
        })
        .collect::<String>();
    if spaces_per_indent < 1 {
        return s;
    }
    Regex::new(r"(?m)^\t+")
        .unwrap()
        .replace_all(&s, &vec![""; spaces_per_indent + 1].join(" ")[..])
        .into_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_newline_world() {
        assert_eq!("    hello\n    world", super::indent("hello\nworld", 1, 4));
    }

    #[test]
    fn newline_hello_newline_world() {
        assert_eq!(
            "\n    hello\n    world",
            super::indent("\nhello\nworld", 1, 4)
        );
    }

    #[test]
    fn hello_newline_world_indent_with_tab() {
        assert_eq!("\thello\n\tworld", super::indent("hello\nworld", 1, 0));
    }

    #[test]
    fn hello_newline_world_with_String() {
        assert_eq!(
            "    hello\n    world",
            super::indent("hello\nworld".to_string(), 1, 4)
        );
    }
}
