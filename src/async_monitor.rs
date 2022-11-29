use crate::{FocusMonitor, Window};

use anyhow::Result;
use std::{future, thread};
use tokio::sync::mpsc::{self, Receiver};

pub struct AsyncFocusMonitor {
    rx: Receiver<Result<Option<Window>>>,
}

impl AsyncFocusMonitor {
    pub fn try_new() -> Result<Self> {
        let mut focus_monitor = FocusMonitor::try_new()?;
        let (tx, rx) = mpsc::channel(100);

        thread::spawn(move || loop {
            let item = focus_monitor.next().unwrap();
            tx.blocking_send(item).expect("Send error");
        });

        Ok(Self { rx })
    }

    pub async fn recv(&mut self) -> Result<Option<Window>> {
        future::poll_fn(|cx| self.rx.poll_recv(cx)).await.unwrap()
    }
}
