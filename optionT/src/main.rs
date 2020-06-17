use std::fmt::Debug;
//<T: Debug>增加了trait限定的泛型，只有实现了Debug trait的类型才能适用。才能拥有{:?} 格式打印。
fn match_option<T: Debug>(o: Option<T>) {
     match o {
         Some(i) => println!("{:?}", i),
         None => println!("nothing"),
     }
}
//适用trait可以让不同的类型实现同一种行为
 pub trait Fly {
     fn fly(&self) -> bool;
 }
 struct Duck;
 struct Pig;
//使用关键字impl为Duck实现Fly trait。impl Trait for Type
 impl Fly for Duck {
     fn fly(&self) -> bool {
         return true;
     }
 }
 impl Fly for Pig {
     fn fly(&self) -> bool {
         return false;
     }
 }
//fly_static泛型函数，参数为T
//<T: Fly>这种语法形式使用Fly trait对泛型T进行行为上的限制，代表实现了Fly trait的类型，或者拥有fly行为的类型
 fn fly_static<T: Fly>(s: T) -> bool {
     s.fly()
 }
 fn fly_dyn(s: &Fly) -> bool {
     s.fly()
 }

fn main(){
    let a: Option<i32> = Some(3);
    let b: Option<&str> = Some("hello");
    let c: Option<char> = Some('A');
    let d: Option<u32>  = None;

    match_option(a);  // 3
    match_option(b);  // "hello"
    match_option(c);  // 'A'
    match_option(d);  // nothing

    let pig = Pig;
    assert_eq!(fly_static::<Pig>(pig), false);
    let duck = Duck;
    assert_eq!(fly_static::<Duck>(duck), true);
    assert_eq!(fly_dyn(&Pig), false);
    assert_eq!(fly_dyn(&Duck), true);
}
