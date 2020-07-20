fn main() {
    for node in 0..=10{
        let hash = if node > 3 {
            node * 2
        } else {
            node -1
        };
        println!("hhhhh = {}",hash);
    }

    println!("Hello, world!");
    let mut num = 5;
    let num_ref = &mut num;
    *num_ref = 100;
    println!("{} sizeof &i32 {}", num, std::mem::size_of::<&i32>());

    let mut b = 1;
    let mut bp = &mut b;
    println!("in main bp's addr:{:p}", bp);
    //println!("in main b's value:{}", b);
    pointer_addr(bp);
    println!("in main pointer_addr bp's addr:{}", *bp);
    //println!("in main b's value:{}", b);
    pointer_addr1(bp);
    println!("in main pointer_addr1 bp's addr:{}", *bp);
    println!("in main b's value:{}", b);
}

fn pointer_addr(p: &mut i32) -> &i32 {
    println!("in pointer_addr p's addr:{:p}", p);
    *p = 100;
    p
}

fn pointer_addr1(p: &mut i32) -> &i32 {
    println!("in pointer_addr1 p's addr:{:p}", p);
    *p = 200;
    p
}