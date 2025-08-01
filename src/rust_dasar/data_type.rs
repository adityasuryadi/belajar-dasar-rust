/*
   scalar type
   - integer
   - float
   - boolean
   - char

   compound type
   - tuple => kumpulan beberapa data yang nilainya berbeda
   - array => kumpulan beberapa data tapi type datanya sama
*/

#[test]
fn explicit() {
    let age: i32 = 20;
    let name: &str = "Adit";
    println!("{} is {} years old", name, age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);
    let b: f32 = 10.51;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);
    let b: i32 = a as i32;
    println!("{}", b);
    let c: i32 = b as i32;
    println!("{}", c);
    let d: i64 = 1000000;
    let e: i8 = d as i8;
    println!("{}", e);
}
