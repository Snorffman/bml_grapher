use bml_grapher::{GraphCtx, GraphSettings, Hex, SnorfWindow, graph2d::{Graph2DCtx, Graph2DSettings}, math::Vec2D};
use minifb::MouseButton;


const WIDTH: usize = 1080; // 480
const HEIGHT: usize = 540; // 540


const BLACK:u32 = 0x000000;
const WHITE:u32 = 0xffffff ;
const RED:u32 = 0xff0800 ;
const GREY: u32 = 0xf5f5f5;  // 0xc2c2c2
const BLUE: u32 = 0x4328ed;
const YELLOW:u32 = 0xf9f034;
const GREEN:u32 =0x1be81b;


fn main() {

    let mut window = SnorfWindow::new("Grapher", WIDTH,HEIGHT, None);
    let mut ctx = window.get_context();
    ctx.set_thickness(1);
    
    // Axis
    let axoff = 20; // axis offset

    let min_xnum = 0.0; let max_xnum = 10.0;
    let min_ynum = 0.0; let max_ynum = 10.0;
    let border_offsets = [min_xnum,max_xnum,min_ynum,max_ynum];

    let settings = Graph2DSettings::new(Vec2D::new(0.5,0.5), 60,60, 1.,1., Vec2D::new(20,20));
    let mut graph_ctx = Graph2DCtx::new(&mut ctx, settings);


    // let f = |x: f32| {
    //     if x < 0.0|| x > max_xnum {0.0} // partwise function
    //     else {   0.5* (x-3.0) * (2.0*x-2.0).sin() + 5.0   }
    // };


    let mut prev_press_pos:Vec2D<f64> = Vec2D::new(0.,0.);
    let mut prev_pressed = false;

    let dataset = vec![
        vec![10.,10.],
        vec![20.,-5.],
        vec![3.,40.],
        vec![100.,100.]
    ];
    
    while window.is_open() {
        graph_ctx.ctx.clear_rect(WHITE);
        graph_ctx.ctx.set_thickness(1);

        // graph_ctx.handle_inputs(&window);
        // println!("{:?}", graph_ctx.get_centre());
        graph_ctx.draw_axis(&window);

        let f = |x: f64| {
            (0.5* (x-3.0) * (2.0*x-2.0).sin() + 5.0) 
            // (std::f64::consts::PI * 0.5 *  x).tan()
            // x.tan()
            // x.sin()
            // (x+5.) % 10.
        };
        graph_ctx.draw_graph(f,  1, Hex::from_rgb(255, 0, 0));
        graph_ctx.plot_dataset(&dataset, 4, Hex::from_rgb(0, 0, 255));

        window.update(&graph_ctx.ctx).unwrap();
    }
}



// let left_pressed = window.get_mouse_down(MouseButton::Left);

// if let Some((scroll_x, scroll_y)) = window.window.get_scroll_wheel() {
//     let sf = - scroll_y.signum() as f64;
//     graph_ctx.zoom += sf;
// }

// if left_pressed {
//     if let Some(m_pos) = window.get_mouse_pos() {
//         let m_pos = (m_pos.0 as f64, m_pos.1 as f64);
//         if !prev_pressed { // Just started holding left click.
//             prev_press_pos.x = m_pos.0; prev_press_pos.y= m_pos.1;
//         }else {
//             graph_ctx.set_offset( (m_pos.0 - prev_press_pos.x) as f64, m_pos.1 - prev_press_pos.y);
//         }
//     }
// }else if !left_pressed && prev_pressed {
//     graph_ctx.set_centre( graph_ctx.get_centre().x  + graph_ctx.get_offset().x,  graph_ctx.get_centre().y + graph_ctx.get_offset().y);
//     graph_ctx.set_offset(0., 0.);
// }
// // println!("{:?}", graph_ctx.get_centre());

// prev_pressed = left_pressed;