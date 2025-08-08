/*
array di rust bersifat statis,tidak dinamis seperti php atau js

*/

#[test]
fn array_test() {
    // jika ingin mengubah value nya gunakan mut (dibuat mutable)
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    // hitung panjang array
    let length = array.len();
    println!("panjang array {}", length);
}

#[test]
fn two_dimension_array() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 1], [3, 4, 3]];

    println!("{:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}
