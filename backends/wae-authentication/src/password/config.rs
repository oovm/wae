//! 密码哈希配置

/// 密码哈希算法类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordHashAlgorithm {
    /// bcrypt 算法
    Bcrypt,
}

/// 密码哈希配置
#[derive(Debug, Clone)]
pub struct PasswordHashConfig {
    /// 使用的哈希算法
    pub algorithm: PasswordHashAlgorithm,
    /// bcrypt 成本因子（4-31，推荐 12）
    pub bcrypt_cost: u32,
}

impl Default for PasswordHashConfig {
    fn default() -> Self {
        Self { algorithm: PasswordHashAlgorithm::Bcrypt, bcrypt_cost: 12 }
    }
}

impl PasswordHashConfig {
    /// 创建新的密码哈希配置
    ///
    /// # Arguments
    /// * `algorithm` - 使用的哈希算法
    pub fn new(algorithm: PasswordHashAlgorithm) -> Self {
        Self { algorithm, ..Default::default() }
    }

    /// 设置 bcrypt 成本因子
    ///
    /// # Arguments
    /// * `cost` - 成本因子（4-31）
    pub fn with_bcrypt_cost(mut self, cost: u32) -> Self {
        self.bcrypt_cost = cost.clamp(4, 31);
        self
    }
}
