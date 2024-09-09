use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(42).into();
    let left: Direction<i32> = 10.into();
    println!("{:?}, {:?}", up, left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

impl<T> From<i32> for Direction<T> {
    fn from(value: i32) -> Self {
        Direction::Left(value as u32)
    }
}

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(value: DirectionUp<T>) -> Self {
//         Direction::Up(value)
//     }
// }
