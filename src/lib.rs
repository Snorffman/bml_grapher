// TODO: 1) Basic line drawing [done], 2) Text via fonts, see how minifb did it.
// TODO: 3) Line thickness Add as ctx property like js. [done,kinda] 


pub mod math;
pub mod text;

use minifb::{Key, Window, WindowOptions};
use math::{invLerp, lerp, Vec2D};
use text::Text;

use std::f32::consts::PI;




/// The Window
pub struct SnorfWindow {
    window: Window,
    w : usize, // window width
    h : usize, // window height
}
impl SnorfWindow {
    /// Create a SnorfWindow
    /// 
    /// Common settings:
    /// borderless, title, resize, scale.
    /// 
    /// Steps to creating a SnorfWindow:
    /// ```
    /// let mut window =  SnorfWindow::new("Grapher", 480,540, None);
    /// let mut ctx = window.get_context();
    /// // 3) <Optional change settings>: window.window.set_...
    /// while window.is_open() {
    ///     ctx.clear_rect(0x000000);
    /// 
    ///     for i in 10..200 {
    ///         ctx.draw_pixel(i, 10, 0xff0800);
    ///     }
    ///     window.update(&ctx).unwrap();
    /// }
    /// ```
    pub fn new(name: &str, w: usize, h: usize, options: Option<WindowOptions>) -> Self {
        if let Some(_options) = options {
            SnorfWindow{ 
                window:  Window::new(name, w, h,_options).unwrap(),
                w, h
            }                        
        }else {
            SnorfWindow{
                window: Window::new(name, w, h, WindowOptions::default() ).unwrap(),
                w,h
            }
        }
    }
    //----------------------- Common settings
    pub fn set_target_fps(&mut self, fps: usize) {
        self.window.set_target_fps(fps);
    }
    pub fn set_position(&mut self, x: isize,y: isize) {
        self.window.set_position(x, y);
    }
    //----------------------------------------

    //------------------------ Common other
    /// Check if the window is open
    pub fn is_open(&self) -> bool {
        return self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    //-------------------------------


    // Drawing~~~~~~~~~~~~~~~~~~~~~~~~

    /// Get a context that matches the size of the window
    pub fn get_context(&self) -> Ctx {
        let (w, h) = &self.window.get_size();
        let buffer: Vec<u32> = vec![0; w*h];

        return Ctx::new(buffer, *w, *h)
    }


    /// Update the window with the context buffer
    pub fn update(&mut self, ctx: &Ctx) -> Result<(), minifb::Error>{
        let (w, h) = self.window.get_size();
        self.window.update_with_buffer(&ctx.buf, w, h)
    }
}

/// Everything is drawn in respect to the bottom left of the screen

pub struct Ctx {
    buf: Vec<u32>,
    w: usize, 
    h: usize,

    thickness: usize, // thickness of lines
}
impl Ctx {
    // border_offsets: [min_xnum, max_xnum, min_ynum, max_ynum]
    pub fn new(buffer: Vec<u32>, width: usize, height: usize)-> Self {
        Ctx{buf:buffer, w: width, h: height, thickness: 1}
    }

    //----------------------- Misc -------------------
    pub fn set_thickness(&mut self, thickness: usize) {
        self.thickness = thickness;
    }


    
    //--------------------------------------------------- DRAWING ---------------------------
    pub fn clear_rect(&mut self, color: u32) {
        for i in self.buf.iter_mut() {
            *i = color;
        }
    }

    /// Draws a pixel, relative to the bottom-left corner of the screen. Color is hexadecimal.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) -> Result<(), String>{ // color is hexadecimal, dis is more memory efficient
        
        let len = self.buf.len();


        if x > self.w || y > self.h  ||  x  >= (y+1) * self.w  || len < (y+1) * self.w {
            Err(format!("Attempted to access ({x},{y}) when dimensions are ({},{})",self.w,self.h))
        }else {
            let idx = (len- (y+1)*self.w) + x;
            self.buf[idx] = color;  
            Ok(())   
        }
    }

    pub fn draw_scaled_pixel(&mut self, x: usize, y:usize, scale:usize, color:u32) -> Result<(), String>{
        let offset = scale-1;

        if x < offset || y < offset {
            return Err(String::from("Attempted to draw pixel at negative coordinate" ))
        }

        let mut err: String= String::new();
        for _x in (x-offset)..=(x+offset) {
            for _y in (y-offset)..=(y+offset) {
                match self.draw_pixel(_x, _y, color) {
                    Ok(()) => continue,
                    Err(e) => {err = e},
                }
            }
        }

        if err.is_empty(){ Ok(()) }
        else {Err(err)}
    }


