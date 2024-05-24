use std::vec;

fn main() {
    let v = my_vec!(1, 2, 3, 4, 5,);
    println!("{:?}", v);
    let v2 = my_vec!(1; 5);
    println!("{:?}", v2);

    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    let c = vec![1, 2, 3];
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

/// my_vec![1,2,3]
/// my_vec!(1,2,3)
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    // $()表示重复的模式，固定用法
    // 逗号放外面表示分隔符
    // $(xxx,)表示分割符为空，每个元素后面都有逗号
    ($($x:expr),+$(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
            // vec![$($x),+];
        }
    };
    ($m:expr; $n:expr) =>{
        {
            let temp_vec = std::vec::from_elem($m, $n);
            temp_vec
        }
    }
}
