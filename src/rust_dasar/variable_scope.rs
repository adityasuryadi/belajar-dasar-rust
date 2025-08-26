// variable hanya di gunakan di dalam scope

#[test]
fn variable_scope() {
    let adit = 1;

    {
        //iner scope
        println!("inner adit {}", adit);
        let tya = 2;
        println!("inner tya {}", tya);
    }

    // tidak bisa akses variable tya karena di luar scope
    // println!("inner tya: {}", tya);
}
