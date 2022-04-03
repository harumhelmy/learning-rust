use std::io;

fn main() {
    let number = 15;

    if number > 29 {
        println!("conditions were met")
    } else {
        println!("conditions were not met")
    }
}

fn _another_function(arg: i32) {
    println!("The value of arg is: {}", arg);
}


fn _another_main() {
    let _heart_eyed_cat = '😻'; // char type
    let tup: (u8, u8, &str) = (4, 16, "1990"); // tuple type

    println!("a is for april, the {}th month", tup.0);
    println!("check this out: {}", tup.2);

    // elements of a Rust array must have the same type
    // arrays also have a fixed length 
    // good for stuff when you know the number of elements will not need to change
    let _array_example: [&str; 3] = ["butt", "pants", "tulip"];

    let _array_of_threes = [3; 5]; // same as let a = [3, 3, 3, 3, 3]

    let array_to_play: [&str; 3] = ["Kacey Musgraves", "Anaïs Mitchell", "SZA"];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number, you disobeyed!");

    let element = array_to_play[index];

    println!("The value of the element at index {} is {}", index, element);
}