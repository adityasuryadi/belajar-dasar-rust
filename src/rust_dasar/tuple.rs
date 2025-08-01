/*
tuple adalah data kumpulan lebih dari satu tipe data
jumlah data di tuple sudah final,artinya tidak bisa berkurang atau berkembang
jika kita membuat tuple dengan total ada 3 data,maka tidak akan bisa di ubah lagi jumlah data dan juga tipe data nya
untuk membuat tuple,kita bisa guankan tanda kurung
*/

#[test]
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.05, true);
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("data {} {} {}", a, b, c);

    // destructuring tuple
    let (a, b, c) = data;
    println!("data {} {} {}", a, b, c);
}

#[test]
fn mutable_tuple() {
    let mut data = (10, 10.05, true);
    data.0 = 20;
    data.1 = 20.05;
    data.2 = false;
    println!("data {} {} {}", data.0, data.1, data.2);
}

fn unit() {
    println!("unit");
}

// tuple kosong
#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
}
