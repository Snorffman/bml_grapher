//! PARAMS:
use minifb::MouseButton;

use crate::{Ctx, Hex, SnorfWindow, math::Vec2D};

//? Prolly will need to redo the scaling system but we'll work with this for now.

/// # Parameters
/// `xstep` = if xstep=10, we draw a line every 10 pixels.
/// 
/// `xscale`= the value we assign to each xstep, eg xscale=1 means every line increments by 1
/// 
/// `(x/y)step` = how much the virtual numbers increment every line draw.
/// 
/// `centre`: (0.5,0.5) = Means (0,0) lies in the middle of the screen, (0.25, 0.5) x is the first quarter of the screen.
/// 
/// `offset` = The x/y offsets of the graph from the screen boundaries.
pub struct Graph2DSettings {
    centre: Vec2D<f64>, // Where on the screen is (0,0)?
    xstep: i32, ystep: i32,
    xscale: f64, yscale: f64,
    offset: Vec2D<usize>,
}
impl Graph2DSettings {
    pub fn new(centre: Vec2D<f64>, xstep:i32, ystep:i32, xscale:f64, yscale:f64, offset: Vec2D<usize>) -> Self {
        Graph2DSettings{centre, xstep, ystep, xscale, yscale, offset}
    }
    pub fn default() -> Self {
        Graph2DSettings::new(Vec2D::new(0.1,0.1), 60, 60,  1., 1.,Vec2D::new(20,20))
    }
}


pub struct Graph2DCtx<'a> {
    pub ctx: &'a mut Ctx,
    pub settings: Graph2DSettings,
    centre_offset: Vec2D<f64>,
    pub zoom:f64,
    // Input handling
    prev_press_position: Option<Vec2D<f64>>,
}
impl<'a> Graph2DCtx<'a> {
    /// Create a SnorfWindow
    /// 
    /// Common settings:
    /// borderless, title, resize, scale.
    /// 
    /// Steps to creating a SnorfWindow with Graph2DCtx graphing a sine graph:
    /// ```
    /// let mut window =  SnorfWindow::new("Grapher", 480,540, None);
    /// let mut ctx = window.get_context();
    /// let mut graph_ctx = Graph2DCtx::new(&mut ctx, Graph2DSettings::default());
    /// 
    /// while window.is_open() {
    ///     graph_ctx.ctx.clear_rect(0x000000);
    ///     graph_ctx.draw_axis(&window);
    /// 
    ///     let f = |x:f64| {x.sin()};
    ///     graph_ctx.draw_graph(f, 1, Hex::from_rgb(255,0,0));    
    /// 
    ///     window.update(&graph_ctx.ctx).unwrap();
    /// }
    /// ```
    /// ```
    pub fn new(ctx: &'a mut Ctx, settings: Graph2DSettings) -> Self {
        let init_centre = Vec2D::new(settings.centre.x*(ctx.w as f64), settings.centre.y*(ctx.h as f64));
        let settings = Graph2DSettings::new(init_centre,settings.xstep,settings.ystep, settings.xscale, settings.yscale, settings.offset);

        Graph2DCtx { ctx, settings, centre_offset: Vec2D::new(0., 0.), zoom:0., prev_press_position: None}
    }

    pub fn set_offset(&mut self, x:f64,y:f64) {self.centre_offset = Vec2D::new(x,y);}
    pub fn set_centre(&mut self, x:f64, y:f64) { self.settings.centre = Vec2D::new(x,y);}
    pub fn get_centre(&self) -> &Vec2D<f64> {&self.settings.centre}
    pub fn get_offset(&self) -> &Vec2D<f64> {&self.centre_offset}
    fn get_xscale(&self) -> f64 {
        return self.settings.xscale * 10_f64.powf(self.zoom);
    }
    fn get_yscale(&self) -> f64 {
        self.settings.yscale * 10_f64.powf(self.zoom)
    }


    /// Lets you pan and zoom around the graph area.
    fn handle_inputs(&mut self, window: &SnorfWindow) {
        let left_pressed = window.get_mouse_down(MouseButton::Left);

        //? Handle zoom 
        if let Some((_, scroll_y)) = window.window.get_scroll_wheel() {
            let sf = 0.2 * (- scroll_y.signum()) as f64;
            self.zoom += sf; 
        }


        if left_pressed {
            if let Some(m_pos) = window.get_mouse_pos() {
                let m_pos = (m_pos.0 as f64, m_pos.1 as f64);
                if let Some(prev_press_pos) = &self.prev_press_position {
                    self.set_offset( (m_pos.0 - prev_press_pos.x) as f64, m_pos.1 - prev_press_pos.y);
                }else { // Just started holding left click.
                    self.prev_press_position = Some(Vec2D::new(m_pos.0, m_pos.1) );
                }
            }
        }else {
            if  self.prev_press_position.is_none(){
                self.set_centre( self.get_centre().x  + self.get_offset().x,  self.get_centre().y + self.get_offset().y);
                self.set_offset(0., 0.);
            }
            self.prev_press_position = None;
        }
    }

