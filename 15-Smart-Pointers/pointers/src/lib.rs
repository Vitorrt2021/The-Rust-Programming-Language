


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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn use_custom_smart_pointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    drop(c);
    println!("Force drop of CustomSmartPointer") ;
}