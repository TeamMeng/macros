use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3]; // vec![1, 2, 3]
    println!("{:?}", v);

    let v: Vec<i32> = my_vec![]; // vec![]
    println!("{:?}", v);

    let v = my_vec![1; 4]; // vec![1, 1, 1, 1]
    println!("{:?}", v);

    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
    ]; // vec![1, 2, 3, 4, 5]
    println!("{:?}", v);

    let v = <[_]>::into_vec(Box::new([1, 2, 3, 4])); // vec![1, 2, 3, 4]
    println!("{:?}", v);
    Ok(())
}
