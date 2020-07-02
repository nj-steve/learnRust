use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
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

    let mut hashbuf = VecDeque::new();
    let child = thread::spawn(move || {
        for node in 0..10{
            //hashbuf.push_back(node);
            //thread::sleep(Duration::from_nanos(1));
            push_into_vector(&mut hashbuf, node);
        }
        push_into_vector(&mut hashbuf, 111);
        println!("Inside print_vector function {:?}",hashbuf);
    });
    for node in 0..10 {
        /*let ophasher = hashbuf.pop_front();
        match ophasher {
            Some(hasher) => println!("hhhhh = {}",hasher),
            None => println!("nothing = {}",node),
        }*/
        //pop_from_vector(&mut hashbuf, node);
        thread::sleep(Duration::from_nanos(2));
    }
    //println!("Inside print_vector function {:?}",hashbuf);
    child.join();
    println!("in main : oooooo after");

    struct SectorId(u64);
    let sss = SectorId(10);
    println!("sss 0 = {}",sss.0);
    struct PublicSector {
        pub id: SectorId,
    }
    let pubs = PublicSector{
        id:sss,
    };
    let iiid = pubs.id;
    println!("sss 0 = {}",iiid.0);

    for n in 0..10{
        if n == 3{
            continue
        }
        println!("lllll = {}",n);
    }
}

fn push_into_vector(x:&mut VecDeque<i32>, node:i32) -> &mut VecDeque<i32>{  // 1. 第一步，定义参数接受一个引用
    x.push_back(node);
    //x.push_back(node);
    thread::sleep(Duration::from_nanos(1));
    println!("Inside print_vector function {:?}",x);
    x
}

fn pop_from_vector(x:&mut VecDeque<i32>, node:i32){  // 1. 第一步，定义参数接受一个引用
    let ophasher = x.pop_front();
    match ophasher {
        Some(hasher) => println!("hhhhh = {}",hasher),
        None => println!("nothing = {}",node),
    }
}
