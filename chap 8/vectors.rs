fn main(){
    let mut v: Vec<i32> = Vec::new();

    v.push(4);
    v.push(3);

    let s = vec!["name","course","batch"];
    let n = s[1];
    let f: &i32 = &v[1];

    println!("{:?} \n{:?} \n{} \n{} ",v,s,n,f);


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
}

