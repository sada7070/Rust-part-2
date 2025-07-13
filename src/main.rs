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

// filter
fn main() {
    let nums = vec![1, 2, 3, 4];
    let iter = nums.iter();

    let iter2 = iter.filter(|x| *x % 2 == 0);   // select x if x % 2 == 0    (*x == -> if x ==)
    for x in iter2 {
        println!("{}", x);
    }
    println!("{:?}", nums);
}
