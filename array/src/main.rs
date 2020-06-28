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

    let a = 3;
    let b = 1;
    debug_assert_eq!(a, b);

    println!("Hello, world!");
}
