extern crate clap;
extern crate image;
extern crate qrcode;

use clap::{App, Arg};
use image::Luma;
use qrcode::QrCode;

fn main() {
    let args = App::new("grcode")
        .version("0.1")
        .about("generates qrcode")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .help("Url for qrcode")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("location")
                .short("l")
                .long("location")
                .help("Place to save image")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let url = args.value_of("url").unwrap();
    let location = args.value_of("location").unwrap();

    let code = QrCode::new(url).unwrap();
    let image = code.render::<Luma<u8>>().build();


    image.save(location).unwrap();
}
