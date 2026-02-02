//! Sloppy error handling.

use crate::copilot::Hallucinate;
use crate::slopify::ToSlop;
use std::panic;

/// Installs a sloppy panic hook.
pub fn install_slop_hook() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("{}", "    panic caught!".to_slop().hallucinate());

        if let Some(location) = panic_info.location() {
            eprintln!(
                "{}",
                format!(
                    "at {}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                )
                .to_slop()
                .hallucinate()
            );
        }

        if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("{}", format!("message: {}", msg).to_slop().hallucinate());
        } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("{}", format!("message: {}", msg).to_slop().hallucinate());
        } else {
            eprintln!("{}", "message: <unknown type>".to_slop().hallucinate());
        }
    }));
}
