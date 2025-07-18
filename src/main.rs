// // vectors(arrays)

// // funtion which filter and returns even values of the given vector
// fn main() {
//     let vec = vec![1, 2, 3, 4];

//     println!("{:?}", vec);
//     println!("Even elements of the vector: {:?}", filter_vec(vec));
// }

// fn filter_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();

//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(val);
//         }
//     }

//     return new_vec;
// }



// // Hashmaps(key value pair)
// // Methods in Hashmap: insert, get, remove, clear

// use std::collections::HashMap;

// fn main() {
//     let mut users = HashMap::new();

//     // insert
//     users.insert(String::from("Sada"), 23);
//     users.insert(String::from("Shiva"), 18);

//     //get
//     let user1 = users.get("Shiva");         // we should use Options here so that if the key does not exist we will get suitable message.

//     match user1 {
//         Some(value) => println!("{}", value),
//         None => println!("Key does not exist."),
//     }
// }


// // create a funtion group_values_by_keys where you will get vector with values as tuple(string, i32) and return the hashmap with tuple values init

// use std::collections::HashMap;

// fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
//     let mut hashmap = HashMap::new();

//     for (key, value) in vec {
//         hashmap.insert(key, value);
//     }

//     return hashmap;
// }

// fn main() {
//     let vec = vec![(String::from("Sada"), 23), (String::from("Shiva"), 83)];

//     let hashmap = group_values_by_keys(vec);

//     println!("{:?}", hashmap);
// }




// Iterators

// // Iterating using for loop                                         // even_num will take the ownership of the original values
// fn main() {
//     let nums = vec![1, 2, 3];
//     let mut even_num = Vec::new();

//     for val in nums {
//         if val % 2 == 0 {
//             even_num.push(val);
//         }
//     }
//     println!("{:?}", even_num);
// }

// // Iterating after creating an 'iterator'(immutable)                 // iter will not take the ownership of the original values
// fn main() {
//     let nums = vec![1, 2, 3];
//     let iter = nums.iter();

//     for values in iter{
//         println!("{}", values);
//     }
//     println!("{:?}", nums);
// }

// // Iterating after creating an 'iterator'(mutable)                   // iter will not take the ownership of the original values
// fn main() {
//     let mut nums = vec![1, 2, 3];
//     let iter = nums.iter_mut();

//     for values in iter {
//         println!("{}", *values + 1);
//     }
//     println!("{:?}", nums);
// }

// // Iterating using '.next()'(both mutable and unmutable)             // iter will not take the ownership of the original values
// fn main() {
//     let nums = vec![1, 2, 3];
//     let mut iter = nums.iter();

//     while let Some(val) = iter.next() {
//         println!("{}", val + 1);
//     }
//     println!("{:?}", nums);
// }

// // Iteating using 'into_iter()'                                         // iter will take the ownership of the original values(this is same as writing 'for loop')
// fn main() {
//     let nums = vec![1, 2, 3];
//     let iter = nums.into_iter();

//     for values in iter {
//         println!("{}", values);
//     }
// }


// // consumig adoptors in Iterators
// // we get some built in functions with the use of iter() which are depend on iter but not on original values

// fn main() {
//     let nums = vec![1, 2, 3];
//     let iter = nums.iter();

//     let sum2: i32 = iter.sum();
    
//     println!("Sum of numbers = {}", sum2);

//     //let sum2: i32 = iter.sum();   here tryied to use iter again but it is already consumed by the sum1
// }


// iterator adoptors
// they do not consume iterators as in consume adoptors, instead they produce different iterators by changing some aspects of the original iterator
// they have map, filter as inbuilt functions

// // map
// fn main() {
//     let nums = vec![1, 2, 3];
//     let iter = nums.iter();

//     let iter2 = iter.map(|x| x + 1);
    
//     for x in iter2 {
//         println!("{}", x);
//     }
//     println!("{:?}", nums)
// }

// // filter
// fn main() {
//     let nums = vec![1, 2, 3, 4];
//     let iter = nums.iter();

//     let iter2 = iter.filter(|x| *x % 2 == 0);   // select x if x % 2 == 0    (*x == -> if x ==)
//     for x in iter2 {
//         println!("{}", x);
//     }
//     println!("{:?}", nums);
// }


