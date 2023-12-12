fn main() {
    try_panic()

}

fn force_panic(){
    panic!("crash and burn");
}


fn try_panic(){
    let v = vec![1, 2, 3];

    v[99];
}