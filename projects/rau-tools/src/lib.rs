#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

wit_bindgen::generate!({
    world: "debug",
});

/// The libw/debug host
#[derive(Copy, Clone, Debug)]
pub struct HostDebug {}

impl Guest for HostDebug {
    fn print_i8(value: i8) {
        println!("{}", value)
    }

    fn print_i16(value: i16) {
        println!("{}", value)
    }

    fn print_i32(value: i32) {
        println!("{}", value)
    }

    fn print_i64(value: i64) {
        println!("{}", value)
    }

    fn print_u8(value: u8) {
        println!("{}", value)
    }

    fn print_u16(value: u16) {
        println!("{}", value)
    }

    fn print_u32(value: u32) {
        println!("{}", value)
    }

    fn print_u64(value: u64) {
        println!("{}", value)
    }

    fn print_f32(value: f32) {
        println!("{}", value)
    }

    fn print_f64(value: f64) {
        println!("{}", value)
    }

    fn print_char(value: char) {
        println!("{}", value)
    }

    fn print_str(value: String) {
        println!("{}", value)
    }

    fn print_list_u8(value: Vec<u8>) {
        println!("{:?}", value)
    }
}

export!(HostDebug);
