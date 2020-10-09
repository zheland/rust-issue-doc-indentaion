use std::path::Path;

fn file_ast(path: &Path) -> syn::File {
    let content = std::fs::read_to_string(path).unwrap();
    syn::parse_file(&content).unwrap()
}

#[cfg(test)]
#[test]
fn self_test() {
    let mod1_ast = file_ast(Path::new("src/mod1.rs"));
    let mod2_ast = file_ast(Path::new("src/mod2.rs"));
    let mod3_ast = file_ast(Path::new("src/mod3.rs"));
    let mod4_ast = file_ast(Path::new("src/mod4.rs"));
    assert_eq!(mod1_ast, mod2_ast);
    assert_eq!(mod2_ast, mod3_ast);
    assert_eq!(mod3_ast, mod4_ast);
}
