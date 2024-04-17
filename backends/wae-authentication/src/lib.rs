#![doc = include_str!("../readme.md")]
#![warn(missing_docs)]

pub mod jwt;
pub mod oauth2;
pub mod saml;
pub mod totp;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// 认证错误类型
#[derive(Debug)]
pub enum AuthError {
    /// 认证失败
    AuthenticationFailed(String),

    /// 无效凭证
    InvalidCredentials,

    /// Token 无效
    InvalidToken(String),

    /// Token 过期
    TokenExpired,

    /// 权限不足
    PermissionDenied(String),

    /// 用户不存在
    UserNotFound(String),

    /// 用户已存在
    UserAlreadyExists(String),

    /// 密码不符合要求
    PasswordRequirement(String),

    /// 账户被锁定
    AccountLocked,

    /// 账户未验证
    AccountNotVerified,

    /// 操作超时
    Timeout(String),

    /// 服务内部错误
    Internal(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AuthError::TokenExpired => write!(f, "Token expired"),
            AuthError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            AuthError::UserNotFound(msg) => write!(f, "User not found: {}", msg),
            AuthError::UserAlreadyExists(msg) => write!(f, "User already exists: {}", msg),
            AuthError::PasswordRequirement(msg) => write!(f, "Password does not meet requirements: {}", msg),
            AuthError::AccountLocked => write!(f, "Account is locked"),
            AuthError::AccountNotVerified => write!(f, "Account is not verified"),
            AuthError::Timeout(msg) => write!(f, "Operation timeout: {}", msg),
            AuthError::Internal(msg) => write!(f, "Auth internal error: {}", msg),
        }
    }
}

impl std::error::Error for AuthError {}

/// 认证操作结果类型
pub type AuthResult<T> = Result<T, AuthError>;

/// 用户 ID 类型
pub type UserId = String;

/// 角色 ID 类型
pub type RoleId = String;

/// 权限代码类型
pub type PermissionCode = String;

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户 ID
    pub id: UserId,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
    /// 头像 URL
    pub avatar_url: Option<String>,
    /// 是否已验证
    pub verified: bool,
    /// 是否已禁用
    pub disabled: bool,
    /// 自定义属性
    pub attributes: HashMap<String, serde_json::Value>,
    /// 创建时间
    pub created_at: i64,
    /// 更新时间
    pub updated_at: i64,
}

/// 用户凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// 用户名或邮箱
    pub identifier: String,
    /// 密码
    pub password: String,
    /// 额外参数
    pub extra: HashMap<String, String>,
}

/// 认证 Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: Option<String>,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间 (秒)
    pub expires_in: u64,
    /// 过期时间戳
    pub expires_at: i64,
}

/// Token 验证结果
#[derive(Debug, Clone)]
pub struct TokenValidation {
    /// 用户 ID
    pub user_id: UserId,
    /// 用户信息
    pub user: Option<UserInfo>,
    /// 角色
    pub roles: Vec<Role>,
    /// 权限
    pub permissions: Vec<PermissionCode>,
    /// Token 元数据
    pub metadata: HashMap<String, String>,
}

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// 角色 ID
    pub id: RoleId,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 权限列表
    pub permissions: Vec<PermissionCode>,
}

/// 用户创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 显示名称
    pub display_name: Option<String>,
    /// 自定义属性
    pub attributes: HashMap<String, serde_json::Value>,
}

/// 用户更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    /// 显示名称
    pub display_name: Option<String>,
    /// 头像 URL
    pub avatar_url: Option<String>,
    /// 自定义属性
    pub attributes: Option<HashMap<String, serde_json::Value>>,
}

/// 密码修改请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

/// 认证配置
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Token 过期时间 (秒)
    pub token_expires_in: u64,
    /// 刷新 Token 过期时间 (秒)
    pub refresh_token_expires_in: u64,
    /// Token 签发者
    pub issuer: String,
    /// Token 受众
    pub audience: String,
    /// 密码最小长度
    pub password_min_length: usize,
    /// 是否要求密码包含数字
    pub password_require_digit: bool,
    /// 是否要求密码包含特殊字符
    pub password_require_special: bool,
    /// 登录失败锁定阈值
    pub max_login_attempts: u32,
    /// 锁定时间 (秒)
    pub lockout_duration: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            token_expires_in: 3600,
            refresh_token_expires_in: 86400 * 7,
            issuer: "wae-authentication".to_string(),
            audience: "wae-api".to_string(),
            password_min_length: 8,
            password_require_digit: true,
            password_require_special: false,
            max_login_attempts: 5,
            lockout_duration: 1800,
        }
    }
}

