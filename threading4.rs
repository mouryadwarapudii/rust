use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

fn main(){


    let x = Arc::new(RwLock::new(HashMap::new()));

    

    {

        let mut m= x.write().unwrap();
        m.insert("mourya".to_string(), 28);
        m.insert("shiva".to_string(), 100);

        

    }




    let mut handles=vec![];


    for i in 0..2{

        let map_clone=Arc::clone(&x);

        let r_handle =thread::spawn(move || {
            let m=map_clone.read().unwrap();
            println!("the reader prints the value {:?}", m);
        });

        handles.push(r_handle);

        let w_clone= Arc::clone(&x);

        handles.push(thread::spawn(move ||{

            let mut w_m=w_clone.write().unwrap();
            w_m.insert("ganesh".to_string(), 90);
            println!("write is completed in map");
        }));




    }
    for h in handles{

        h.join().unwrap();
    }


    println!("the final hash map is {:?}", *x.read().unwrap());


    
}
