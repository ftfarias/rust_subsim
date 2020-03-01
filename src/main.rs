mod physics;
use physics::Point;


fn main() {
    println!("Hello, world!");
    let a = String::from("teste");
    let b = a;

    println!("{}",b);

    let p = Point { x:10.12345, y:12.0 };
    println!("{}", p);

}
