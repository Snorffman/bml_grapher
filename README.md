To get started, first add a SnorfWindow

let mut window =  SnorfWindow::new("Grapher", 480,540, None);
let mut ctx = window.get_context();
while window.is_open() {
   ctx.clear_rect();
 
    for i in 10..200 {
        ctx.draw_pixel(i, 10, 0xff0800);
    }
    window.update(&ctx).unwrap();
}

