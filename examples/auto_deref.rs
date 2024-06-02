#![allow(unused)]
use macros::AutoDeref;

#[derive(Debug, AutoDeref)]
#[deref(mutable = true, field = "inner")]
pub struct RespBulkString {
    inner: String,

    // #[debug(skip)]
    nothing: (),
}

fn main() {}
