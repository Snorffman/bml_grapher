use std::iter::{self, Sum};

use bml_grapher::{math::Vec2D, *};
use rand::random;

const BLACK:u32 = 0x000000;
const WHITE:u32 = 0xffffff ;
const RED:u32 = 0xff0800 ;
const GREY: u32 = 0xf5f5f5;  // 0xc2c2c2
const BLUE: u32 = 0x4328ed;
const YELLOW:u32 = 0xf9f034;
const GREEN:u32 =0x1be81b;


const WIDTH: usize = 720; // 480
const HEIGHT: usize = 540; // 540

// TASK: MAKE SPINNY LINE

fn dot<T: std::ops::Mul<Output = T> + Sum + Clone+Copy>(v1: &Vec<T>, v2: &Vec<T>) -> T {
    return v1.iter().enumerate()
        .map(|(i,_)| v1[i] * v2[i] )
        .sum()
}


fn main() {
    let mut window = SnorfWindow::new("Grapher", WIDTH,HEIGHT, None);
    let mut ctx = window.get_context();

    // Axis
    let axoff = 20; // axis offset

    let min_xnum = 0.0; let max_xnum = 10.0;
    let min_ynum = 0.0; let max_ynum = 10.0;
    let border_offsets = [min_xnum,max_xnum,min_ynum,max_ynum];


    let settings = GraphSettings::new(axoff, min_xnum, max_xnum, min_ynum, max_ynum);
    let mut graph_ctx = GraphCtx::new(&mut ctx, settings);


    



    //---------------------------------

    let X = vec![
        vec![2.],
        vec![4.],
        vec![6.],
        vec![7.],
        vec![6.],
        vec![9.],
        vec![0.5],
        vec![0.],
        vec![5.],
        vec![4.],
        vec![0.600615],
    ];
    let y = vec![3., 5., 7.,6.,9.,6.,0.25,0.,5.,2.,0.0001];

    let dataset_plot: Vec<Vec<f32>> = X.iter().zip(&y).map(|(x_row,y)|  [x_row.clone(),vec![*y]].concat() ).collect();

    //-----------------  Hyperparameters ----------------------
    let r1= rand::random::<f32>()*10.; let r2 = rand::random::<f32>()*10.;
    let mut theta = vec![r1,r2];//vec![0., -99.];
    let alpha = 0.01;
    //---------------------------------------------------------

    // 1) Add 1 to the x vectors so theta and x have same dimensions
    let X: Vec<Vec<f32>>= X.iter().map(|row| {
        let mut new_row = Vec::with_capacity(row.len()+1);
        new_row.push(1.); row.iter().for_each(|x| new_row.push(*x));
        return new_row
    }).collect();

    assert_eq!(X[0].len(), theta.len());


    while window.is_open() {
        graph_ctx.ctx.clear_rect(WHITE);

        // Draw axis
        graph_ctx.draw_axis(true, 1.0, min_xnum, max_xnum);
        graph_ctx.draw_axis(false,1.0, min_ynum, max_ynum);

        graph_ctx.plot_dataset(&dataset_plot, 5, BLUE);

        //---------- Create temporary function for the line
        let h = |x:f32| -> f32 {
            theta[0] + theta[1]*x
        };


        graph_ctx.draw_graph(h, 1, axoff, min_xnum, max_xnum, min_ynum, max_ynum, RED);

        //-------- PERFORM GRADIENT DESCENT
        // 2) Tj = Tj - (alpha/n)E( h(x_i) - y_i)x_ij
        let n = X.len();
        // - Update parameters, theta_j
        let mut new_theta = vec![0.; theta.len()];
        for j in 0..theta.len() {
            let mut sum = 0.0;
            for i in 0..n { // Error considers the mean of the results from each entry.
                let dot_prod = dot(&X[i], &theta);
                sum += (dot_prod - y[i])* X[i][j];
            }
            new_theta[j] = theta[j] - (alpha/n as f32) * sum;
        }
        theta = new_theta;

    
        //------------------------------------------------
        let MAE= X.iter().enumerate().map(|(i,x)| (dot(x, &theta) - &y[i]).abs() ).sum::<f32>()/ (n as f32);
        println!("MAE= {}, theta={:?}, ", MAE, theta);

        std::thread::sleep(std::time::Duration::from_millis(10));

        window.update(&graph_ctx.ctx).unwrap();
    }









}



