(Note to self: Run "cargo publish" to update this)
To get started, first add a SnorfWindow.
The following code will display a 480x540 black window, with a red line drawn at the bottom.

```rust
let mut window =  SnorfWindow::new("Grapher", 480,540, None);
let mut ctx = window.get_context();
while window.is_open() {
    ctx.clear_rect(0x000000);
 
    for i in 10..200 {
        ctx.draw_pixel(i, 10, 0xff0800);
    }
    window.update(&ctx).unwrap();
}
```

A good example of how to use this crate is in the simulated_annealing example, it displays a function and runs a hill climbing algorithm on it indicated by squares.

cargo run --example simulated_annealing


