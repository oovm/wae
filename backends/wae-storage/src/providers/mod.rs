//! 存储服务提供商模块
//!
//! 包含各种存储服务的具体实现

/// 腾讯云 COS 存储提供商
pub mod cos;
/// 本地存储提供商
pub mod local;
/// 阿里云 OSS 存储提供商
pub mod oss;
