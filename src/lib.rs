#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[napi]
pub fn concat_str(a: String, b: String) -> String {
    format!("{}{}", a, b)
}

#[napi(object)]
pub struct ToolOptions {
  pub id: i32,
  pub name: String,
}

#[napi]
pub fn get_options(options: ToolOptions) -> ToolOptions {
    println!("id: {}, name: {}", options.id, options.name);
    options
}
