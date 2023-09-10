use clap::Parser;
use image::{GenericImageView, Pixel};

const EXTENDED_ASCII_LIST: [char; 69] = ['$','@','B','%','8','&','W','M','#','*','o','a','h','k','b','d','p','q','w','m','Z','O','0','Q','L','C','J','U','Y','X',
'z','c','v','u','n','x','r','j','f','t','/', '\\', '|','(',')','1','{','}','[',']','?','-','_','+','~','<','>','i','!','l','I',';',':',',','"','^','`','.', ' '];
const REDUCED_ASCII_LIST: [char; 14] = ['$','@','%','&','#','/', '\\', '|','(',')','~',',','.', ' '];
const MAX_BRIGHTNESS: u32 = 255;


#[derive(Parser)]
struct Cli {
    /// The path to the image file to read
    #[arg(long)]
    path: std::path::PathBuf, 

    /// number of ascii characters per line
    #[arg(long)]
    #[clap(default_value_t=64)]
    width: u32, 

    /// number of ascii characters per column
    #[clap(default_value_t=64)]
    #[arg(long)]
    height: u32, 

    /// use a larger list of 69 ascii characters (default is 14 characters)
    #[clap(default_value_t=false)]
    #[arg(long)]
    use_extended_char_list: bool, 
    
    /// higher values will result in more dark characters and lower values will result in more light characters
    #[clap(default_value_t=0.8)]
    #[arg(long)]
    char_bias: f64, 
}

fn get_pixel_brightness(pixel: image::Rgba<u8>) -> u32 {
    pixel.to_luma().0[0] as u32
}

fn main() {
    let args = Cli::parse();

    // Handle args
    let img = image::open(args.path).expect("File not found!");
    let ascii_list: Vec<char>;
    if args.use_extended_char_list {
        ascii_list = EXTENDED_ASCII_LIST.to_vec();
    }
    else {
        ascii_list = REDUCED_ASCII_LIST.to_vec();
    }
    let char_bias = args.char_bias;

    // Get the dimensions of the image and the dimensions of the ascii art
    let (img_width, img_height) = img.dimensions();
    let ascii_width = std::cmp::min(args.width, img_width);
    let ascii_height = std::cmp::min(args.height, img_height);
    let (patch_width, patch_height) = (img_width / ascii_width, img_height / ascii_height);
    let (num_patches_x, num_patches_y) = (img_width / patch_width, img_height / patch_height);

    // Iterate through the patches of the image and print the ascii character corresponding to the brightness of the patch
    for y in 0..num_patches_y {
        let mut line: String = String::new();
        for x in 0..num_patches_x {
            // Get the sum of the brightness of all the pixels in the patch
            let mut patch_brightness = img.view(x * patch_width, y * patch_height, patch_width, patch_height).pixels().
                fold(0, |val, p| val + get_pixel_brightness(p.2));
            // Get average of brightness in patch
            patch_brightness /= patch_width * patch_height;

            // Map the brightness of the patch to the index of an ascii character
            // char_bias is used to bias the mapping towards darker or lighter characters (higher = darker, lower = lighter)
            let index = ((patch_brightness as f64 / MAX_BRIGHTNESS as f64).powf(char_bias) * (ascii_list.len() - 1) as f64).round() as usize;

            let ascii_char = ascii_list[index];
            line.push(ascii_char);
        }
        println!("{}", line);
    }
}