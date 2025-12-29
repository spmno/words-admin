use std::env;
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    // 获取日志级别，默认INFO
    let log_level = env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "INFO".to_string())
        .parse::<Level>()
        .unwrap_or(Level::INFO);

    // 获取日志目录，默认logs
    let log_dir = env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());

    // 创建日志目录
    std::fs::create_dir_all(&log_dir).unwrap_or_else(|e| {
        eprintln!("创建日志目录失败: {}", e);
    });

    // 创建每日滚动的文件追加器
    let file_appender = tracing_appender::rolling::daily(&log_dir, "app.log");

    // 创建控制台输出层
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // 创建文件输出层
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_timer(fmt::time::ChronoLocal::rfc_3339())
        .with_ansi(false) // 文件不需要ANSI颜色
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // 设置环境过滤器
    let env_filter = EnvFilter::from_default_env().add_directive(log_level.into());

    // 初始化订阅器
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    info!("📝 日志系统初始化完成");
    info!("📁 日志文件目录: {}", log_dir);
    info!("📊 日志级别: {}", log_level);
}
