use anyhow::Result;

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

// my_vec! = my_vec! { 1, 2, 3 }; - Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // + 表示1~, *表示0~
    ($($x:expr),+ $(,)?) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //     temp_vec.push($x);
        // )*
        // temp_vec
        <[_]>::into_vec(Box::new([$($x),*]))
    }};
}
