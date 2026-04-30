use std::thread;

fn main(){
     let name= String::from("Rust");

    //move transfers ownership to the thread
    let handle= thread::spawn(move||{
        println!("hello from child thread {}", name);

    });

    handle.join().unwrap();
    println!("hello from main thread");
}
