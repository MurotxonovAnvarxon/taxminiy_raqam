use std::cmp::Ordering;
use std::io;
use rand::RngExt;

fn main() {
    println!("🔢  Raqamni topish o'yini! 🔢");

    let yashirin_raqam=rand::rng().random_range(1..=100);

    loop{
        println!("*********** Iltimos taxminingizni kiriting **********");
        let mut taxmin=String::new();

        io::stdin()
            .read_line(&mut taxmin)
            .expect("Satrni o'qib bo'lmadi");

        match taxmin.trim().as_ref() {
            "quit" => {
              break_program();
                break;
            }
            _ => {
            }
        }

        let taxmin:u32= match taxmin.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Satrni o'qib bo'lmadi");
                continue;
            },
        };

        println!("Sizning taxminingiz: {taxmin}");

        match taxmin.cmp(&yashirin_raqam){
            Ordering::Less=>println!("Raqam kichik!"),
            Ordering::Greater=>println!("Raqam katta!"),
            Ordering::Equal=>{
                println!("🥳  Siz yutdingiz! 🥳");
                break;
            }
        };
    }


}

fn break_program(){
    println!("😔  Dastur yakunlandi 😔");
}
