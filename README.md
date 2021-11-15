# Indentasy

Indent like a party

Rust implementation of https://cwestblog.com/2014/01/02/javascript-indenting-text/

# Examples

```rust
fn hello_newline_world() {
    assert_eq!("    hello\n    world", indentasy::indent("hello\nworld", 1, 4));
}

fn newline_hello_newline_world() {
    assert_eq!(
        "\n    hello\n    world",
        indentasy::indent("\nhello\nworld", 1, 4)
    );
}

fn hello_newline_world_indent_with_tab() {
    assert_eq!("\thello\n\tworld", indentasy::indent("hello\nworld", 1, 0));
}

fn hello_newline_world_with_String() {
    assert_eq!(
        "    hello\n    world",
        indentasy::indent("hello\nworld".to_string(), 1, 4)
    );
}
```
