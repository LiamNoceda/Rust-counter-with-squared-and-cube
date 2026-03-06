use std::thread;
use std::time::Duration;

fn main() {
    let spaces = "guru";
    println!("Name space: {spaces}");

    let mut counter: i32 = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter: {counter}");
        thread::sleep(Duration::from_millis(500));
        if counter == 10 {
            let squared = counter.pow(2);
            let cube = counter.pow(3);
            println!("Counter squared: {squared}, and cube: {cube}");
        }
    }
    println!("Finished");
}
