pub mod compute;
extern crate sdl2;
use std::thread;
use sdl2::{pixels::Color, event::Event};
use std::time::Duration;
use sdl2::rect::{Rect};

fn calculate_color(x_position: f32, y_position : f32, size : u32) -> Color{
    let new_size = (size / 2) as f32;
    let red : f32;
    let green : f32;
    let blue : f32;

    red = (1.0 - (((x_position - new_size).abs()) / new_size)) * 255.0;
    green = (1.0 - (((y_position - new_size).abs()) / new_size)) * 255.0;
    blue = (1.0 - (((((x_position + y_position) / 2.0) - new_size).abs()) / new_size)) * 255.0;

    return Color::RGB(red as u8, green as u8, blue as u8); 
}

fn main (){
    // Create sdl context
    let sdl_context = sdl2::init().unwrap();
    // Creat sdl video subsystem
    let video_subsystem = sdl_context.video().unwrap();

    // Window size
    let window_size = 800;

    // Create and configure initial window
    let window = video_subsystem.window("Game of Life", window_size, window_size)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

    // Create and configure renderer to set background as black
    let mut canvas = window.into_canvas().build().unwrap();
    
    // Cleaning the canvas with black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Base Matrix that the game follows
    let mut matrix = compute::populate_matrix(compute::create_matrix());
    let rectangle_size = window_size / 198;
    let rect_size = rectangle_size as i32;
    
    // Keybord controller
    let mut event_log = sdl_context.event_pump().unwrap();

    'main_loop: loop{
        // Cleaning the canvas with black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // New things drawed as white
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for columns in 1..matrix.len()-1{
            for rows in 1..matrix[columns].len()-1{
                if matrix[columns][rows] == 1 {
                    //Creating rect_example, drawing and filling.
                    let x_position = (columns as i32) * rect_size;
                    let y_position = (rows as i32) * rect_size;
                    let alive_rect = Rect::new(x_position, y_position, rectangle_size, rectangle_size);
                    canvas.set_draw_color(calculate_color(x_position as f32, y_position as f32, window_size));
                    canvas.draw_rect(alive_rect);
                }
            }
        }
        
        // Showing the canvas.
        canvas.present();
        thread::sleep(Duration::new(0, 6000000));

        // Computes the next cylce
        matrix = compute::compute_matrix(matrix);

        // Capture event to quit window if user clicks
        for event in event_log.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    break 'main_loop;
                }
                _ => {}
            }
        }

    }
}