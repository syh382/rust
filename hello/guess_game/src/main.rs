use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("숫자 추리하기!");
    let secret_number:u32 = rand::thread_rng()
        .gen_range(1..=100);
    println!("{secret_number}");//실제론 지워야함
    loop {
        println!("추리한 숫자 입력.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("읽기 실패");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        println!("당신의 추리: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {println!("정답");break;}
            Ordering::Less => println!("더 큼"),
            Ordering::Greater => println!("더 작음")
        }
    }
}
