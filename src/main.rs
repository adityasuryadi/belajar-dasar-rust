mod rust_dasar;
use axum::{extract::State, routing::get, serve, Router};
use clap::{Parser, Subcommand};
use std::{fmt::format, result, sync::Arc, thread, time::Duration};
use tokio::{net::TcpListener, sync::Notify};

/*
buat command line interface parsesr
menggunakan package https://crates.io/crates/clap
*/
// #[derive(Parser)]
// #[command(name = "Persegi", version, about = "Hitung luas persegi", long_about = None)]
/*

short: Mengizinkan opsi pendek, seperti -s.

long: Mengizinkan opsi panjang, seperti --sisi.

*/

// struct Args {
//     /// Panjang sisi persegi
//     #[arg(short, long)]
//     sisi: u32,
// }

#[derive(Parser)]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Worker,
    Run,
}

pub struct AppState {
    notify: Arc<Notify>,
}
#[tokio::main]
async fn main() {
    // let args = Args::parse();

    // let luas = args.sisi * args.sisi;
    // println!("Luas persegi dengan sisi {} adalah {}", args.sisi, luas);

    let args = App::parse();
    let notify = Arc::new(Notify::new());
    let state = Arc::new(AppState {
        notify: notify.clone(),
    });
    match args.command {
        Command::Worker => {
            // Clone untuk worker
            let notify_worker = notify.clone();

            // Worker async task
            tokio::spawn(async move {
                loop {
                    println!("🔄 Worker menunggu sinyal...");
                    notify_worker.notified().await; // Tunggu sinyal
                    println!("✅ Worker menerima sinyal dan mulai bekerja...");
                    // Simulasikan pekerjaan
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    println!("🏁 Pekerjaan selesai.");
                }
            });
            println!("🚀 Worker berjalan. Tekan Ctrl+C untuk keluar.");
            // Tahan agar program tetap hidup
            tokio::signal::ctrl_c().await.unwrap();
            println!("🛑 Program dihentikan.");
        }
        Command::Run => {
            // untuk worker
            let notify_worker = notify.clone();

            // Worker async task
            tokio::spawn(async move {
                loop {
                    println!("🔄 Worker menunggu sinyal...");
                    notify_worker.notified().await; // Tunggu sinyal
                    println!("✅ Worker menerima sinyal dan mulai bekerja...");
                    // Simulasikan pekerjaan
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    println!("🏁 Pekerjaan selesai.");
                }
                // println!("🚀 Worker berjalan. Tekan Ctrl+C untuk keluar.");
                // Tahan agar program tetap hidup
                // tokio::signal::ctrl_c().await.unwrap();
                // println!("🛑 Program dihentikan.");
            });

            let app_url = "127.0.0.1:3000";
            let app = Router::new()
                .route("/test", get(test_notify))
                .with_state(state);
            let listener = TcpListener::bind(app_url.clone()).await.unwrap();
            println!("Listening on {:}", app_url.clone());
            serve(listener, app).await.unwrap();
        }
    }
}

pub async fn test_notify(State(state): State<Arc<AppState>>) {
    let notify = &state.notify;
    notify.notify_one();
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

// variable
/*
di rust bersifat imutable
jika ingin mengubah jadi mutable tambah kan mut
 */
#[test]
fn test_variable() {
    let mut name = "Adit";
    name = "adi";
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Adit";
    println!("Hello {}", name);

    // ini bisa karena karena beda memory
    // tapi yang di akses hanya yg terakhir

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn variable_scope() {
    let adi = 1;
    {
        //inner scope
        println!("inner adit {}", adi);
        let tya = 2;
        println!("inner tya {}", tya);
    }
    // variable tya tidak bisa diakses karena di luar scope
    // println!("inner tya: {}", tya);
}

/*
    data fix di simpan di stack seperti arrau,int
    sedangkan data heap untuk menyimpan data yang tidak fixed,seperti string
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

// ownership

/*
setiap vaue di rust harus ounya owner (variable pemilik value)
dalam satu waktu,hanya boleh ada satu owner
ketika owner keluar scope,value akan di hapus

transfer ownersship di stack copy data
transfer ownership di heap move data
*/

#[test]
fn transfer_ownership() {
    let nama1 = String::from("Adit");
    let nama2 = nama1;

    // karena ini nama1 heap trus di transfer ownership nya ke nama2 maka nama 1 tidak bisa diakses lagi
    // println!("{}", nama1);
    print!("nama saya {}", nama2);
}

#[test]
fn clone() {
    let nama1 = String::from("Adit");

    // clone untuk menduplikat data,ini berbeda dengan ownership jadi jika nama1 misalnya 10 mb trus di clone jadi duplicate total nya jadi 20mb

    let nama2 = nama1.clone();

    // karena ini nama1 heap trus di transfer ownership nya ke nama2 maka nama 1 tidak bisa diakses lagi
    // println!("{}", nama1);
    print!("nama 1 {} nama 2 {}", nama1, nama2);
}

// reference and borrowing
/*
    refferences tidak boleh di modifikasi
    refference adalah pointer
    refferences boleh di buat lebih dari 1 tapi owner nya harus satu

    borrowing
    aksi reference disebut borrowing
    jika ingin memodifikasi value dari refference gunakan mutable reference
*/
#[test]

fn test_full_name() {
    let first_name = String::from("Adit");
    let last_name = String::from("Tya");

    let full_name = full_name(&first_name, &last_name);
    println!("full name {}", full_name);
}
fn full_name(first_name: &String, last_name: &String) -> String {
    let fullname = format!("{} {}", first_name, last_name);
    fullname
}

// test borrowing
#[test]
fn test_change_value() {
    let mut value = String::from("Adit");

    let value1 = &mut value;
    // let value2: &mut String = &mut value; ini tidak boleh karena mutable hanya boleh ada 1 referenece ke data yang sama

    change_value(value1);
    change_value(value1);
    change_value(value1);
    // change_value(value2);
    // change_value(value);
    // change_value(value);
}

fn change_value(value: &mut String) {
    value.push_str("Test");
    print!("{}", value);
}

// dangling pointer
/*
tidak boleh return refference
gunakan return biasa
*/
#[test]
fn test_full_name_dangling() {
    let first_name = String::from("Adit");
    let last_name = String::from("Tya");

    let full_name = get_full_name(&first_name, &last_name);
    println!("full name {}", full_name);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    full_name
}

// lifetime
/* 'a merupkan antotation lifetime
    menggunakan huruf kecil
    jika tidak menggunakan life anotation akan error,akan bigung menentukan reference nya
*/
fn longest<'a>(value: &'a str, value2: &'a str) -> &'a str {
    if value.len() > value2.len() {
        value
    } else {
        value2
    }
}

#[test]
fn test_lifetime_anotation() {
    let value1 = "Adit";
    let value2 = "Tya";
    let result = longest(value1, value2);
    println!("{}", result);
}

#[test]
fn test_lifetime_anotation_dangling_reference() {
    let value1 = String::from("Adit");
    let value2 = String::from("Tya");
    let result;
    {
        // as_str() untuk mengubah string ke &str
        result = longest(value1.as_str(), value2.as_str());
    }
    println!("{}", result);
}

// attribute
#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute() {
    let company = Company {
        name: "google".to_string(),
        location: "indonesia".to_string(),
        website: "google.com".to_string(),
    };
    println!("{:?}", company)
}
