// whileとifを用いて実装
fn fizzbuzz_1(num: i32 ){
    let mut cnt = 1;
    while cnt <= num {
        if cnt % 3 == 0 && cnt % 5 == 0 {
            println!("FizzBuzz");
        }else if cnt % 3 == 0 {
            println!("Fizz");
        }else if cnt % 5 == 0 {
            println!("Buzz");
        } 
        else{
            println!("{}", cnt);
        }
        cnt += 1;
    };
}

// forとmatchを使って実装
fn fizzbuzz_2(num: i32){
    for i in 1..=num {
        match i % 15 {
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

//  forとタプルを使って実装
fn fizzbuzz_3(num: i32){
    for i in 1..=num {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    let num = 30;
    // fizzbuzz_1(num);
    // fizzbuzz_2(num);
    fizzbuzz_3(num);
}