    pub fn draw_axis(&mut self, window: &SnorfWindow) {
        self.handle_inputs(window);
        // We'll just spawn the centre at the centre of the screen by default, and just have you drag to change (like in desmos)
        let init = &self.settings.centre;
        let d = &self.centre_offset;
        let min_x = self.settings.offset.x; let max_x = self.ctx.w - self.settings.offset.x;
        let min_y = self.settings.offset.y; let max_y = self.ctx.h - self.settings.offset.y;
        let min_xi32 = min_x as i32; let max_xi32 = max_x as i32; 
        let min_yi32 = min_y as i32; let max_yi32 = max_y as i32;

        let cx = (init.x + d.x) as i32;
        let cy = (init.y + d.y) as i32;

        let ystep = self.settings.ystep;
        let alpha = (max_yi32 - cy) % ystep;
        let alpha = if alpha < 0 {alpha + ystep} else {alpha};        

        let mut y = max_yi32 - alpha;
        while y >= min_yi32 {
            let _ = self.ctx.draw_line(&Vec2D::new(min_x, y as usize), &Vec2D::new(max_x, y as usize), Hex::from_word("grey"));
            
            if (cx >= min_xi32 && cx <= max_xi32 ) && y >= min_yi32 {
                let vyi = self.get_yscale() * ((y - cy) / self.settings.ystep) as f64; // Virtual x value (how many xscales away from (0,0))
                let text = format!("{:.2}", vyi);
                self.ctx.draw_text(&Vec2D::new(cx as usize, y as usize), &text, 1, None);
            }
            
            y -= ystep;
        }

        let xstep = self.settings.xstep;
        let alpha = (max_xi32 - cx) % xstep;
        let alpha = if alpha < 0 {alpha + xstep} else {alpha};        
        let mut x =  max_xi32 - alpha;
        while x >= min_xi32 {
            let _ = self.ctx.draw_line(&Vec2D::new(x as usize, max_y), &Vec2D::new(x as usize, min_y), Hex::from_word("grey") );


            if (cy >= min_yi32 && cy <= max_yi32 ) && x >= min_xi32 {
                let vxi = self.get_xscale() * ((x - cx) / self.settings.xstep) as f64; // Virtual x value (how many xscales away from (0,0))
                let text = format!("{:.2}", vxi);
                // let text = format!("{:e}", vxi);
                self.ctx.draw_text(&Vec2D::new(x as usize, cy as usize), &text, 1, None);
            }
            x -= xstep;
        }

        // Draw the axis
        if init.y+d.y < max_y as f64  && init.y+d.y > min_y as f64{
            let _ = self.ctx.draw_line(&Vec2D::new(min_x, (init.y+d.y) as usize), &Vec2D::new(max_x, (init.y+d.y) as usize), 0x00000000 );
        }
        if init.x+d.x < max_x as f64 && init.x+d.x > min_x as f64 {
            let _ = self.ctx.draw_line(&Vec2D::new((init.x+d.x) as usize, max_y), &Vec2D::new((init.x+d.x) as usize, min_y), 0x00000000 );
        }

        // let f = |x:f64| {10.* (x/10.).sin()};
        //         let f = |x:f64| {x};
        // self.draw_graph(f, 1, Hex::from_word("red"));


    
    }




    pub fn draw_graph<F: Fn(f64) -> f64 >(&mut self, f: F, sample_rate:usize, color: u32 ) {
        // let f = |x:f64| {x.sin()};
        let min_x = self.settings.offset.x; let max_x = self.ctx.w - self.settings.offset.x;
        let min_y = self.settings.offset.y as f64; let max_y = (self.ctx.h - self.settings.offset.y) as f64;
        let min_yusize = min_y as usize; let max_yusize = max_y as usize;

        let cx = self.settings.centre.x + self.centre_offset.x;
        let cy = self.settings.centre.y + self.centre_offset.y;


        let mut prev_point: Option<Vec2D<isize>> = None;
        for xi in (min_x..=max_x).step_by(sample_rate) {
            let _xi = xi as f64;
            let vxi = self.get_xscale() * ((_xi - cx) / self.settings.xstep as f64);
            let vyi = f(vxi);
            let yi = cy + (self.settings.ystep as f64 * vyi / self.get_yscale()); // virtual y
            // println!("({},{}) -> ({},{}), {}", vxi, vyi, xi,yi, vyi.is_finite() );

            if !(yi.is_nan()  || vyi.abs() > 1e5) {
                let p = Vec2D::new(xi as isize, yi as isize); // current point

                if let Some(pp) = &prev_point {
                    let dy = p.y - pp.y; let dx = sample_rate as isize;
                    // let dy_dx = dy as f64 / dx as f64;

                    if p.distance_squared(pp)  > (self.ctx.thickness/2) as isize && p.distance_squared(pp) < 2_000_000{ 
                    // if dx != 0 && dy_dx.abs() > 1. && dy_dx.abs() < 100. {
                        let prev_thickness = self.ctx.thickness;
                        self.ctx.set_thickness(prev_thickness *2);
                        self.ctx.bounded_draw_line(&Vec2D::new(pp.x as usize, pp.y as usize), &Vec2D::new(p.x as usize, p.y as usize), 
                            max_x, min_x, max_yusize, min_yusize, color);
                        self.ctx.set_thickness(prev_thickness);
                    }else {
                        let _ = self.ctx.bounded_scaled_pixel(xi, yi as usize, 
                                    min_x, max_x, min_yusize, max_yusize, self.ctx.thickness, color);
                    }
                    prev_point = Some(p);
                }else {
                    let _ = self.ctx.draw_scaled_pixel(xi, yi as usize, self.ctx.thickness , color);
                    prev_point = Some(p);
                }
            }else {
                prev_point = None;
            }
        }
    }

