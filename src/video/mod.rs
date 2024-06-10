
use std::error::Error;


use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{self, VideoSubsystem};
use sdl2::ttf;

pub mod font;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
pub struct SdlScene{
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: Canvas<Window>,
    pub event_pump: sdl2::EventPump,
    pub ttf_context: ttf::Sdl2TtfContext,
    pub font_and_metadata: font::FontAndMetadata,
}
impl SdlScene{
    fn build_canvas(window: Window,draw_colour: Color) ->Result<Canvas<Window>,Box<dyn Error>>{
        let mut canvas = window.into_canvas().build()?;
        canvas.set_draw_color(draw_colour);
        Ok(canvas)
    }

    fn build_window(video_subsystem: &VideoSubsystem,name :&str,size: (u32,u32))->Result<Window,Box<dyn Error>>{
        let window = video_subsystem.window(name, size.0, size.1)
            .position_centered()
            .build()?;
        Ok(window)
    }

    pub unsafe fn build()->Result<SdlScene,Box<dyn Error>>{
        let sdl_context = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;

        let ttf_context=sdl2::ttf::init()?;
         
        let window = Self::build_window(&video_subsystem, "baMViz!", (800,600))?;
        
        let canvas=Self::build_canvas(window, Color::RGB(0,0,255))?;
        
        let event_pump = sdl_context.event_pump()?;

        let font_and_metadata=font::FontAndMetadata::build(&ttf_context)?;

        Ok(SdlScene{
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            ttf_context,
            font_and_metadata,
        })
        
    }
}


fn hsi2rgb(color: &[f64]) -> Vec<f64> {
    let h = color[0];
    let s = color[1];
    let i = color[2];

    let h = h % 360.0;
    let h = if h < 0.0 { 360.0 + h } else { h };

    let rgb = match h {
        h if (0.0..120.0).contains(&h) => {
            let h = h.to_radians();

            let b = i * (1.0 - s);
            let r = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let g = 3.0 * i - (r + b);

            vec![r, g, b]
        }
        h if (120.0..240.0).contains(&h) => {
            let h = (h - 120.0).to_radians();

            let r = i * (1.0 - s);
            let g = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let b = 3.0 * i - (r + g);

            vec![r, g, b]
        }
        h if (240.0..360.0).contains(&h) => {
            let h = (h - 240.0).to_radians();

            let g = i * (1.0 - s);
            let b = i * (1.0 + (s * h.cos()) / (std::f64::consts::FRAC_PI_3 - h).cos());
            let r = 3.0 * i - (g + b);

            vec![r, g, b]
        }
        _ => {
            eprintln!("Hsi To Rgb conversion error: Hue must be between 0 and 360");
            vec![0.0, 0.0, 0.0]
        }
    };

    rgb.iter().map(|&x| (x * 255.0).round()).collect()
}





pub fn put_text_somewhere(canvas:&mut Canvas<Window>,font: &ttf::Font,text: &str,pos: (u32,u32)){

    let surface =font
        .render(text)
        .shaded(Color::RGB(0,0,0),canvas.draw_color())
        .unwrap();
    let texture_creator=canvas.texture_creator();
    
    let size=font.size_of(text).unwrap();

    let texture=texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    let _=canvas.copy(&texture, None, Some(rect!(pos.0,pos.1,size.0,size.1)));
    
}



pub fn change_canvas(canvas:&mut Canvas<Window>,i:f64){
    let hsi = [i ,0.5,1.0];
    let colour=hsi2rgb(&hsi);
    canvas.set_draw_color(Color::RGB(colour[0] as u8,colour[1] as u8,colour[2] as u8));
    canvas.clear();
}

pub fn should_quit(event_pump:&mut sdl2::EventPump)->bool{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true
            },
            _ => return false
        }
    }
    return false
}



