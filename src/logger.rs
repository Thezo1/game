pub enum LogLevel
{
   LogLevelFatal,
   LogLevelError,
   LogLevelWarn,
   LogLevelInfo,
   LogLevelDebug,
   LogLevelTrace,
}

pub fn get_level_color(level: LogLevel) -> u8
{
    match level
        {
            LogLevel::LogLevelFatal => 30,
            LogLevel::LogLevelError => 31,
            LogLevel::LogLevelWarn => 33,
            LogLevel::LogLevelInfo => 32,
            LogLevel::LogLevelDebug => 34,
            LogLevel::LogLevelTrace => 36,
        }

}
pub fn get_level_string<'a>(level: LogLevel) -> &'a str
{
    match level
        {
            LogLevel::LogLevelFatal => "FATAL",
            LogLevel::LogLevelError => "ERROR",
            LogLevel::LogLevelWarn => "WARN",
            LogLevel::LogLevelInfo => "INFO",
            LogLevel::LogLevelDebug => "DEBUG",
            LogLevel::LogLevelTrace => "TRACE",
        }

}

pub mod macros
{
    #[macro_export]
    macro_rules! log_output
    {
        ($name:path, $msg:literal, $($arg:expr),*) => {
            {
                let level = get_level_string($name);
                let color = get_level_color($name);
                let msg = format!($msg, $($arg),*);
                panic!("\x1b[1;{}m[{}]: {}\x1b[0m",color, level, msg);
            }
        };

        ($name:path, $msg:literal) => {
            {
                let level = get_level_string($name);
                let color = get_level_color($name);
                let msg = format!($msg);
                println!("\x1b[1;{}m[{}]: {}\x1b[0m",color, level, msg);
            }
        }
    }

    #[macro_export]
    macro_rules! fatal
    {
        ($msg:literal, $($arg:expr),*) => {
            log_output!(LogLevel::LogLevelFatal, $msg, $($arg),*)
        };
        ($msg:literal) => {
            log_output!(LogLevel::LogLevelFatal, $msg)
        }
    }

    #[macro_export]
    macro_rules! error
    {
        ($msg:literal, $($arg:expr),*) => {
            log_output!(LogLevel::LogLevelError, $msg, $($arg),*)
        };
        ($msg:literal) => {
            log_output!(LogLevel::LogLevelError, $msg)
        }
    }
    #[macro_export]
    macro_rules! warn
    {
        ($msg:literal, $($arg:expr),*) => {
            log_output!(LogLevel::LogLevelWarn, $msg, $($arg),*)
        };
        ($msg:literal) => {
            log_output!(LogLevel::LogLevelWarn, $msg)
        }
    }
    #[macro_export]
    macro_rules! info
    {
        ($msg:literal, $($arg:expr),*) => {
            log_output!(LogLevel::LogLevelInfo, $msg, $($arg),*)
        };
        ($msg:literal) => {
            log_output!(LogLevel::LogLevelInfo, $msg)
        }
    }
    #[macro_export]
    macro_rules! debug
    {
        ($msg:literal, $($arg:expr),*) => {
            log_output!(LogLevel::LogLevelDebug, $msg, $($arg),*)
        };
        ($msg:literal) => {
            log_output!(LogLevel::LogLevelDebug, $msg)
        }
    }
    #[macro_export]
    macro_rules! trace
    {
        ($msg:literal, $($arg:expr),*) => {
            log_output!(LogLevel::LogLevelTrace, $msg, $($arg),*)
        };
        ($msg:literal) => {
            log_output!(LogLevel::LogLevelTrace, $msg)
        }
    }
}
