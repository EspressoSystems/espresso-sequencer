use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::{Parser, ValueEnum};
use log_panics::BacktraceMode;

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum BacktraceLoggingMode {
    #[default]
    Pretty,
    Tracing,
}

/// Logging configuration.
#[derive(Clone, Debug, Default, Parser)]
pub struct Config {
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_BACKTRACE_MODE",
        default_value = "pretty"
    )]
    backtrace_mode: BacktraceLoggingMode,
}

impl Config {
    /// Get the logging configuration from the environment.
    pub fn from_env() -> Self {
        Self::parse_from(std::iter::empty::<String>())
    }

    /// Initialize logging and panic handlers based on this configuration.
    pub fn init(&self) {
        setup_logging();
        match self.backtrace_mode {
            BacktraceLoggingMode::Pretty => setup_backtrace(),
            BacktraceLoggingMode::Tracing => log_panics::Config::new()
                .backtrace_mode(BacktraceMode::Resolved)
                .install_panic_hook(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_panic() {
        Config::from_env().init();

        panic!("panic message");
    }
}
