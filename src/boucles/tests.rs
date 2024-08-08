use super::r#loop::X;

//Test type struct
#[test]
fn test_new() {
    let x = X::new(0);
    assert_eq!(x.n, 0);
}

//test case one
#[test]
fn test_while_a() {
    let mut x = X::new(0);
    x.while_a();
    assert_eq!(x.n, 10);
}

//test max condition
#[test]
fn test_while_max() {
    let mut x = X::new(10);
    x.while_a();
    assert_eq!(x.n, 10);
}

//test even case
#[test]
fn test_while_modulo() {
    let mut x = X::new(0);
    x.while_modulo();
    assert_eq!(x.n, 10);
}