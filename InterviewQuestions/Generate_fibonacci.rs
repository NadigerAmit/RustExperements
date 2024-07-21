use std::io;
fn finonaci(n:i32) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    finonaci(n-1)+finonaci(n-2)

}

fn main() {
    
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Cant read the input from string");
    let num  = input.trim().parse::<i32>().unwrap();
    let mut i = 0;
    loop {
        println!("Fib of{} is {}",i,finonaci(i));
        i += 1;
        if i == num {break;}
    }
}