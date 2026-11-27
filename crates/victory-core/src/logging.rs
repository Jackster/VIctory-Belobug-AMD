//! One tracing setup for every binary.
//!
//! Filter with `VICTORY_LOG` (falls back to `RUST_LOG`), e.g.
//! `VICTORY_LOG=victory_loader=trace,info`. Debug and trace output exist
//! only in builds with the `diagnostics` feature; a plain release build
//! logs info and above whatever the filter asks for.

use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// The most verbose level this build can log.
const CAP: LevelFilter = if cfg!(feature = "diagnostics") {
    LevelFilter::TRACE
} else {
    LevelFilter::INFO
};

/// Whether `VICTORY_LOG` is a single bare token that is not a level: almost
/// always a mistyped level, which `EnvFilter` would take for a target and
/// hide every other log line. Directives and lists (`=`, `,`, `[`) are left
/// to `EnvFilter`.
fn bare_target_typo(value: &str) -> bool {
    let value = value.trim();
    let level = ["off", "error", "warn", "info", "debug", "trace"]
        .iter()
        .any(|level| value.eq_ignore_ascii_case(level));
    !value.is_empty()
        && !value.contains(['=', ',', '['])
        && !level
        && !matches!(value, "0" | "1" | "2" | "3" | "4" | "5")
}

pub fn init(default_level: &str) {
    let filter = ["VICTORY_LOG", "RUST_LOG"]
        .into_iter()
        .find_map(|var| std::env::var(var).ok().map(|value| (var, value)))
        .and_then(|(var, value)| {
            if var == "VICTORY_LOG" && bare_target_typo(&value) {
                eprintln!(
                    "ignoring {var}={value:?}: not a level; write {value}=debug to filter by target"
                );
                return None;
            }
            match EnvFilter::try_new(&value) {
                Ok(filter) => Some(filter),
                Err(e) => {
                    eprintln!("ignoring {var}={value:?}: {e}");
                    None
                }
            }
        })
        .unwrap_or_else(|| EnvFilter::new(default_level));
    let _ = tracing_subscriber::registry()
        .with(CAP)
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_writer(std::io::stderr),
        )
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::bare_target_typo;

    #[test]
    fn levels_are_not_typos() {
        for value in ["debug", "INFO", "0", "5", " debug "] {
            assert!(!bare_target_typo(value), "{value:?}");
        }
    }

    #[test]
    fn directives_and_lists_are_not_typos() {
        for value in ["debug,", "victory_gpu=debug", "info,[span]=debug", ""] {
            assert!(!bare_target_typo(value), "{value:?}");
        }
    }

    #[test]
    fn bare_mistyped_levels_are_typos() {
        for value in ["deubg", "victory_gpu"] {
            assert!(bare_target_typo(value), "{value:?}");
        }
    }
}
