use anyhow::{anyhow, Result};
#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => format!("{}", err),
        }
    };
}
fn main() -> anyhow::Result<()> {
    let ret = f1("hello".to_string()).and_then(f2).and_then(f3);
    println!("{:?}", ret);
    // let ret = f3(f2(f1("success".to_string())?)?)?;
    // println!("ret: {:?}", ret);

    let ret = my_try!(f3(my_try!(f2(my_try!(f1("success".to_string()))))));
    println!("my try: {:?}", ret);
    Ok(())
}
fn f1(s: String) -> Result<String> {
    Ok(format!("f1: {}", s))
}

fn f2(s: String) -> Result<String> {
    Ok(format!("f2: {}", s))
}

fn f3(s: String) -> Result<String> {
    Err(anyhow!("f3: {}", s))
}
