//! # Iterator over focused window change on Linux
//!
//! ### Example usage:
//!
//! The following will print the window that gets focused every time the active window changes.
//!
//! `window` can be `None` if there is no active window.
//!
//! ```no_run
//! # use focus_monitor::FocusMonitor;
//! # fn main() -> anyhow::Result<()> {
//! let focus_monitor = FocusMonitor::try_new()?;
//! for window in focus_monitor {
//!     let window = window?;
//!     println!("{:?}", window);
//! }
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "tokio")]
mod async_monitor;

mod monitor;
mod window;

#[cfg(feature = "tokio")]
pub use async_monitor::AsyncFocusMonitor;

pub use monitor::FocusMonitor;
pub use window::Window;

use anyhow::{Context, Result};
use std::str;
use xcb::x;

fn get_active_window(
    conn: &xcb::Connection,
    root_window: x::Window,
    atom_active_window: x::Atom,
) -> Result<x::Window> {
    let active_window = conn.send_request(&x::GetProperty {
        delete: false,
        window: root_window,
        property: atom_active_window,
        r#type: x::ATOM_WINDOW,
        long_offset: 0,
        long_length: 1,
    });
    let active_window = conn.wait_for_reply(active_window)?;
    let active_window = active_window
        .value::<x::Window>()
        .get(0)
        .context("Active window reply was empty")?;
    Ok(*active_window)
}

fn get_window(conn: &xcb::Connection, window: x::Window) -> Result<Window> {
    let title = get_window_title(conn, window)?;
    let class = get_window_class(conn, window)?;
    Ok(Window { title, class })
}

fn get_window_title(conn: &xcb::Connection, window: x::Window) -> Result<String> {
    let cookie = conn.send_request(&x::GetProperty {
        delete: false,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        long_offset: 0,
        long_length: 1024,
    });
    let reply = conn.wait_for_reply(cookie)?;
    Ok(str::from_utf8(reply.value())
        .context("The WM_NAME property is not valid UTF-8")?
        .to_string())
}

fn get_window_class(conn: &xcb::Connection, window: x::Window) -> Result<(String, String)> {
    let cookie = conn.send_request(&x::GetProperty {
        delete: false,
        window,
        property: x::ATOM_WM_CLASS,
        r#type: x::ATOM_STRING,
        long_offset: 0,
        long_length: 1024,
    });
    let reply = conn.wait_for_reply(cookie)?;
    let class =
        str::from_utf8(reply.value()).context("The WM_CLASS property is not valid UTF-8")?;
    let (left, right) = parse_window_class(class).context("Unexpected WM_CLASS format")?;
    let left = left.to_string();
    let right = right.to_string();
    Ok((left, right))
}

fn parse_window_class(class: &str) -> Option<(&str, &str)> {
    let mut split = class.split('\0');
    let left = split.next()?;
    let right = split.next()?;
    Some((left, right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_window_class() {
        let result = parse_window_class("Navigator\0firefox\0");
        assert_eq!(result, Some(("Navigator", "firefox")));
    }
}
