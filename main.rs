use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
loop {
    println!("数あてクイズです。!");
  println!("1から30までの数です。10回以内に当たればあなたの勝ちです。!");
    let secret_number = rand::thread_rng().gen_range(1, 31);

    println!("The secret number is: {}", "???");
let mut s=0;

 loop {
        println!("あなたが予想した数を言ってください。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {println!("小さすぎます。!");
s=s+1;
if s>10{
println!("貴方の負けです。!");
println!("The secret number is: {}", secret_number);
for  _i  in  1..10{
println!("{}","*");}

println!("{}","ーーーーーーーーーーーーーー");

break;
}}

            Ordering::Greater => {println!("大きすぎます。!");
s=s+1;
if s>10{
println!("貴方の負けです。!");
println!("The secret number is: {}", secret_number);
break;
for  _i  in  1..10{
println!("{}","*");}

println!("{}","ーーーーーーーーーーーーーー");

}}
            Ordering::Equal => {
                println!("貴方の勝ちです。!");
println!("The secret number is: {}", secret_number);
{
for  _i  in  1..10{
println!("{}","*");}

println!("{}","ーーーーーーーーーーーーーー");

               break;
            }}
       }} 
    
}}