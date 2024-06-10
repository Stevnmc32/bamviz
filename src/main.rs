pub mod video;

use std::process;


fn main() {
    let sdl_scene=bamviz::video::SdlScene::build().unwrap_or_else(|err|{
        eprintln!("Error loading Sdl scene: {err}");
        process::exit(1);
    });

    
    
    bamviz::run(sdl_scene);
}
