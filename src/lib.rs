use std::time::Duration;

use video::font;
pub mod video;


pub fn run(mut sdl_scene: video::SdlScene){
    let mut i:f64=0.0;
    'running: loop {
        i +=1.0;
        video::change_canvas(&mut sdl_scene.canvas, i);
        video::put_text_somewhere(&mut sdl_scene.canvas, & sdl_scene.font_and_metadata.plaintext_font, "Hello party people!",(200,200));
        video::put_text_somewhere(&mut sdl_scene.canvas, & sdl_scene.font_and_metadata.music_font, font::get_staff_text().to_string().as_str(),(100,100));
        sdl_scene.canvas.present();
        if video::should_quit(&mut sdl_scene.event_pump){
            break 'running;
        }
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}