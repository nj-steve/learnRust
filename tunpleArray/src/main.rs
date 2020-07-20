
fn main() {
    struct User {
        sign_in_count: u64,
        active: bool,
    }
    let mut tunplearray = Vec::with_capacity(100);
    for i in (0..100) {
        tunplearray.push((i,"aaaaaa",User{sign_in_count:i,active:false}));
    }
    //User{sign_in_count:i,active:false}
    let a = [10,20,30];
    let iter = a.iter();
    for data in iter {
        //println!("{}",data);
    }

    for tunp in tunplearray.iter() {
        //println!("tunp {:?}",tunp.2.sign_in_count);
        //println!("tunp {:?}",tunp.2.sign_in_count);
    }

    for tunl in tunplearray.into_iter().map(|(t1,t2,t3)| {
        //println!("tun {:?}",tunp.2.sign_in_count);
        //println!("tun {:?}",tun.2.sign_in_count);
        (t1,t2,t3)
    }){
        //println!("tunl {:?}",tunp.2.sign_in_count);
        println!("tunl {:?}",tunl.0);
        println!("tunl {:?}",tunl.1);
        println!("tunl {:?}",tunl.2.sign_in_count + 1);
        println!("tunl {:?}",tunl.2.active);
    }

    let mut c = 0;

    for pair in vec!['a', 'b', 'c'].into_iter()
        .map(|letter| { c += 1; (letter, c) }) {
        println!("{:?}", pair);
    }

    println!("Hello, world!");
}
