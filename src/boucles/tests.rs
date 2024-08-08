use super::r#while::X;
use super::r#for::Array;

//Testing while
//Test type strut
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

//Testing for
//Struct test
#[test]
fn test_array(){
    let array = Array::new([1, 2, 3, 4, 5]);
    assert_eq!(array.elements, [1, 2, 3, 4, 5]);
}
//test fn iteration
fn test_iteration(){
    let array = Array::new([1, 2, 3, 4, 5]);
    array.iteration();
    assert_eq!(array.elements, [1, 2, 3, 4, 5]);
}