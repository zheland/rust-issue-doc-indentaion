# rust-issue-doc-indentaion

## Rustdoc treats indentations in the following syntactically the same doc-comments and doc-attributes differently

According to <https://doc.rust-lang.org/rustdoc/the-doc-attribute.html>
the lines below are the same:
```rust
/// This is a doc comment.
#[doc = " This is a doc comment."]
```

But when mixing doc-comments and doc-attributes indentation in the next comment or attribute is ignored.

For example `rust doc` generate different doc pages for:
```rust
///Header
///
///    assert!(true);
pub fn func() {}
```
and:
```rust
///Header
///
#[doc = "    assert!(true);"]
pub fn func() {}
```

where `"____assert!(true);"` become `CodeBlock` for first function ([example](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod1/fn.func.html)), but `"____assert!(true);"` become plain text paragraph for second function ([example](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod3/fn.func.html)).

This behavior might be usefull for saving identical indents in documentation declared in comments and attributes:
```rust
/// Some docs
#[cfg_attr(feature = "foo", doc = #"
    continuation with visually the same indentation
"#)]
/// continuation with visually the same indentation
```
but it is not naturally expected and explicit.

For example [syn crate](https://crates.io/crates/syn) produce exactly the same output for when parsing docs for `fn foo()` and `fn bar()`.

If this behavior is intentional it should be docummented.

Examples are available at links:
[Source with doc-comments only](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/src/mod1.rs),
[Generated docs with CodeBlock](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod1/fn.func.html),
[Source with both doc-comments and doc-attributes](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/src/mod3.rs),
[Generates docs without CodeBlock](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod3/fn.func.html).