    /// Draws a line, using brezenheimer algorithm for speeeed because it avoids float calculations
    /// 
    /// It's kinda bad at doing thick lines, I don't really know how to do that properly so I used a hacky method.
    pub fn draw_line(&mut self, start: &Vec2D<usize>, end: &Vec2D<usize>, color: u32) -> Result<(), String> {
        const PIXEL_SCALE:usize = 1;

        let thickness = self.thickness.clone();
        let [x0, y0, x1, y1] = [start.x as i32, start.y as i32, end.x as i32, end.y as i32];
        
        let mut draw_single_line = |x0: i32, y0: i32, x1: i32, y1: i32| -> Result<(), String> { // x0,y0 is the start position
            let mut x:i32 = x0; let mut y:i32 = y0 as i32;
            let dx = (x1 - x0).abs();   let sx = if x0<x1{1} else{-1}; // how it increments
            let dy = - (y1 - y0).abs(); let sy = if y0<y1{1} else{-1};
            let mut error = dx+dy;
    
            loop {
                if x > 0 && y > 0 {
                    let _ = self.draw_scaled_pixel(x as usize,y as usize, PIXEL_SCALE, color);
                }
                if x==x1 && y==y1 {break}
                let e2 = 2*error;
    
                if e2 >= dy {
                    if x==x1{break}
                    error = error + dy;
                    x+=sx;
                }
                if e2 <= dx {
                    if y==y1{break}
                    error = error + dx;
                    y+=sy;
                }
            }
            Ok(())
        };

        // Thicken up the line
        if thickness > 1 {
            // Difference vector, with magnitude 0.5
            let norm: Vec2D<f32> = Vec2D::new((x1-x0) as f32, (y1-y0) as f32).rotate(PI/2.0);

            let mut norm_size = 0.5;
            
            let start = Vec2D::new(x0 as f32, y0 as f32);
            let end = Vec2D::new(x1 as f32, y1 as f32);

            let _ = draw_single_line(x0, y0, x1, y1);
            for _ in 1..thickness {
                let norm_scaled = norm.with_magnitude(norm_size);
                let start_1 = start.add_vec(&norm_scaled);
                let start_2 = start.sub_vec(&norm_scaled);

                let end_1 = end.add_vec(&norm_scaled);
                let end_2 = end.sub_vec(&norm_scaled);

                let _ = draw_single_line(start_1.x as i32, start_1.y as i32, end_1.x as i32, end_1.y as i32);
                let _ = draw_single_line(start_2.x as i32, start_2.y as i32, end_2.x as i32, end_2.y as i32);


                norm_size += 0.5;
            }
        }else {
            let _ = draw_single_line(x0, y0, x1, y1);
        }

        Ok(())

        
    }

    pub fn rect(&mut self, pos: &Vec2D<usize>, width: usize, height: usize, color: u32) {
        let sw:&Vec2D<usize> = pos; 
        let se:&Vec2D<usize> = &[pos.x+width, pos.y].into(); // south-east
        let ne:&Vec2D<usize> = &[pos.x+width, pos.y+height].into(); 
        let nw:&Vec2D<usize> = &[pos.x, pos.y+height].into();

        self.draw_line(sw, se, color);
        self.draw_line(se, ne, color);
        self.draw_line(ne, nw, color);
        self.draw_line(nw, sw, color);
    }


    pub fn draw_text(&mut self, pos: &Vec2D<usize>, text: &str, scale:usize) {
        let text_obj = Text::new(self.w, self.h, scale, true);
        text_obj.draw(&mut self.buf, (pos.x, self.h-pos.y), text);
    }
}

//*----------------------------------------- */
//*------------Graph Ctx-------------------- */
//*----------------------------------------- */
//

pub struct GraphSettings {
    axis_offset: usize,
    min_xnum: f32,
    max_xnum: f32,
    min_ynum: f32,
    max_ynum: f32,
}
impl GraphSettings {
    pub fn new(axis_offset: usize, min_xnum:f32, max_xnum:f32, min_ynum:f32, max_ynum:f32) -> Self {
        GraphSettings{axis_offset, min_xnum, max_xnum, min_ynum, max_ynum}
    }
    pub fn get_border_offsets(&self) -> [f32; 4] {
        [self.min_xnum, self.max_xnum, self.min_ynum, self.max_ynum]
    }
}

