#[test]
fn test_variable() {
    let name = "Aditya";

    /// name = "adi"; ini error karena ga mutable

    println!("Hello {}", name);
}

#[test]
fn test_mutable_variable() {
    // mutable berarti variable dapat di assign
    let mut name = "Aditya";
    name = " Adi";

    println!("Hello {}", name);
}
