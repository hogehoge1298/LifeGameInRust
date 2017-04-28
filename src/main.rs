extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate life_game_in_rust;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use life_game_in_rust::Rect;

pub struct App{
    gl: GlGraphics,
    rotation: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs)
    {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 24.0);
        let rotation = self.rotation;
        let (w, h) = ((args.width) as f64, (args.height) as f64);

        self.gl.draw(args.viewport(), |c, gl|{
            //Clear the screen.
            clear(GREEN, gl);
            let wm = w as i32 ;
            let hm = h as i32 ;
            'outer: for x in 0..wm {
                'inner: for y in 0..hm {
                    let _x = (x * 25) as f64;
                    let _y = (y * 25) as f64;
                    let transform = c.transform.trans(_x, _y).rot_rad(rotation).trans(-25.0,-25.0);
                    rectangle(RED, square, transform, gl);
                }
            }
        });
    }
}

fn main(){
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [500, 500]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App{
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            app.render(&r);
        }
    }
}