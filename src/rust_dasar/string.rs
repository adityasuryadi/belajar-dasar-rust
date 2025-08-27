/*
rust memiliki tipe data text yang fixed size,yaitu &str (string slice),dan yang bisa mengembang ukuran nya,yaitu String

&str karena ukurannya fixed size,jadi rust akan menyimpan di stack,sedangkan string karena bisa mengembang,maka di simpan di heap

String di simpan di heap dan bisa berkembang datanya


*/

#[test]
fn string() {
    // ini string slice
    // data tidak akan di ganti melainkan membuat data baru dan di simpan di stack
    let name: &str = "  Aditya. ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    // ini type string
    // bisa mengubah data dan membuat string baru
    let username: String = String::from("Aditya Suryadi");

    // ini membuat string baru
    let budi = username.replace("Aditya", "Budi");
    println!("{}", username);
    println!("{}", budi);
}