    fn to_virtual_space(&self, vec: &Vec2D<f64>) -> Vec2D<f64> {
        let c = self.settings.centre.add_vec(&self.centre_offset);
        Vec2D::new(
            self.get_xscale()* ((vec.x - c.x)/ self.settings.xstep as f64), 
            self.get_yscale()* ((vec.y - c.y)/ self.settings.ystep as f64),
        )
    }
    fn to_window_space(&self, vec: &Vec2D<f64>) -> Vec2D<f64> {
        let c = self.settings.centre.add_vec(&self.centre_offset);
        Vec2D::new( 
            c.x + (self.settings.xstep as f64 * vec.x / self.get_xscale()),
            c.y + (self.settings.ystep as f64 * vec.y / self.get_yscale()),
        )
    }

        /// offset = the axis offset
    /// border_offsets = the ranges used in the plot  [xmin, xmax, ymin, ymax ]
    pub fn plot_on_graph(&mut self, point: &Vec2D<f64>, scale:usize, color: u32) -> Result<(), String>{
        // let offset = self.settings.axis_offset;
        // let border_offsets = self.settings.get_border_offsets();

        // let h =self.ctx.h; let w = self.ctx.w;
        // let new_x = Self::to_window_space(h, w, false, point.x, border_offsets, offset);
        // let new_y = Self::to_window_space(h, w, true, point.y, border_offsets, offset);
        let min_x = self.settings.offset.x as f64; let max_x = (self.ctx.w - self.settings.offset.x) as f64;
        let min_y = self.settings.offset.y as f64; let max_y = (self.ctx.h - self.settings.offset.y) as f64;


        let window_point = Self::to_window_space(&self, point);
        // let window_point = point;
        // println!("window point: ({},{})", window_point.x, window_point.y);

        // let window_point = Vec2D::new(window_point.x as usize, window_point.y as usize)
        if window_point.x > min_x && window_point.x < max_x && window_point.y > min_y && window_point.y < max_y {
            return self.ctx.draw_scaled_pixel(window_point.x as usize, window_point.y as usize, scale, color);
        }else {
            Err(String::new())
        }
        
    }

    pub fn plot_dataset(&mut self, points: &Vec<Vec<f64>>, scale: usize, color: u32) {
        for point in points {
            let point = Vec2D::new(point[0] as f64, point[1] as f64);
            self.plot_on_graph(&point, scale, color);
        }
    }
}






// let mut x = min_x;
// let step = 10;

// let mut prev_xi = self.settings.xscale * ((min_x as f64 - cx) / self.settings.xstep as f64) as f64;
// let vy0 = f(prev_xi);
// let mut prev_yi: Option<f64> =  None; // (cy + (self.settings.ystep as f64 * vy0 / self.settings.yscale) ) as i64;

// for xi in (min_x..=max_x).step_by(sample_rate) {
    // let _xi = xi as f64;
    // let vxi = self.settings.xscale * ((_xi - cx) / self.settings.xstep as f64);
    // let vyi = f(vxi);

//     let yi = cy + (self.settings.ystep as f64 * vyi / self.settings.yscale); // virtual y
//     // if yi >= min_y  && yi <= max_y {
//     if !yi.is_nan() {
    
//         // Draw a line if the previous point is far away enough
//         const DIST:i64 = 5;
//         if (yi as i64 - prev_yi).pow(2) + sr*sr >= DIST*DIST {
//             self.ctx.draw_line(&Vec2D::new(xi-sample_rate, prev_yi as usize), &Vec2D::new(xi, yi as usize), color);
//         }else {
//             // self.ctx.draw_scaled_pixel(xi, yi as usize, 1,color);
//         }
//     }

//     prev_yi = yi as i64;
// }