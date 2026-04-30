use std::thread;
use std::sync::{Arc, Mutex};

fn main(){

 let count=Arc::new(Mutex::new(0));

 let mut handles=vec![];

 for _i in 1..5{


    let count_clone=Arc::clone(&count);

    let handle = thread::spawn(move || {
        let mut s_count=count_clone.lock().unwrap();
        println!("the count is {}", s_count);
        *s_count+=1;

    });

    handles.push(handle);
 }

 for h in handles{

    h.join().unwrap();
 }

 println!("the final count is {}", *count.lock().unwrap());

}
