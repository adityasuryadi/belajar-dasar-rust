/*
di rust ga ada garbage collection seperti golang atau java
garbage collection akan menghapus memory pada saat limit tertentu

berebeda dengan rust, rust tidak menggunakan manual atau garbage collection
managemenr memory di rust di bagi ke 2 bagian ke heap dan stack

stack (tumpukan) : bagian data akan di simpan dalam struktur tumpukan
bersifat LIFO last in first out
tipe data yang di simpan data yang fixed


Heap untuk menyimpan data
data yang di simpan tidak fixed seperti String bisa mengecil dan membesar
data di heap di beri pointer

*/

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;

    // String::from("Adit"); ini buat di alokasikan heap
    let b = String::from("Adit");

    println!("{},{}", a, b);
}

fn function_b() {
    let a = 20;

    // String::from("Adit"); ini buat di alokasikan heap
    /*
    &str gunakan ini buat fixed ,ini bersifat imuutable
    String gunakan untuk data dinamis
     */
    let b = String::from("Tya");

    println!("{},{}", a, b);
}
