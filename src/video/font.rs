use std::error::Error;

use sdl2::rwops;
use sdl2::ttf;
use sdl2::ttf::Font;
use smufl;



pub struct FontAndMetadata{
    pub music_font: ttf::Font<'static,'static>,
    pub plaintext_font: ttf::Font<'static,'static>,
    pub smufl_metadata: smufl::Metadata,
}


impl FontAndMetadata {

    fn load_font<'a>(ttf_context:&'a ttf::Sdl2TtfContext, font_as_bytes: &'static[u8])->Result<Font<'a, 'static>,String>{
        let font_as_rwops=rwops::RWops::from_bytes(font_as_bytes)?;
        ttf_context.load_font_from_rwops(font_as_rwops, 32)
    }

    pub unsafe fn build<'a>( ttf_context: &'a ttf::Sdl2TtfContext)-> Result<FontAndMetadata,Box<dyn Error>>{
        
        
        let music_font=std::mem::transmute::<Font,Font<'static,'static>>(Self::load_font(ttf_context,include_bytes!("..\\..\\res\\Leland.ttf"))?);
        
        let plaintext_font=std::mem::transmute::<Font,Font<'static,'static>>(Self::load_font(ttf_context,include_bytes!("..\\..\\res\\Edwin-Roman.ttf"))?);

        let metadata_as_bytes=include_bytes!("..\\..\\res\\leland_metadata.json");
        let smufl_metadata=smufl::Metadata::from_reader(&metadata_as_bytes[..])?;

        Ok(FontAndMetadata{
            music_font,
            plaintext_font,
            smufl_metadata,
        })
    }
    
}

pub fn get_staff_text()->char{
    smufl::Glyph::GClef.codepoint()
}