fn linear_regression() {
    // //---------------------------------
    // let X = vec![
    //     vec![5., 10.],
    //     vec![2., 50.],
    //     vec![10.,5.]
    // ];
    // let y = vec![35., 156., 40.];
    // //-----------------  Hyperparameters ----------------------
    // let mut theta = vec![2., 2., 2.];
    // let alpha = 0.0001;
    // //---------------------------------------------------------

    // // 1) Add 1 to the x vectors so theta and x have same dimensions
    // let X: Vec<Vec<f32>>= X.iter().map(|row| {
    //     let mut new_row = Vec::with_capacity(row.len()+1);
    //     new_row.push(1.); row.iter().for_each(|x| new_row.push(*x));
    //     return new_row
    // }).collect();
    // // 2) Tj = Tj - (alpha/n)E( h(x_i) - y_i)x_ij
    // let n = X.len();
    // for iter in 0..5 {
    //     // - Update parameters, theta_j
    //     let mut new_theta = vec![0.; theta.len()];
    //     for j in 0..theta.len() {
    //         let mut sum = 0.0;
    //         for i in 0..n { // Error considers the mean of the results from each entry.
    //             let dot_prod = dot(&X[i], &theta);
    //             sum += (dot_prod - y[i])* X[i][j];
    //         }

    //         new_theta[j] = theta[j] - (alpha/n as f64) * sum;
    //     }
    //     theta = new_theta;

    //     println!("theta={:?}", theta);
    // }
}






    // bias,x,y(label)

    // let X_cringe = vec![
    //     vec![1., 2., 3.],
    //     vec![1., 4.,5.],
    //     vec![1., 6.,7.],
    //     vec![1., 7.,6.],
    //     vec![1., 6.,9.],
    //     vec![1., 9., 6.],
    //     vec![1., 0.5, 0.2533333333333333],
    //     vec![1., 0.,0.0],
    //     vec![1., 5.,5.],
    //     vec![1., 4.,2.],
    //     vec![1., 0.600615,0.0001],
    // ];

    // let X_dataset = vec![
    //     vec![2., 3.],
    //     vec![4.,5.],
    //     vec![6.,7.],
    //     vec![7.,6.],
    //     vec![6.,9.],
    //     vec![9., 6.],
    //     vec![0.5, 0.2533333333333333],
    //     vec![0.,0.0],
    //     vec![5.,5.],
    //     vec![4.,2.],
    //     vec![0.600615,0.0001],
    // ];



    // let mut X = vec![];
    // for i in 0..X_cringe.len() {
    //     X.push(vec![X_cringe[i][0], X_cringe[i][1]]);
    // }

    // let mut y = vec![];
    // for i in 0..X_cringe.len() {
    //     y.push(X_cringe[i][2]);
    // }
    // //? LINE STUFF ------------ 
    // // bias, weight,
    // let mut params = vec![0.,3.];


    // //?------------------------
    // // let fish = 0.0000001;
    // let fish =1.;

    // let mut iteration = 0;
    
    // while window.is_open() && false{
    //     graph_ctx.ctx.clear_rect(WHITE);

    //     // Draw axis
    //     graph_ctx.draw_axis(true, 1.0, min_xnum, max_xnum);
    //     graph_ctx.draw_axis(false,1.0, min_ynum, max_ynum);

    //     graph_ctx.plot_dataset(&X_dataset, 5, BLUE);

    //     //---------- Create temporary function for the line
    //     let h = |x:f32| -> f32 {
    //         params[0]*x + params[1]
    //     };


    //     graph_ctx.draw_graph(h, 1, axoff, min_xnum, max_xnum, min_ynum, max_ynum, RED);

    //     //-------- PERFORM GRADIENT DESCENT
    //     for j in 0..params.len() {

    //         let mut sum = 0.0;
    //         for i in 0..X[0].len() {
    //             // println!("thing = ({}-{})*{}", (dot(&X[i], &params)), (y[i]), X[i][j]);
    //             sum +=  (dot(&X[i], &params) - y[i]) * X[i][j];
    //         } 
    //         let loss_gradient = sum;
    //         params[j] = params[j] - (fish / params.len() as f32) * loss_gradient;
    //     }

    //     println!("ITER: {}", iteration);
    //     iteration += 1;
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     //------------------------------------------------

    //     window.update(&graph_ctx.ctx).unwrap();
    // }