// vectors(arrays)

// funtion which filter and returns even values of the given vector
fn main() {
    let vec = vec![1, 2, 3, 4];

    println!("{:?}", vec);
    println!("Even elements of the vector: {:?}", filter_vec(vec));
}

fn filter_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    return new_vec;
}
