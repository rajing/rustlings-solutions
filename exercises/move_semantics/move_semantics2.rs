// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {

    // Option 1
    // let vec0 = Vec::new();
    // let vec_cloned = vec0.clone();
    // let mut vec1 = fill_vec(vec_cloned);

    // Option 2
    // let vec0 = Vec::new();
    // let mut vec1 = fill_vec(&vec0);

    // Option 3
    let mut vec0 = Vec::new();
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // Option 1 & 2
    // vec1.push(88);
    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // Option 3
    vec0.push(88);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

// Option 1
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;
//
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//
//     vec
// }

// Option 2
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.clone();
//
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//
//     vec
// }

// Option 3
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    
}
