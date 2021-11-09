# Indent text

Indent like a party

Rust implementation of https://cwestblog.com/2014/01/02/javascript-indenting-text/

# Examples

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn hello_newline_world() {
        assert_eq!("    hello\n    world", crate::indent::indent("hello\nworld", 1, 4));
    }

    #[test]
    fn newline_hello_newline_world() {
        assert_eq!(
            "\n    hello\n    world",
            crate::indent::indent("\nhello\nworld", 1, 4)
        );
    }

    #[test]
    fn hello_newline_world_indent_with_tab() {
        assert_eq!("\thello\n\tworld", crate::indent::indent("hello\nworld", 1, 0));
    }

    #[test]
    fn hello_newline_world_with_String() {
        assert_eq!(
            "    hello\n    world",
            crate::indent::indent("hello\nworld".to_string(), 1, 4)
        );
    }
}
```
