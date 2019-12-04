use std::collections::HashMap;

fn main(){
    let mut age = HashMap::new();

    age.insert("std 1",16);
    age.insert("std 2",18);
    age.insert("std 3",20);
    age.insert("std 4",19);
    age.insert("std 4",17);
    
    println!("{:?}",age );
    println!("{}",age["std 1"] );

    for (key , values) in &age{
        println!("{} {}",key,values )
    }



}