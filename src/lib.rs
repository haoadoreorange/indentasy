#![doc = include_str!("../README.md")]

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
    s.as_ref()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let tmp;
            [
                if i > 0 { "\n" } else { "" },
                if !line.is_empty() {
                    tmp = if spaces_per_indent < 1 {
                        vec![""; num_of_indents + 1].join("\t")
                    } else {
                        vec![""; num_of_indents + 1]
                            .join(&vec![""; spaces_per_indent + 1].join(" "))
                    };
                    &tmp
                } else {
                    ""
                },
                line,
            ]
            .concat()
        })
        .collect::<String>()
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
