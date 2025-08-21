use tracing::{debug, error, info, warn};

pub fn test_log() {
    let file_appender = tracing_appender::rolling::hourly("logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
