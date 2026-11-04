//! Host terminal configuration for the interactive Ouros UART simulator.
//!
//! On Unix, a terminal normally provides canonical (line-buffered) input and
//! local echo. The FPGA UART does neither: every keystroke is available to the
//! UART immediately, and only bytes transmitted by Ouros are displayed.
//!
//! Use `stty` rather than a new Cargo dependency. Save the *complete* existing
//! terminal settings and restore them with RAII on normal exit, errors, and
//! Rust panics (when panic=unwind). Redirected/piped stdin remains untouched.

use std::io::{self, IsTerminal};

#[cfg(unix)]
use std::process::{Command, Output, Stdio};

pub struct RawTerminal {
    #[cfg(unix)]
    saved_settings: Option<String>,
}

impl RawTerminal {
    pub fn enter() -> io::Result<Self> {
        #[cfg(unix)]
        {
            if !io::stdin().is_terminal() {
                // Piped input retains the previous line-stream behavior.
                return Ok(Self { saved_settings: None });
            }

            let original = run_stty(&["-g"])?;
            let saved_settings = String::from_utf8(original.stdout)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?
                .trim()
                .to_owned();
            if saved_settings.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "stty -g returned empty terminal settings",
                ));
            }
            if let Err(err) = run_stty(&["raw", "-echo"]) {
                // Best effort in case stty applied some flags before failing.
                let _ = run_stty(&[saved_settings.as_str()]);
                return Err(err);
            }
            Ok(Self { saved_settings: Some(saved_settings) })
        }
        #[cfg(not(unix))]
        {
            // Leave redirected inputs usable on unsupported platforms.
            // Native interactive raw mode currently targets Unix terminals.
            Ok(Self {})
        }
    }

    pub fn is_raw(&self) -> bool {
        #[cfg(unix)]
        {
            self.saved_settings.is_some()
        }
        #[cfg(not(unix))]
        {
            false
        }
    }
}

#[cfg(unix)]
fn run_stty(args: &[&str]) -> io::Result<Output> {
    // Command::output() would otherwise connect stdin to /dev/null, which
    // makes `stty -g` fail with "Inappropriate ioctl for device".
    let output = Command::new("stty")
        .args(args)
        .stdin(Stdio::inherit())
        .output()?;
    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("stty {} failed: {}", args.join(" "), String::from_utf8_lossy(&output.stderr).trim()),
        ));
    }
    Ok(output)
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(saved_settings) = self.saved_settings.take() {
            use std::io::Write;
            // Recover if the Haskell program was stopped while in the Game of
            // Life alternate screen. These ANSI sequences are harmless when
            // the main screen is already active. Avoid writing to pipelines.
            if io::stdout().is_terminal() {
                let mut stdout = io::stdout().lock();
                let _ = stdout.write_all(b"\x1b[?25h\x1b[?1049l");
                let _ = stdout.flush();
            }
            let _ = run_stty(&[saved_settings.as_str()]);
        }
    }
}
