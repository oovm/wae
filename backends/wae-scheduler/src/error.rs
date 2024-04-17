use std::fmt;

/// 任务调度错误类型
#[derive(Debug)]
pub enum SchedulerError {
    /// 任务不存在
    TaskNotFound(String),

    /// 任务已存在
    TaskAlreadyExists(String),

    /// Cron 表达式解析失败
    InvalidCronExpression(String),

    /// 任务执行失败
    ExecutionFailed(String),

    /// 调度器已关闭
    Shutdown,

    /// 内部错误
    Internal(String),
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchedulerError::TaskNotFound(msg) => write!(f, "Task not found: {}", msg),
            SchedulerError::TaskAlreadyExists(msg) => write!(f, "Task already exists: {}", msg),
            SchedulerError::InvalidCronExpression(msg) => {
                write!(f, "Invalid cron expression: {}", msg)
            }
            SchedulerError::ExecutionFailed(msg) => write!(f, "Task execution failed: {}", msg),
            SchedulerError::Shutdown => write!(f, "Scheduler is shutdown"),
            SchedulerError::Internal(msg) => write!(f, "Scheduler internal error: {}", msg),
        }
    }
}

impl std::error::Error for SchedulerError {}

/// 任务调度结果类型
pub type SchedulerResult<T> = Result<T, SchedulerError>;
