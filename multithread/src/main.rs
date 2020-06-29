use std::thread;
use std::time::Duration;
fn main() {
    let mut v = vec![];
    for id in 0..5 {
        let child = thread::spawn(move || {
            println!("in child: {}", id);
            println!("in child: {}", id);
            println!("in child: {}", id);
            println!("in child: {}", id);
            println!("in child: {}", id);
        });
        v.push(child);
        v.pop();
    }
    let child = thread::spawn(move || {
        println!("in child: {}", 6);
        println!("in child: {}", 6);
        println!("in child: {}", 6);
        println!("in child: {}", 6);
        println!("in child: {}", 6);
        println!("in child: {}", 6);
    });
    v.push(child);
    let child = thread::spawn(move || {
        println!("in child: {}", 7);
        println!("in child: {}", 7);
        println!("in child: {}", 7);
        println!("in child: {}", 7);
        println!("in child: {}", 7);
        println!("in child: {}", 7);
        println!("in child: {}", 7);
        println!("in child: {}", 7);
    });
    v.push(child);
   println!("in main : join before ");
   //for child in v {
       //child.join(); //Waits for the associated thread to finish.
   //}
    println!("in child: {}", 8);
    println!("in child: {}", 8);
    println!("in child: {}", 8);
    println!("in child: {}", 8);
    println!("in child: {}", 8);
    println!("in child: {}", 8);
    println!("in child: {}", 8);
    for child in v {
        child.join(); //Waits for the associated thread to finish.
    }
    println!("in main : join after");

    let tt = thread::spawn(move || {
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9%3 == 1);
        return
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
        println!("in child: {}", 9);
    });
    //let ttt = thread::spawn(move || {
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        thread::sleep(Duration::from_nanos(1));
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 10);
        println!("in child: {}", 11);
        println!("in child: {}", 10);

    //});
    tt.join();
    //ttt.join();
    println!("in main : jjjjjjjoin after");
 }

