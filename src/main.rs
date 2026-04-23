// ------------- Basic variables with explicit types
// fn main() {
//     let x: i32 = 5;    // signed 32‑bit
//     let y: u32 = 1000; // unsigned 32‑bit
//     let z: f32 = 1001.23; // 32‑bit float
//
//     println!("x = {}", x);
//     println!("y = {}", y);
//     println!("z = {}", z);
// }

// ---------------------------------------------
// ------------- Working with String and chars
// fn main() {
//     let s: String = String::from("Rohit is good");
//     println!("s = {}", s);
//
//     // 3rd character (0‑based); Option<char> in case index is out of bounds
//     let charr: Option<char> = s.chars().nth(2);
//     println!("charr = {:?}", charr);
// }

// ---------------------------------------------
// ------------- Simple if/else with a bool
// fn main() {
//     let is_even: bool = true;
//
//     if is_even {
//         println!("The number is even");
//     } else {
//         println!("The number is odd");
//     }
// }


//---------------------------------------------
// ------------- For loop + helper function example
// fn main() {
//     // 0..5 gives 0,1,2,3,4
//     for x in 0..5 {
//         println!("x = {}", x);
//     }

//     let sentence: String = String::from("This is Rohit");

//     // Get first word from the sentence
//     let _first_word = get_first_word(sentence);

//     print!("first word = {}", _first_word);
// }

// // ** Returns characters up to the first space (including the space) **
// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::from("");

//     for ch in sentence.chars() {
//         ans.push(ch);
//         if ch == ' ' {
//             break;
//         }
//     }

//     ans
// }


//---------------------------------------------
// ------ Struct example
struct User {
    name: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        email: String::from("rohit@gmail.com"),
        name: String::from("Rohit"),
        active: true,
        sign_in_count: 1,
    };
    println!(
        "UserName: {} , Email: {} , Active: {} , Sign In Count: {}",
        user1.name, user1.email, user1.active, user1.sign_in_count
    );
}

