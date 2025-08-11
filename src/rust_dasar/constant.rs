const MAXIMUM: i32 = 100;

/*
konstant bersifat imutable
di sarankan uppercase dan snake case
harus explisit
*/
#[test]

fn test_const() {
    const MINIMUM: i32 = 10;

    println!("{} {}", MAXIMUM, MINIMUM);
}
