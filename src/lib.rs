// 这是一个技巧：re-export
// 以后你在新项目里只要 `use my_rust_stack::prelude::*;`
// 所有你喜欢的库就都自动引入了，不用写一堆 use
pub mod prelude{
    pub use anyhow::{Result, Context};// 自动把 Result 变成 anyhow 的
    pub use text_io::{scan,read};
    pub use walkdir::WalkDir;
    pub use rayon::prelude::*;
    pub use inquire::{Text, validator::{StringValidator, Validation}};
    pub use clap::Parser;
    pub use xshell::{cmd, Shell};
    pub use itertools::Itertools;
     // 你甚至可以封装自己的胶水函数
    pub fn pause() {
        
    }
}