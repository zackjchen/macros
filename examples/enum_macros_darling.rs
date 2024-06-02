#![allow(unused)]
use std::fmt::Debug;

use macros::EnumFromDarling;

#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    // 这个超过了1个参数，会报错
    Right(u32, u32),
    // Back(u32) // 当有两个inner相同时，会报错
    Front { speed: u32 }, //不是unnamed, 不会生成impl
}

#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(v: T) -> Self {
        Self { speed: v }
    }
}

#[derive(Debug)]
struct DirectionDown;

fn main() {
    let up = DirectionUp { speed: 100 };

    let left: Direction<u32> = 100.into();
    println!("up: {:?}", up);
    println!("left: {:?}", left);
}
