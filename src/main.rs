use std::fmt::format;

fn main() {
    let mut vec = Vec::new();
    vec.push(String::from("Babalola Opeyemi"));
    vec.push(String::from("Ajoke"));
    vec.push(String::from("Test"));


//
    //Create vector using macros
//    let  v2 = vec![1, 2, 3, 4];
    //pass value
//     vec[1]  = String::from("Ajoke Ibrahim");

    //use get
//    let  d = vec.get(1);

//    if let Some(val)  = dan  {
//        println!("{val}")
//    }

    //loop

    for i in &mut vec {
        i.push_str("!");
    }

//    for j in &mut vec {
//        println!("{j}")
//    }

    let  mut v3 = vec![];

    for k in vec.into_iter() {
        v3.push(k)
    }

    for i in &mut v3 {
        println!("{i}")
    }


}
