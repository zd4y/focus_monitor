use crate::{get_active_window, get_window, Window};

use anyhow::{bail, Context, Result};
use xcb::{x, Connection, Event};

pub struct FocusMonitor {
    conn: Connection,
    root_window: x::Window,
    atom_active_window: x::Atom,
}

impl FocusMonitor {
    pub fn try_new() -> Result<Self> {
        let (conn, screen_num) = Connection::connect(None)?;
        let setup = conn.get_setup();

        let atom_active_window = conn.send_request(&x::InternAtom {
            only_if_exists: true,
            name: "_NET_ACTIVE_WINDOW".as_bytes(),
        });
        let atom_active_window = conn.wait_for_reply(atom_active_window)?.atom();
        if atom_active_window == x::ATOM_NONE {
            bail!("Not supported");
        }

        let screen = setup
            .roots()
            .nth(screen_num as usize)
            .context("screen was None")?;
        let root_window = screen.root();

        conn.send_request(&x::ChangeWindowAttributes {
            window: root_window,
            value_list: &[x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE)],
        });

        conn.flush()?;
        Ok(Self {
            conn,
            root_window,
            atom_active_window,
        })
    }

    fn wait_for_window_change(&self) -> Result<x::Window> {
        let event = self.conn.wait_for_event()?;
        if let Event::X(x::Event::PropertyNotify(ev)) = event {
            if ev.atom() == self.atom_active_window {
                return get_active_window(&self.conn, self.root_window, self.atom_active_window);
            }
        }
        self.wait_for_window_change()
    }

    fn get_next_window(&self) -> Result<Option<Window>> {
        let window = self.wait_for_window_change()?;
        if window == x::WINDOW_NONE {
            return Ok(None);
        }
        let window = get_window(&self.conn, window)?;
        Ok(Some(window))
    }
}

impl Iterator for FocusMonitor {
    type Item = Result<Option<Window>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_next_window())
    }
}
