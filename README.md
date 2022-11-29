# Iterator over focused window change on Linux

## Example usage:

The following will print the window that gets focused every time the active window changes.

`window` can be `None` if there is no active window.

```rust
use focus_monitor::FocusMonitor;

fn main() -> anyhow::Result<()> {
    let focus_monitor = FocusMonitor::try_new()?;
    for window in focus_monitor {
        let window = window?;
        println!("{:?}", window);
    }

    Ok(())
}
```

### Async

To enable `AsyncFocusMonitor` use `features=["tokio"]` in `Cargo.toml`:

```
focus_monitor = { version = "0.1", features = ["tokio"] }
```

And you can use it like this:

```rust
use focus_monitor::AsyncFocusMonitor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut focus_monitor = AsyncFocusMonitor::try_new()?;
    let window = focus_monitor.recv().await?;
    println!("{:?}", window);
}
```