/// 认证服务 trait

pub trait AuthService: Send + Sync {
    /// 用户登录
    ///
    /// # Arguments
    /// * `credentials` - 用户凭证
    ///
    /// # Returns
    /// 认证 Token
    async fn login(&self, credentials: &Credentials) -> AuthResult<AuthToken>;

    /// 用户登出
    ///
    /// # Arguments
    /// * `token` - 访问令牌
    async fn logout(&self, token: &str) -> AuthResult<()>;

    /// 刷新 Token
    ///
    /// # Arguments
    /// * `refresh_token` - 刷新令牌
    async fn refresh_token(&self, refresh_token: &str) -> AuthResult<AuthToken>;

    /// 验证 Token
    ///
    /// # Arguments
    /// * `token` - 访问令牌
    async fn validate_token(&self, token: &str) -> AuthResult<TokenValidation>;

    /// 创建用户
    ///
    /// # Arguments
    /// * `request` - 创建请求
    async fn create_user(&self, request: &CreateUserRequest) -> AuthResult<UserInfo>;

    /// 获取用户信息
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    async fn get_user(&self, user_id: &str) -> AuthResult<UserInfo>;

    /// 更新用户信息
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    /// * `request` - 更新请求
    async fn update_user(&self, user_id: &str, request: &UpdateUserRequest) -> AuthResult<UserInfo>;

    /// 删除用户
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    async fn delete_user(&self, user_id: &str) -> AuthResult<()>;

    /// 修改密码
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    /// * `request` - 密码修改请求
    async fn change_password(&self, user_id: &str, request: &ChangePasswordRequest) -> AuthResult<()>;

    /// 重置密码
    ///
    /// # Arguments
    /// * `identifier` - 用户标识 (用户名/邮箱/手机号)
    async fn reset_password(&self, identifier: &str) -> AuthResult<()>;

    /// 验证用户权限
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    /// * `permission` - 权限代码
    async fn check_permission(&self, user_id: &str, permission: &str) -> AuthResult<bool>;

    /// 获取用户角色
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    async fn get_user_roles(&self, user_id: &str) -> AuthResult<Vec<Role>>;

    /// 分配角色
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    /// * `role_id` - 角色 ID
    async fn assign_role(&self, user_id: &str, role_id: &str) -> AuthResult<()>;

    /// 移除角色
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    /// * `role_id` - 角色 ID
    async fn remove_role(&self, user_id: &str, role_id: &str) -> AuthResult<()>;

    /// 获取配置
    fn config(&self) -> &AuthConfig;
}

/// API Key 认证 trait

pub trait ApiKeyAuth: Send + Sync {
    /// 验证 API Key
    ///
    /// # Arguments
    /// * `api_key` - API Key
    async fn validate_api_key(&self, api_key: &str) -> AuthResult<TokenValidation>;

    /// 创建 API Key
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    /// * `name` - Key 名称
    /// * `expires_in` - 过期时间 (秒)，None 表示永不过期
    async fn create_api_key(&self, user_id: &str, name: &str, expires_in: Option<u64>) -> AuthResult<String>;

    /// 撤销 API Key
    ///
    /// # Arguments
    /// * `api_key` - API Key
    async fn revoke_api_key(&self, api_key: &str) -> AuthResult<()>;

    /// 列出用户的所有 API Key
    ///
    /// # Arguments
    /// * `user_id` - 用户 ID
    async fn list_api_keys(&self, user_id: &str) -> AuthResult<Vec<ApiKeyInfo>>;
}

/// API Key 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyInfo {
    /// Key ID
    pub id: String,
    /// Key 名称
    pub name: String,
    /// Key 前缀 (用于识别)
    pub prefix: String,
    /// 创建时间
    pub created_at: i64,
    /// 过期时间
    pub expires_at: Option<i64>,
    /// 最后使用时间
    pub last_used_at: Option<i64>,
}

/// 内存认证实现
pub mod memory {
    use super::*;
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::RwLock;

    /// 内存用户存储
    struct UserRecord {
        info: UserInfo,
        password_hash: String,
        roles: Vec<Role>,
        login_attempts: u32,
        locked_until: Option<i64>,
    }

    /// 内存认证服务
    pub struct MemoryAuthService {
        config: AuthConfig,
        users: Arc<RwLock<HashMap<UserId, UserRecord>>>,
        tokens: Arc<RwLock<HashMap<String, (UserId, i64)>>>,
        refresh_tokens: Arc<RwLock<HashMap<String, (UserId, i64)>>>,
    }