// This is a wrapper around ctx that is able to draw graphs on said ctx.
pub struct GraphCtx<'a> {
    pub ctx: &'a mut Ctx,
    settings: GraphSettings,
}
impl<'a> GraphCtx<'a> {
    pub fn new(ctx: &'a mut Ctx, settings: GraphSettings) -> Self {
        GraphCtx { ctx, settings}
    }
    /// draws a y/x-axis, 
    /// offset= how far away the axis is from the screen edge
    /// screen_dim = dimensions of the screen [WIDTH, HEIGHT]
    /// num_offset = hor far away the numbers are from the axis
    /// step = how much the numbers increment
    /// min_num = the starting number
    /// max_num = the final number
    pub fn draw_axis(&mut self, y_axis: bool, step: f32, min_num: f32, max_num: f32 ) {
        let [w,h] = [self.ctx.w, self.ctx.h];
        let offset = self.settings.axis_offset;
        let num_offset = offset / 2;
        

        if y_axis {
            let end_point = Vec2D::new(offset,h-offset);
            let pix_step =  (step * (h- 2* offset) as f32 / (max_num - min_num))  as usize; 

            let mut curr_num = min_num;
            for y in (offset..=(h - offset)).step_by(pix_step) {
                let text_pos = Vec2D::new(offset - num_offset as usize, y);
                // Draw grid line
                if y > offset {
                    self.ctx.draw_line(&text_pos, &Vec2D::new(w-offset, text_pos.y), Hex::from_word("grey"));
                }
                
                self.ctx.draw_text(&text_pos, &curr_num.to_string(), 1);

                curr_num += step;
            }

            // Y axis
            self.ctx.draw_line(&Vec2D::new(offset,offset), &end_point, 0x000000);
            //  Y text
            self.ctx.draw_text(&[offset/2,h-offset/2].into(), "Y", 1);
        
        }else {
            let pix_step =  (step * (w-2*offset) as f32 / (max_num - min_num))  as usize; 

            let mut curr_num = min_num;
            for x in (offset..w).step_by(pix_step) {
                let text_pos = Vec2D::new(x,offset - num_offset as usize);
                // Draw grid line
                if x > offset {
                    self.ctx.draw_line(&text_pos, &Vec2D::new(text_pos.x, h-offset), Hex::from_word("grey"));
                }
                self.ctx.draw_text(&text_pos, &curr_num.to_string(), 1);
                
                curr_num += step;
            }
            // X axis
            self.ctx.draw_line(&Vec2D::new(offset,offset), &Vec2D::new(w-offset, offset), 0x000000);
            // X text
            self.ctx.draw_text(&[w-offset/2,offset].into(), "X", 1);
        }
    }


    //------------------------------- Lerping
    fn to_window_space(h:usize,w:usize,is_y_component: bool, n: f32, border_offsets: [f32;4], offset: usize) -> usize {
        let [min_xnum, max_xnum, min_ynum, max_ynum] = border_offsets;
        if is_y_component {
            let numerator = (n - min_ynum as f32) * (h - 2*offset) as f32;
            let denominator = (max_ynum - min_ynum) as f32;
            (numerator / denominator) as usize + offset
        }else {
            let numerator = (n - min_xnum as f32) * (w - 2*offset) as f32;
            let denominator = (max_xnum - min_xnum) as f32;
            (numerator/ denominator) as usize + offset
        }
    }

        // num_n = minY+ (maxY - minY) * (win_n -o)/ (h-2o)
    // ((num_n - minY) * (h-2o) / (maxY - minY) ) + o = win_n
    fn to_number_space(h:usize,w:usize, is_y_component: bool, n: usize, border_offsets: [f32;4], offset: usize) -> f32 {

        let [min_xnum, max_xnum, min_ynum, max_ynum] = border_offsets;
    
        if is_y_component {
            lerp(min_ynum, max_ynum, (n - offset) as f32  / (h - 2*offset) as f32) 
        }else {
            lerp(min_xnum, max_xnum, (n - offset) as f32 / (w - 2*offset) as f32) 
        }
    }
    //----------------------------------

    // draw_graph
    // This function keeps a copy of the previously plotted point, if the new point is too far away from the previous, 
    // it will draw a line between the two (this is to remove the situation where it looks like the function is dotted) 

