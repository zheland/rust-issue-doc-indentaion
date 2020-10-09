# rust-issue-doc-indentaion

## Rustdoc treats indentations in the consecutive syntactically same doc-comments and doc-attributes differently

According to <https://doc.rust-lang.org/rustdoc/the-doc-attribute.html>
the lines below are the same:
```rust
/// This is a doc comment.
#[doc = " This is a doc comment."]
```

But when mixing doc-comments and doc-attributes indentation in the next comment or attribute is ignored.

For example `rust doc` generate different doc pages for
```rust
///Header
///
///    assert!(true);
pub fn func() {}
```
and
```rust
///Header
///
#[doc = "    assert!(true);"]
pub fn func() {}
```

where `///____assert!(true);` produce `CodeBlock` for the first function ([generated docs](https://htmlpreview.github.io/?https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod1/fn.func.html)), but `#[doc = "____assert!(true);"]` produce plain text paragraph for the second function ([generated docs](https://htmlpreview.github.io/?https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod3/fn.func.html)).

This behavior might be usefull for saving identical indents in documentation declared in comments and attributes:
```rust
/// Some docs
#[cfg_attr(feature = "foo", doc = #"
    continuation with visually the same indentation
"#)]
/// continuation with visually the same indentation
```
but in general this behavior is not explicit.

For example [syn crate](https://crates.io/crates/syn) produce exactly the same output when parsing docs both functions with `syn::parse_file` and preserves all the spaces.

If this behavior is intentional it should be docummented.

More examples:
* [File with doc-comments only](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/src/mod1.rs) that [generates docs with CodeBlock](https://htmlpreview.github.io/?https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod1/fn.func.html),
* [File with doc-attributes only](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/src/mod2.rs) that [generates docs with CodeBlock](https://htmlpreview.github.io/?https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod2/fn.func.html),
* [File with both doc-comments and doc-attributes](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/src/mod3.rs) that [generates docs without CodeBlock](https://htmlpreview.github.io/?https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod3/fn.func.html).
* [File with both doc-attributes and doc-comments](https://github.com/zheland/rust-issue-doc-indentaion/blob/master/src/mod4.rs) that [generates docs without CodeBlock](https://htmlpreview.github.io/?https://github.com/zheland/rust-issue-doc-indentaion/blob/master/doc/rust_issue_doc_indentaion/mod4/fn.func.html).

Reproduced with `cargo doc` on `rustc 1.47.0 (18bf6b4f0 2020-10-07)` and `rustc 1.49.0-nightly (91a79fb29 2020-10-07)`