    impl MemoryAuthService {
        /// 创建新的内存认证服务
        pub fn new(config: AuthConfig) -> Self {
            Self {
                config,
                users: Arc::new(RwLock::new(HashMap::new())),
                tokens: Arc::new(RwLock::new(HashMap::new())),
                refresh_tokens: Arc::new(RwLock::new(HashMap::new())),
            }
        }

        fn hash_password(password: &str) -> String {
            format!("hash:{}", password)
        }

        fn verify_password(password: &str, hash: &str) -> bool {
            hash == &Self::hash_password(password)
        }

        fn generate_token() -> String {
            format!("token_{}", uuid::Uuid::new_v4())
        }

        fn current_timestamp() -> i64 {
            chrono::Utc::now().timestamp()
        }
    }

    impl Default for MemoryAuthService {
        fn default() -> Self {
            Self::new(AuthConfig::default())
        }
    }

    impl AuthService for MemoryAuthService {
        async fn login(&self, credentials: &Credentials) -> AuthResult<AuthToken> {
            let mut users = self.users.write().await;

            let user = users
                .values_mut()
                .find(|u| u.info.username == credentials.identifier || u.info.email.as_deref() == Some(&credentials.identifier))
                .ok_or(AuthError::InvalidCredentials)?;

            if user.locked_until.map(|t| t > Self::current_timestamp()).unwrap_or(false) {
                return Err(AuthError::AccountLocked);
            }

            if !Self::verify_password(&credentials.password, &user.password_hash) {
                user.login_attempts += 1;
                if user.login_attempts >= self.config.max_login_attempts {
                    user.locked_until = Some(Self::current_timestamp() + self.config.lockout_duration as i64);
                    return Err(AuthError::AccountLocked);
                }
                return Err(AuthError::InvalidCredentials);
            }

            user.login_attempts = 0;
            user.locked_until = None;

            let access_token = Self::generate_token();
            let refresh_token = Self::generate_token();
            let now = Self::current_timestamp();

            self.tokens
                .write()
                .await
                .insert(access_token.clone(), (user.info.id.clone(), now + self.config.token_expires_in as i64));
            self.refresh_tokens
                .write()
                .await
                .insert(refresh_token.clone(), (user.info.id.clone(), now + self.config.refresh_token_expires_in as i64));

            Ok(AuthToken {
                access_token,
                refresh_token: Some(refresh_token),
                token_type: "Bearer".to_string(),
                expires_in: self.config.token_expires_in,
                expires_at: now + self.config.token_expires_in as i64,
            })
        }

        async fn logout(&self, token: &str) -> AuthResult<()> {
            self.tokens.write().await.remove(token);
            Ok(())
        }

        async fn refresh_token(&self, refresh_token: &str) -> AuthResult<AuthToken> {
            let mut refresh_tokens = self.refresh_tokens.write().await;
            let (user_id, _) =
                refresh_tokens.remove(refresh_token).ok_or_else(|| AuthError::InvalidToken("Invalid refresh token".into()))?;

            let access_token = Self::generate_token();
            let new_refresh_token = Self::generate_token();
            let now = Self::current_timestamp();

            self.tokens
                .write()
                .await
                .insert(access_token.clone(), (user_id.clone(), now + self.config.token_expires_in as i64));
            refresh_tokens.insert(new_refresh_token.clone(), (user_id, now + self.config.refresh_token_expires_in as i64));

            Ok(AuthToken {
                access_token,
                refresh_token: Some(new_refresh_token),
                token_type: "Bearer".to_string(),
                expires_in: self.config.token_expires_in,
                expires_at: now + self.config.token_expires_in as i64,
            })
        }

        async fn validate_token(&self, token: &str) -> AuthResult<TokenValidation> {
            let tokens = self.tokens.read().await;
            let (user_id, expires_at) = tokens.get(token).ok_or_else(|| AuthError::InvalidToken("Token not found".into()))?;

            if *expires_at < Self::current_timestamp() {
                return Err(AuthError::TokenExpired);
            }

            let users = self.users.read().await;
            let user = users.get(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.clone()))?;

            let permissions: Vec<PermissionCode> = user.roles.iter().flat_map(|r| r.permissions.iter().cloned()).collect();

            Ok(TokenValidation {
                user_id: user_id.clone(),
                user: Some(user.info.clone()),
                roles: user.roles.clone(),
                permissions,
                metadata: HashMap::new(),
            })
        }

