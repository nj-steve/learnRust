use std::thread as stdthread;
use std::time::Duration;
/*fn main(){

    let v = vec![10,20,30]; // 声明一个向量，变量 v 具有数据的所有权
    print_vector(v);
    println!("{}",v[0]);    // 这行会报错
}

fn print_vector(x:Vec<i32>){
    println!("Inside print_vector function {:?}",x);
}*/
/*
我们在 main() 函数中定义了一个向量，同时将这个向量传递给 print_vector() 作为参数。
因为参数的传递会触发所有权的转让。
因此将 v 传递给 print_vector() 函数时，数据的所有权就从 v 转让到了 参数 x 上。
但函数返回时我们并没有把 x 对数据的所有权转让回 v 变量，因此上面这段代码编译的时候编译的时候就会报错了。

借用 Borrowing 或者说 出借 应该不用我再详细解释了吧，很简单的，
就是 临时性的把东西借给别人，当别人用完了之后就要还回来。

这里的重点有两个字： 借 和 还。

借： 把东西借给他人后，自己就暂时性的失去了东西的所有权（现实中是失去了使用权）。
还： 借了别人的东西要主动还，这应该养成一个良好的习惯，如果不还，就是 占为己有 了。

了解了 借用、借、还 的概念后，对 Rust 语言的 借用 Borrowing 概念就很清晰了。

*/

/*
同时，Rust 也引用了自动 还 的概念，就是要求函数的参数离开其作用域时需要将 所有权 还给当初传递给他的变量，
这个过程，我们需要将函数的参数定义为 & variable_name，同时传递参数时，需要传递 & variable_name。

站在 C++ 语言的角度考虑，就是将 函数的参数定义为引用，同时传递变量的引用。
有了 借用 Borrowing 也就是引用的概念后，我们只要修改两处就能让上面的代码运行起来.
 */
fn print_vector(x:&Vec<i32>){  // 1. 第一步，定义参数接受一个引用
    println!("Inside print_vector function {:?}",x);
    //x[0] = 20;
}

fn main(){
    let v = vec![10,20,30];     // 声明一个向量，变量 v 具有数据的所有权
    print_vector(&v);           // 第二步，传递变量的引用给函数
    println!("{}",v[0]);        // 这行会报错
    let mut device_id:usize = 10000;
    loop {
        stdthread::sleep(Duration::from_millis(10));
        for k in 0..10003 {
            device_id = k;
            println!("device_id == {}",device_id);
            if device_id == 10001{
                println!("10001 device_id == {}",device_id);
                break
            }
        }
        if device_id != 10000 {
            println!("bbbbbb device_id == {}",device_id);
            break
        }
    }
    println!("lllllll device_id == {}",device_id);

    //把引用作为函数参数的行为叫借用
    let s = String::from("hello world");
    let len = cal_len(&s);
    println!("lllll len {} s:{}",len,s);
    //引用不发生所有权转移，所以s依然可以使用

    //可变引用
    let mut ms = String::from("hhh");
    let lenth = cal_lenth(&mut ms);
    println!("mmmmmm len {} s:{}",lenth,ms);
}

//&String类型是String类型的引用
//引用是允许你使用某些值，但不能获得其所有权
fn cal_len(s:&String) -> usize{
    //s.push_str("!!!!"); //不可以改变借用的内容
    s.len()
}

fn cal_lenth(s:&mut String) -> usize{
    s.push_str("!!!!");
    s.len()
}