pub fn hello(){
    println!("hello, world");
}

pub struct RectActor{
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: [f32; 4],
    pub rotation: f64
}

impl RectActor {
     pub fn new(_x:f64, _y:f64, _w: f64, _h:f64, _c:[f32;4], _r:f64) -> Self
     {
         RectActor{
             x: _x,
             y: _y,
             width: _w,
             height: _h,
             color: _c,
             rotation: _r
         }
     }
}