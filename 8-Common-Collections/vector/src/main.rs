fn main() {
    create();
    update();
    read();
}

fn create(){
    let init: Vec<i32> = Vec::new();
    let values = vec![1, 2, 3];

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let different_types = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn update(){
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
}

fn read(){
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    let immutable = vec![100, 32, 57];
    for i in &immutable {
        println!("{i}");
    }

    let mut mutable = vec![100, 32, 57];
    for i in &mut mutable {
        *i += 50;
    }

}