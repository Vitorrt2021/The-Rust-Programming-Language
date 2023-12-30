


#[test]
fn reference(){
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn box_reference(){
//  The main difference between this and reference is that here we set y to be an instance of a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}