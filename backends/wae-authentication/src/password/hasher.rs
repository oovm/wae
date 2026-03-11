//! 密码哈希器实现

use zeroize::Zeroize;

use crate::password::{PasswordHashConfig, PasswordHashError, PasswordHashResult};

/// 密码哈希器
#[derive(Debug, Clone)]
pub struct PasswordHasherService {
    config: PasswordHashConfig,
}

impl PasswordHasherService {
    /// 创建新的密码哈希器
    ///
    /// # Arguments
    /// * `config` - 密码哈希配置
    pub fn new(config: PasswordHashConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建密码哈希器
    pub fn default() -> Self {
        Self::new(PasswordHashConfig::default())
    }

    /// 哈希密码
    ///
    /// # Arguments
    /// * `password` - 明文密码
    pub fn hash_password(&self, password: &str) -> PasswordHashResult<String> {
        let mut password_bytes = password.as_bytes().to_vec();

        let result = self.hash_bcrypt(&password_bytes);

        password_bytes.zeroize();
        result
    }

    /// 验证密码
    ///
    /// # Arguments
    /// * `password` - 明文密码
    /// * `hash` - 密码哈希
    pub fn verify_password(&self, password: &str, hash: &str) -> PasswordHashResult<bool> {
        let mut password_bytes = password.as_bytes().to_vec();

        let result = self.verify_bcrypt(&password_bytes, hash);

        password_bytes.zeroize();
        result
    }

    /// 使用 bcrypt 哈希密码
    fn hash_bcrypt(&self, password: &[u8]) -> PasswordHashResult<String> {
        bcrypt::hash(password, self.config.bcrypt_cost).map_err(|_| PasswordHashError::HashFailed)
    }

    /// 使用 bcrypt 验证密码
    fn verify_bcrypt(&self, password: &[u8], hash: &str) -> PasswordHashResult<bool> {
        bcrypt::verify(password, hash).map_err(|_| PasswordHashError::VerifyFailed)
    }
}

impl Default for PasswordHasherService {
    fn default() -> Self {
        Self::new(PasswordHashConfig::default())
    }
}
