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

#[test]
fn static_typing() {
    /// di rust bersifat static typing jadi string. tidak boleh di assign ke integer
    let mut name = "Aditya";
    println!("Hello {}", name);

    name = "Adi";
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Aditya";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}
