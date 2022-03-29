fn main() {
    let parents = [0,1,2,3,4,5,6,7,8,9,0];
    let parent = &parents[..8];
    for n in 0..parent.len(){
        println!("nnnn = {}",parent[n]);
    }

    let mut multiparents =  [1;36];
    for n in 0..6 {
        multiparents[n*6 + 0] = 0 ;
        multiparents[n*6 + 1] = 1 ;
        multiparents[n*6 + 2] = 2 ;
        multiparents[n*6 + 3] = 3 ;
        multiparents[n*6 + 4] = 4 ;
        multiparents[n*6 + 5] = 5 ;
    }
    for n in 0..multiparents.len(){
        println!("multiparents = {}",multiparents[n]);
    }

    let mut multiparents2 =  [1;36];
    for n in 0..3 {
        multiparents2[n*14 + 0] = 0 ;
        multiparents2[n*14 + 1] = 1 ;
        multiparents2[n*14 + 2] = 2 ;
        multiparents2[n*14 + 3] = 3 ;
        multiparents2[n*14 + 4] = 4 ;
        multiparents2[n*14 + 5] = 5 ;
        multiparents2[n*14 + 6] = 6 ;
        multiparents2[n*14 + 7] = 7 ;
        if n < 2 {
            multiparents2[n*14 + 8] = 8 ;
            multiparents2[n*14 + 9] = 9 ;
            multiparents2[n*14 + 10] = 10 ;
            multiparents2[n*14 + 11] = 11 ;
            multiparents2[n*14 + 12] = 12 ;
            multiparents2[n*14 + 13] = 13 ;
        }
    }
    for n in 0..multiparents2.len(){
        println!("multiparents2 = {}",multiparents2[n]);
    }

    let a = [0, 1, 2, 3, 4, 5];
    let mut iter = a.iter().step_by(2);
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);

    println!("Hello, world!");
    ownership();
    borrow();
}

fn ownership(){
    let a = [1,2,3];
    let b = &a;
    println!("a addr {:p}",&a);//打印地址
    println!("b addr {:p}",b);//打印地址
    println!("b array {:?}",b);//打印内容
}

//通过&mut 获取c的可变引用
fn borrow(){
    let mut c = vec![1,2,3];//要获取可变引用，必须先声明可变绑定
    let d = &mut c;
    d.push(4);
    println!("d array {:?}",d);
}
