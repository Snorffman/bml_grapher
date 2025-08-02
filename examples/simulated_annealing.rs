#![allow(unused_must_use, nonstandard_style)]
extern crate bml_grapher;
use rand::{rng, Rng};

use bml_grapher::{Rgba, SnorfWindow, math::Vec2D, Ctx};

const WIDTH: usize = 720; // 480
const HEIGHT: usize = 540; // 540

const BLACK:u32 = 0x000000;
const WHITE:u32 = 0xffffff ;
const RED:u32 = 0xff0800 ;
const GREY: u32 = 0xf5f5f5;  // 0xc2c2c2
const BLUE: u32 = 0x4328ed;
const YELLOW:u32 = 0xf9f034;
const GREEN:u32 =0x1be81b;



//--------------------------------------------
#[derive(Clone, Copy)]
struct Node {
    x : f32,  // x position of the node
}
impl Node {
    fn new(x: f32) -> Self  {
        Node{x}
    }

    fn get_children(&self) -> Vec<Node> {
        // Return adjacesnt nodes
        let x = self.x;
        vec![Node::new(x -0.1), Node::new(x+0.1),Node::new(x -0.2),Node::new(x +0.2),
        Node::new(x-0.5), Node::new(x + 0.5)]
    }
    // Maximises the child according to evaluation function f.
    fn get_best_child<F: Fn(f32) -> f32>(children: &Vec<Node>, f: &F) -> Node {
        assert!( children.len() > 0);

        let mut best = children[0];

        for i in 1..children.len() {
            let child = children[i];
            if f(child.x) > f(best.x) {
                best = child;
            }
        }
        best
    }
}


fn hill_climbing<F: Fn(f32) -> f32>(window: &mut SnorfWindow, ctx: &mut Ctx, offset:usize, border_offsets: [f32;4],start_node: Node, f: F) -> Node  {
    let mut current = start_node;

    loop {
        // ---------------- Display stuff
        ctx.plot_on_graph(&Vec2D::new(current.x, f(current.x)), 
            5, BLUE, offset, border_offsets);

        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("({},{})", current.x, f(current.x));
        //------------------


        let neighbors = current.get_children();

        let best_neighbor = Node::get_best_child(&neighbors, &f);
        
        if f(best_neighbor.x) <= f(current.x) {
            ctx.plot_on_graph(&Vec2D::new(current.x, f(current.x)), 
            5, GREEN, offset, border_offsets);
            return current
        }
        window.update(&ctx).unwrap();


        current = best_neighbor;
    }

}




// Make sure the evaluation function is minimal when out of bounds
fn simulated_annealing<F: Fn(f32) -> f32>(window: &mut SnorfWindow, ctx: &mut Ctx, offset:usize, border_offsets: [f32;4],start_node: Node, f: F) -> Node  {
    let mut current = start_node;
    
    //? Shedule determines the value of temperature T as a function of time
    // takes 20 iterations to finish
    let schedule = |t: f32| 10.0/(t+1.0) - 0.5;

    let mut t = 0.0; // time
    loop {
        // ---------------- Display stuff
        ctx.plot_on_graph(&Vec2D::new(current.x, f(current.x)), 
            5, BLUE, offset, border_offsets);

        std::thread::sleep(std::time::Duration::from_millis(100));
        //------------------

        //? The algorithm:

        // 1) Get the temperature
        let T = schedule(t);
        if T <= 0.0 {return current}

        // 2) Select a random neighbor
        let neighbors = current.get_children();
        let rand_i = rng().random_range(0..neighbors.len()); 
        let next = neighbors[rand_i];

        // Difference in energy
        let de = f(next.x) - f(current.x); 

        if de > 0.0 { // if next > current
            println!("next>current: pick random");
            current = next;
        }else {
            // current = next only with probability e^(de/T)
            let random_variable: f32 = rng().random_range(0.0..=1.0);
            let acceptance_region: f32 = (de/T).exp(); 
            // the higher the difference the less likely it is to shake (we prefer to shake on places with low difference eg a local minima)
            // only applies If the difference is negative


            if random_variable < acceptance_region {
                println!("shake, p(shake) = {}", (de/T).exp() );
                current = next;
                
            }else {
                println!("do nothing");
            }
        }
        println!("({},{}), t={t}, T={T}, P={}\n", current.x, f(current.x), (de/T).exp() );

        t += 0.5;

        window.update(&ctx).unwrap();

    }

}
//--------------------------------------------

// fn draw_graph() {

// }




fn main() {
    let mut window = SnorfWindow::new("Grapher", WIDTH,HEIGHT, None);
    let mut ctx = window.get_context();

    // Axis
    let axoff = 20; // axis offset

    let min_xnum = 0.0; let max_xnum = 10.0;
    let min_ynum = 0.0; let max_ynum = 10.0;
    let border_offsets = [min_xnum,max_xnum,min_ynum,max_ynum];

    let f = |x: f32| {
        if x < 0.0|| x > max_xnum {0.0} // partwise function
        else {   0.5* (x-3.0) * (2.0*x-2.0).sin() + 5.0   }
    };

    // let node = Node::

    while window.is_open() {
        ctx.clear_rect(WHITE);

        ctx.set_thickness(2);

        // ctx.draw_line(&[50,50].into(), &[100,100].into(), RED);
        // ctx.draw_text(&[200,200].into(), "Yo", 1);



        // ------------------ Draw the plotter
        
        ctx.draw_axis(true, axoff, axoff /2, 1.0, min_xnum, max_xnum);
        ctx.draw_axis(false, axoff,  axoff /2, 1.0, min_ynum, max_ynum);

        ctx.draw_graph(f, 1, axoff, 
            min_xnum, max_xnum,
            min_ynum, max_ynum, 
            RED
        );

        simulated_annealing(&mut window,&mut ctx, axoff, 
            [min_xnum,max_xnum,min_ynum,max_ynum],
            Node::new(2.0), f
        );

        // ctx.plot_on_graph(Vec2D::new(5.0,5.0), 3, BLUE, axoff, border_offsets);


        window.update(&ctx).unwrap();
    }
}
