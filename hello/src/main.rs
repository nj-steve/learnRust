use std::io;//input , output
use rand::Rng;//trait 其他语言的接口
use std::cmp::Ordering;

fn main() {
    test_s1();
    generate_string();
    let xx = another_hello(255);
    println!("xxx={}",xx);
    //ThreadRng这个类型是随机数生成器
    let sec_num = rand::thread_rng().gen_range(1,101);
    //println!("secret num is {}",sec_num);
    println!("Hello, Enter your guss:");
    // let mut foo = 1;//不可变变量，immutable，加mut可变
    // let bar = foo;//把foo变量值绑定到bar上,bar值不可变
    //foo = 2;

    let gg : i32= "66".parse().expect("not parse a num");
    println!("gg is {}",gg);
    let mut t = true;
    t = false;
    let tt:bool = t;
    println!("gg is {}",tt);

    let tup:(i32,f32,bool,u8) = (12,3.14,false,9);
    let (x,y,z,k) = tup;
    println!("{} {} {} {}",tup.0,tup.1,tup.2,tup.3);
    println!("{} {} {} {}",x,y,z,k);

    let p = 1;
    let q = {
        let p = p+1;
        p+2
    };
    println!("qqq = {}",q);

    //loop 无限循环
    loop {
        let mut gus = String::new();//用等号绑定，String类型，创建类型示例
        //read_line获得用户输入，将用户输入放到字符串中gus，&表示引用（上下两个gus指向同一块内存），引用在rust中也是不可变的，所以要加上mut
        io::stdin().read_line(&mut gus).expect("cannot read line");
        //read_line 返回值类型io::Result<usize>

        //允许同名变量 此后 gus都为u32类型
        //trim 去掉两端空白 parse解析成整数
        //let gus:u32 = gus.trim().parse().expect("Please enter a number!");
        let gus:u32 = match  gus.trim().parse(){
            Ok(num) => num,//Ok里面带的值就是我们解析出来的数字，数字赋给gus,返回就可以了
            Err(_) => continue,
        };
        //{}是占位符，输出时会被后面替换
        println!("the num you gus is: {}",gus);

        //Ordering 是枚举类型
        match  gus.cmp(&sec_num){
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
        }
    }

}

fn another_hello(x:u8) -> u8{
    for i in (1..10).rev(){//包含开始不包含结束
        println!("iiii is {}",i);
    }

    let a = [10,20,30,40,50];
    for element in a.iter(){
        println!("the value is {}",element);
    }
    if x>5{
        println!("xxx>5");
    }else {
        println!("xxx<5");
    }
    println!("aaaaaaaaaaa x={}",x);
    return x-1//没分号
}

fn generate_string() {
    let mut s = String::from("hello world");
    s.push_str("!!");
    println!("sssss = {}",s);
}

fn test_s1(){
    let s1 = String::from("hello");
    let s2 = s1;// s1 此时失效了
    //println!("ssss {}",s1); //借用了已经移动的值
    println!("ssss2 {}",s2);
    let s3 = s2.clone();//深拷贝 不管是s2上stack数据还是heap上数据都拷贝过去
    println!("ssss2  {} s3 {}",s2,s3);

    let x = 5;
    let y = x;
    println!("x={} y={}",x,y) //x依然有效，因为x是常量在栈（stack）上
    //如果一个变量使用了Copy trait,那么旧变量赋值后依然可以使用
    //如果一个类型或者该类型一部分实现了Drop trait,那么rust不允许它再去实现Copy trait
}