    // step = How many times it steps, by default it steps by 1 meaening each pixel
    pub fn draw_graph<F: Fn(f32) -> f32 >(&mut self, f: F, step: usize, offset: usize, min_xnum : f32, max_xnum: f32, min_ynum: f32, max_ynum: f32, color: u32 ) {
        let h: usize = self.ctx.h; let w = self.ctx.w;

        let to_number_space = |is_y_component: bool, n: usize| -> f32 {
            Self::to_number_space(h,w,is_y_component, n, [min_xnum, max_xnum, min_ynum,max_ynum], offset)
        };
        let to_window_space = |is_y_component: bool, n: f32| -> usize {
            Self::to_window_space(h,w,is_y_component, n, [min_xnum, max_xnum, min_ynum,max_ynum], offset)
        };

        // Calculate initial point
        // let init_x = to_number_space(false, offset);
        // let wy = to_window_space( true, f(init_x));
        // let mut prev_point: Vec2D<usize> = Vec2D::new(offset, wy);

        let mut prev_point: Option< Vec2D<isize>  > = None;

        for window_x in (offset..=(w - offset)).step_by(step) { 
            // Put x into number space
            let x: f32 = to_number_space(false, window_x);
            let y: f32 = f(x);



            if !(y.is_nan() || y > max_ynum || y < min_ynum) {
                let wx: usize = window_x; let wy = to_window_space(true, y);

                // println!("({x}, {y}) --> ({wx}, {wy})");
                let point = Vec2D::new(wx as isize, wy as isize);

                if let Some(pp) = &prev_point {
                    if point.distance(pp) > 2 {
                        let prev_thickness = self.ctx.thickness;
                        self.ctx.set_thickness(prev_thickness *2);
                        self.ctx.draw_line(&Vec2D::new(pp.x as usize, pp.y as usize), &Vec2D::new(point.x as usize, point.y as usize), color);
                        self.ctx.set_thickness(prev_thickness);
                    }else {
                        let _ = self.ctx.draw_scaled_pixel(wx, wy, self.ctx.thickness, color);
                    }
                    prev_point = Some(point);
                }else {
                    let _ = self.ctx.draw_scaled_pixel(wx, wy, self.ctx.thickness , color);
                    prev_point = Some(point);
                }

            }else {
                prev_point = None;
            }


            
        }
    }

    /// offset = the axis offset
    /// border_offsets = the ranges used in the plot  [xmin, xmax, ymin, ymax ]
    pub fn plot_on_graph(&mut self, point: &Vec2D<f32>, scale:usize, color: u32) -> Result<(), String>{
        let offset = self.settings.axis_offset;
        let border_offsets = self.settings.get_border_offsets();

        let h =self.ctx.h; let w = self.ctx.w;
        let new_x = Self::to_window_space(h, w, false, point.x, border_offsets, offset);
        let new_y = Self::to_window_space(h, w, true, point.y, border_offsets, offset);
        
        self.ctx.draw_scaled_pixel(new_x, new_y, scale, color)
    }

    pub fn plot_dataset(&mut self, points: &Vec<Vec<f32>>, scale: usize, color: u32) {
        for point in points {
            let point = Vec2D::new(point[0] as f32, point[1] as f32);
            self.plot_on_graph(&point, scale, color).unwrap();
        }
    }
}
//*----------------------------------------- */
//*----------------------------------------- */
//*----------------------------------------- */




/// Struct for red-green-b22232222lue-alpha pixels: [red,green,blue,alpha]
pub struct Rgba( pub [u32;4] );
///TODO: RGB to hex conversions


pub struct Hex( pub u32);
impl Hex {
    pub fn from_word(color: &str) -> u32 {
        match color {
            "white" => 0xffffff,
            "grey" => 0xd1d1d1,

            "red" => 0xff0000,
            "blue" => 0x4328ed,
            "green" => 0x008000,

            
            "black" | _ => 0x000000,
            
        }
    }
}




























// struct ctx



//-------------------- Dead zone
//pub fn draw_line(&mut self, start: &Vec2D<usize>, end: &Vec2D<usize>, color: u32) {
    // let thickness = self.thickness.clone();
    // let [x0, y0, x1, y1] = [start.x as i32, start.y as i32, end.x as i32, end.y as i32];

    // let mut x:i32 = x0; let mut y:i32 = y0 as i32;
    // let dx = (x1 - x0).abs();   let sx = if x0<x1{1} else{-1}; // how it increments
    // let dy = - (y1 - y0).abs(); let sy = if y0<y1{1} else{-1};
    // let mut error = dx+dy;

    // loop {
    //     self.draw_scaled_pixel(x as usize,y as usize, thickness, color).unwrap();
    //     if x==x1 && y==y1 {break}
    //     let e2 = 2*error;

    //     if e2 >= dy {
    //         if x==x1{break}
    //         error = error + dy;
    //         x+=sx;
    //     }
    //     if e2 <= dx {
    //         if y==y1{break}
    //         error = error + dx;
    //         y+=sy;
    //     }
    // }