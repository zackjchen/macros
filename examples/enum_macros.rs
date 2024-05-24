#![allow(unused)]
use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    // 这个超过了1个参数，会报错
    Right(u32, u32),
    // Back(u32) // 当有两个inner相同时，会报错
    Front { speed: u32 }, //不是unnamed, 不会生成impl
}

#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}
#[derive(Debug)]
struct DirectionDown;

fn main() {
    let up = DirectionUp { speed: 100 };
    let up: Direction = up.into();

    let left: Direction = 100.into();
    println!("up: {:?}", up);
    println!("left: {:?}", left);
}
