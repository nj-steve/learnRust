
trait Node {
    fn move_to(&mut self, x: f32, y: f32);
    fn draw(&self);
}

struct EmptyNode {
    x: f32,
    y: f32,
}

impl Node for EmptyNode {
    fn draw(&self) {
        println!("node: x={}, y={}", self.x, self.y)
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

struct Sprite {
    x: f32,
    y: f32,
}

impl Node for Sprite {
    fn draw(&self) {
        println!("sprite: x={}, y={}", self.x, self.y)
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

// 接口
trait Area {
    fn area(&self) -> f64;
}

// 具体类
struct Circle {
    r: f64
}

// 让【具体类】实现【接口】
impl Area for Circle {
    fn area(&self) -> f64 {
        (3.14 * self.r) // 作为返回值 => 必须使用 () 括起来，并不能写 ;
    }
}

// 利用结构体定义成员变量
struct Fruit {
    color: String,
    weight: f32
}
// 利用impl关键字来定义结构体成员方法
impl Fruit {
    // 相当于方法Fruit::new()调用
    fn new(color: String,weight:f32) -> Fruit {
        Fruit {
            color: color,
            weight: weight
        }
    }
    fn printInfo(&self) {
        println!("{},{}",self.color,self.weight);
    }
}

trait Page{
    fn set_page(&self, p: i32){
        println!("Page Default: 1");
    }
}
trait PerPage{
    fn set_perpage(&self, num: i32){
        println!("Per Page Default: 10");
    }
}
trait Paginate: Page + PerPage{
    fn set_skip_page(&self, num: i32){
        println!("Skip Page : {:?}", num);
    }
}
impl <T: Page + PerPage>Paginate for T{}

struct MyPaginate{ page: i32 }
impl Page for MyPaginate{}
impl PerPage for MyPaginate{}

fn main()
{
    let en = EmptyNode{x:10.5,y:20.5};
    en.draw();

    let sp = Sprite{x:10.5,y:20.5};
    sp.draw();

    let r = Circle {r:10.5};
    println!("area = {:?}", r.area());

    let f = Fruit::new(String::from("green"), 12.5);
    f.printInfo();

    let my_paginate = MyPaginate{page: 1};
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);
}
