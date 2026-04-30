use std::thread;

fn main(){


    let handle= thread::spawn(||{
        println!("hello from child thread");

    });

    handle.join().unwrap(); // join waits until child thread completed.
    println!("hello from main thread");
}