// // assignment: write the logic to first filter all odd values then double each value and create a new vector
// fn main() {
//     let nums = vec![1, 2, 3, 4, 5, 6];
//     let iter = nums.iter();

//     let iter2 = iter.filter(|x| *x % 2 != 0).map(|x| x * 2);

//     let new_vec: Vec<i32> = iter2.collect();
//     println!("{:?}", new_vec);
// }




// strings and slices

// // strings
// fn main() {
//     let mut str = String::from("Sada");                                  // string initialization
//     println!("{}", str);

//     str.push_str("shiva");                                               // string mutation
//     println!("{}", str);

//     str.replace_range(4..str.len(), "");                                 // edit/delete string
//     println!("{}", str);   
// }

// // string
// fn main(){
//     let full_name = String::from("Sada Shiva");
//     let first_name = first_name(full_name);
//     println!("{}", first_name);
// }

// fn first_name(full_name: String) -> String {
//     let mut first_name = String::from("");

//     for char in full_name.chars() {
//         if char == ' ' {
//             break;
//         }
//         first_name.push_str(&char.to_string());
//     }

//     return first_name;
// }

// // the above code creates new string to store first_name, we dont want it. We want to crete a view to the first_name from full_name.
// // solution

// slices
// fn main() {
//     let full_name = String::from("Sada Shiva");
//     let first_name = first_name(&full_name);
//     println!("{}", first_name);
// }

// fn first_name(full_name: &String) -> &str {
//     let mut space_index = 0;

//     for val in full_name.chars() {
//         if val == ' '{
//             break;
//         }
//         space_index += 1;
//     }

//     return &full_name[0..space_index];
// }




// // Generics
// fn main() {
//     let largest_num = largest(1, 2);
//     let largest_char = largest('a', 'b');

//     println!("largest number: {}", largest_num);
//     println!("largest character: {}", largest_char);
// }

// fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }




// // Traits
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub trait Fix {
//     fn fix(&self) -> String {
//         return String::from("Hi");
//     }
// }

// struct User {
//     name: String,
//     age: u32,
// }

// impl Summary for User{
//     fn summarize(&self) -> String {
//         return format!("Name: {}, age: {}", self.name, self.age);
//     }
// }

// impl Fix for User {}

// fn notify<T: Summary + Fix>(u: T) {
//     print!("{},  {}", u.fix(), u.summarize());
// }

// fn main() {
//     let user1 = User {
//         name: String::from("Sada"),
//         age: 23,
//     };
//     notify(user1);
// }




// Lifetimes

// fn longest_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// fn main() {
//     let longest_str;
//     let str1 = String::from("Sada");
//     {
//         let str2 = String::from("Shiva");
//         longest_str = longest_string(&str1, &str2);
//     }
//     println!("{}", longest_str);
// }

// // Structs with Lifetimes
// struct User<'a> {
//     name: &'a str,
// }

// fn main() {
//     let name = String::from("Sada");
//     let user1 = User {
//         name: &name,
//     };

//     println!("{}", user1.name);
// }





// // Multithreading

// use std::thread;
// use std::time::Duration;
// fn main() {
//     thread::spawn(|| {
//         for i in 0..100000 {
//             println!("Hi, number {i} from spwan thread.");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     //handle.join().unwrap();

//     for i in 0..50000 {
//         println!("Hello, number {i} from main thread.");
//         thread::sleep(Duration::from_millis(1));
//     }
// }




// Message Parsing                                                      // transfering messages from one thread to another

// use std::sync::mpsc;                                                 // mpsc  => Multi Producer Single Consumer
// use std::thread;
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move|| {
//         let val = String::from("Hi");
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("{}", received);
// }

// assignment: write multi-threded code that utilizes all core to calculate values from 1 to 7*2^10(my pc is 7 core)
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..7 {
        let producer = tx.clone();              // we are using clone of 'tx'
        thread::spawn(move|| {
            let mut ans: i64 = 0;
            for j in 0..10000000 {
                ans = ans + (i * 10000000 + j); 
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);

    let mut ans = 0;

    // this for loop will wait until all 'tx' finishes. Since we are not using original 'tx', we are droping it at the end of loop.
    for val in rx {
        ans = ans + val;
        println!("Recieved value");
    }

    println!("Total sum = {}", ans);
}