        async fn create_user(&self, request: &CreateUserRequest) -> AuthResult<UserInfo> {
            let mut users = self.users.write().await;

            if users.values().any(|u| u.info.username == request.username) {
                return Err(AuthError::UserAlreadyExists(request.username.clone()));
            }

            let user_id = uuid::Uuid::new_v4().to_string();
            let now = Self::current_timestamp();

            let info = UserInfo {
                id: user_id.clone(),
                username: request.username.clone(),
                email: request.email.clone(),
                phone: request.phone.clone(),
                display_name: request.display_name.clone(),
                avatar_url: None,
                verified: false,
                disabled: false,
                attributes: request.attributes.clone(),
                created_at: now,
                updated_at: now,
            };

            let record = UserRecord {
                info: info.clone(),
                password_hash: Self::hash_password(&request.password),
                roles: vec![],
                login_attempts: 0,
                locked_until: None,
            };

            users.insert(user_id, record);
            Ok(info)
        }

        async fn get_user(&self, user_id: &str) -> AuthResult<UserInfo> {
            let users = self.users.read().await;
            users.get(user_id).map(|u| u.info.clone()).ok_or_else(|| AuthError::UserNotFound(user_id.into()))
        }

        async fn update_user(&self, user_id: &str, request: &UpdateUserRequest) -> AuthResult<UserInfo> {
            let mut users = self.users.write().await;
            let user = users.get_mut(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.into()))?;

            if let Some(name) = &request.display_name {
                user.info.display_name = Some(name.clone());
            }
            if let Some(url) = &request.avatar_url {
                user.info.avatar_url = Some(url.clone());
            }
            if let Some(attrs) = &request.attributes {
                user.info.attributes = attrs.clone();
            }
            user.info.updated_at = Self::current_timestamp();

            Ok(user.info.clone())
        }

        async fn delete_user(&self, user_id: &str) -> AuthResult<()> {
            let mut users = self.users.write().await;
            users.remove(user_id).map(|_| ()).ok_or_else(|| AuthError::UserNotFound(user_id.into()))
        }

        async fn change_password(&self, user_id: &str, request: &ChangePasswordRequest) -> AuthResult<()> {
            let mut users = self.users.write().await;
            let user = users.get_mut(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.into()))?;

            if !Self::verify_password(&request.old_password, &user.password_hash) {
                return Err(AuthError::InvalidCredentials);
            }

            user.password_hash = Self::hash_password(&request.new_password);
            user.info.updated_at = Self::current_timestamp();
            Ok(())
        }

        async fn reset_password(&self, identifier: &str) -> AuthResult<()> {
            let users = self.users.read().await;
            let user = users
                .values()
                .find(|u| u.info.username == identifier || u.info.email.as_deref() == Some(identifier))
                .ok_or_else(|| AuthError::UserNotFound(identifier.into()))?;

            tracing::info!("Password reset requested for user: {}", user.info.id);
            Ok(())
        }

        async fn check_permission(&self, user_id: &str, permission: &str) -> AuthResult<bool> {
            let users = self.users.read().await;
            let user = users.get(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.into()))?;

            Ok(user.roles.iter().any(|r| r.permissions.iter().any(|p| p == permission)))
        }

        async fn get_user_roles(&self, user_id: &str) -> AuthResult<Vec<Role>> {
            let users = self.users.read().await;
            let user = users.get(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.into()))?;
            Ok(user.roles.clone())
        }

        async fn assign_role(&self, user_id: &str, role_id: &str) -> AuthResult<()> {
            let mut users = self.users.write().await;
            let user = users.get_mut(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.into()))?;

            if !user.roles.iter().any(|r| r.id == role_id) {
                user.roles.push(Role { id: role_id.into(), name: role_id.into(), description: None, permissions: vec![] });
            }
            Ok(())
        }

        async fn remove_role(&self, user_id: &str, role_id: &str) -> AuthResult<()> {
            let mut users = self.users.write().await;
            let user = users.get_mut(user_id).ok_or_else(|| AuthError::UserNotFound(user_id.into()))?;

            user.roles.retain(|r| r.id != role_id);
            Ok(())
        }

        fn config(&self) -> &AuthConfig {
            &self.config
        }
    }
}

/// 便捷函数：创建内存认证服务
pub fn memory_auth_service(config: AuthConfig) -> memory::MemoryAuthService {
    memory::MemoryAuthService::new(config)
}
