//! 密码哈希和验证模块

use crate::error::{CryptoError, CryptoResult};
use zeroize::Zeroize;

/// 密码哈希算法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordAlgorithm {
    /// bcrypt 算法
    Bcrypt,
    /// argon2 算法
    Argon2,
}

/// 密码哈希器配置
#[derive(Debug, Clone)]
pub struct PasswordHasherConfig {
    /// 密码哈希算法
    pub algorithm: PasswordAlgorithm,
    /// bcrypt 成本因子
    pub bcrypt_cost: u32,
}

impl Default for PasswordHasherConfig {
    fn default() -> Self {
        Self { algorithm: PasswordAlgorithm::Bcrypt, bcrypt_cost: 12 }
    }
}

/// 密码哈希器
#[derive(Debug, Clone)]
pub struct PasswordHasher {
    config: PasswordHasherConfig,
}

impl PasswordHasher {
    /// 创建新的密码哈希器
    pub fn new(config: PasswordHasherConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建密码哈希器
    pub fn default() -> Self {
        Self::new(PasswordHasherConfig::default())
    }

    /// 哈希密码
    pub fn hash_password(&self, password: &str) -> CryptoResult<String> {
        let mut password_bytes = password.as_bytes().to_vec();
        let result = match self.config.algorithm {
            PasswordAlgorithm::Bcrypt => self.hash_bcrypt(&password_bytes),
            PasswordAlgorithm::Argon2 => self.hash_argon2(&password_bytes),
        };
        password_bytes.zeroize();
        result
    }

    /// 验证密码
    pub fn verify_password(&self, password: &str, hash: &str) -> CryptoResult<bool> {
        let mut password_bytes = password.as_bytes().to_vec();
        let result = match self.config.algorithm {
            PasswordAlgorithm::Bcrypt => self.verify_bcrypt(&password_bytes, hash),
            PasswordAlgorithm::Argon2 => self.verify_argon2(&password_bytes, hash),
        };
        password_bytes.zeroize();
        result
    }

    /// 使用 bcrypt 哈希密码
    fn hash_bcrypt(&self, password: &[u8]) -> CryptoResult<String> {
        bcrypt::hash(password, self.config.bcrypt_cost).map_err(|_| CryptoError::PasswordHashError)
    }

    /// 使用 bcrypt 验证密码
    fn verify_bcrypt(&self, password: &[u8], hash: &str) -> CryptoResult<bool> {
        bcrypt::verify(password, hash).map_err(|_| CryptoError::PasswordVerifyError)
    }

    /// 使用 argon2 哈希密码（预留）
    fn hash_argon2(&self, _password: &[u8]) -> CryptoResult<String> {
        Err(CryptoError::InvalidAlgorithm)
    }

    /// 使用 argon2 验证密码（预留）
    fn verify_argon2(&self, _password: &[u8], _hash: &str) -> CryptoResult<bool> {
        Err(CryptoError::InvalidAlgorithm)
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new(PasswordHasherConfig::default())
    }
}
