use std::{
    ffi::{c_int, c_uchar, c_void},
    ptr::null_mut,
};

use crate::truetype::stbtt__buf;

/// The following structure is defined publicly so you can declare one on
/// the stack or as a global or etc, but you should treat it as opaque.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct stbtt_fontinfo {
    #[allow(missing_docs)]
    pub userdata: *mut c_void,

    /// pointer to .ttf file
    pub data: *mut c_uchar,

    /// offset of start of font
    pub fontstart: c_int,

    /// number of glyphs, needed for range checking
    pub num_glyphs: c_int,

    /// table locations as offset from start of .ttf
    pub loca: c_int,

    /// table locations as offset from start of .ttf
    pub head: c_int,

    /// table locations as offset from start of .ttf
    pub glyf: c_int,

    /// table locations as offset from start of .ttf
    pub hhea: c_int,

    /// table locations as offset from start of .ttf
    pub hmtx: c_int,

    /// table locations as offset from start of .ttf
    pub kern: c_int,

    /// table locations as offset from start of .ttf
    pub gpos: c_int,

    /// table locations as offset from start of .ttf
    pub svg: c_int,

    /// a cmap mapping for our chosen character encoding
    pub index_map: c_int,

    /// format needed to map from glyph index to glyph
    pub index_to_loc_format: c_int,

    /// cff font data
    pub cff: stbtt__buf,

    /// the charstring index
    pub charstrings: stbtt__buf,

    /// global charstring subroutines index
    pub gsubrs: stbtt__buf,

    /// private charstring subroutines index
    pub subrs: stbtt__buf,

    /// array of font dicts
    pub fontdicts: stbtt__buf,

    /// map from glyph to fontdict
    pub fdselect: stbtt__buf,
}

impl Default for stbtt_fontinfo {
    fn default() -> Self {
        stbtt_fontinfo {
            userdata: null_mut(),
            data: null_mut(),
            fontstart: 0,
            num_glyphs: 0,
            loca: 0,
            head: 0,
            glyf: 0,
            hhea: 0,
            hmtx: 0,
            kern: 0,
            gpos: 0,
            svg: 0,
            index_map: 0,
            index_to_loc_format: 0,
            cff: stbtt__buf::default(),
            charstrings: stbtt__buf::default(),
            gsubrs: stbtt__buf::default(),
            subrs: stbtt__buf::default(),
            fontdicts: stbtt__buf::default(),
            fdselect: stbtt__buf::default(),
        }
    }
}
