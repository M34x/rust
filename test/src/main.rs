// // const TAX: f64 = 0.13;
// fn double(x: i32) -> i32 {
//     let y = x * 2;
//     if (y > x) {
//         return x * 2;
//     } else {
//         return x;
//     }
// }

// fn average(scores: &[i32]) -> (i32, i32, f64){
//     let mut min = scores[0];
//     let mut max = scores[scores.len()];
//     let total = scores.len();
//     let mut sum = 0;
//     for s in scores {
//         if *s < min {min = *s}
//         if *s > max {max = *s}
//         sum += s;
//     }
//     let avg = sum as f64 / total as f64;
//     (min, max, avg)
// }
use std::{io::{self, Write}};
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
fn input<'a>(msg:&'a str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();
    let mut q = String::new();
    io::stdin().read_line(&mut q).expect("Invalid number");
    q.trim().to_string()
}
fn int(str:String) -> i128 {
    let x = str.trim().parse().unwrap_or(0);
    if x != 0 || str == "0" {
        return x
    }
    println!("Invalid number, returning 0");
    return x
}
fn float(str:String) -> f64 {
    let x = str.trim().parse::<f64>().unwrap() as f64;
    if x != 0.0 || str == "0" {
        return x
    }
    println!("Invalud number, returning 0");
    return x
}
fn bin(mut num:u128) -> String {
    if num == 0 {
        return String::from("0");
    }
    let mut string = Vec::new(); 
    while num > 0 {
        string.push(if num % 2 == 0 {'0'} else {'1'});
        num /= 2;
    }
    return string.iter().rev().collect()
}
fn main() {
    // let mut total = 0.0;
    // let mut purchases: Vec<f64> = vec![1.0, 2.0, 3.0];
    // purchases.push(5.0);
    // for item in purchases {
    //     total += item;
    // }
    // let total_tax = TAX * total; 
    // total = total + total_tax;
    // println!("the total is {total}");


    // let a:i8 = 7;
    // let b:i8 = 2;
    // println!("{}", a / b);
    // println!("{}", a as f64 / b as f64);
    // let x:u8 = 255;
    // println!("{}", x.wrapping_add(1));
    // println!("{}", x.saturating_add(1));
    // println!("{:?}", x.checked_add(1));
    // let f = 3.99_f64;
    // println!("{}", f as i32);
    // let big = 300_u16;
    // println!("{}", big as u8);


    // println!("{}", double(7623432));

    // let pi = std::f64::consts::PI;
    // println!("{:.2}", pi);
    // let er: f64 = 3.0;
    // println!("{}", er as i32 + 1);
    // let beeg:u128 = 340282366920938463463374607431768211455;
    // println!("{}", beeg as i128 + 1);

    // print!("Enter your name: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Enter something");
    // let name = input.trim();
    // println!("{name}");

    // let mut arr = vec![];
    // print!("How many numbers do you want to add? ");
    // io::stdout().flush().unwrap();
    // let mut count = String::new();
    // io::stdin().read_line(&mut count).expect("Enter a value");
    // let num = count.trim().parse().unwrap();
    // for i in 0..num {
    //     print!("Enter number {}: ", i+1);
    //     io::stdout().flush().unwrap();
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("Enter a number");
    //     let output:i32 = input.trim().parse().expect("Please enter a valid number");
    //     arr.push(output);
    // }
    // let mut total = 0;
    // for i in arr{
    //     total += i;
    // }
    // println!("{total}");

    // let x = int(input("Enter a number: "));
    // for i in 0..x + 1{
    //     print!("{} ", i as u128 * i as u128);
    //     let y;
    //     if i != 0 {
    //         y = i as f32 / x as f32 * 100.0;
    //     } else {
    //         y = 0 as f32;
    //     }
    //     println!("{} / {}, {:.2}% ", i, x, y);
    // }

    // STARS

    // for i in 0..11 {
    //     for _j in 0..i {
    //         print!("* ");
    //     }
    //     println!();
    // }
    // for i in 0..11 {
    //     println!();
    //     for _j in 0..10 - i {
    //         print!("* ");
    //     }
    //     for _j in i - 10..9 {
    //         print!("  ")
    //     }
    // }
    // println!();
    // for i in 0..10 {
    //     for _j in 0..i {
    //         print!("  ");
    //     }
    //     for _j in i..10 {
    //         print!("* ")
    //     }
    //     println!()
    // }
    // for i in 0..11 {
    //     for _j in 0..10 - i {
    //         print!("  ");
    //     }
    //     for _j in 10 - i..10 {
    //         print!("* ")
    //     }
    //     println!();
    // }
    let num = int(input("Enter a number to convert to binary: "));
    println!("{}", bin(num.try_into().unwrap()));
    //bin
}