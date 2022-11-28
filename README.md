# Iterator over focused window change on Linux

### Example usage:

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
