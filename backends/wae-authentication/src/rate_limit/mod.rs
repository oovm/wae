//! 速率限制模块
//! 
//! 提供登录/注册等敏感接口的速率限制功能，防范暴力破解攻击。

mod config;
mod service;
mod error;

pub use config::*;
pub use service::*;
pub use error::*;
