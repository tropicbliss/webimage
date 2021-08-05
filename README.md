# webimage

A simple image resizer built for the web. If any of the features are oddly specific, it is because it is meant for my personal use. I was trying to scale images to match different screen sizes for my websites and wished there was an automated script available that allowed me to resize images easily and quickly. I decided to share it and hopefully you'll find this utility helpful.

## Command line arguments

Run `webimage --help` or `./webimage --help`.

## Compiling from source

1. Download the `rustup` toolchain right [here](https://rustup.rs/). Follow the instructions for your platform.
2. Run `git clone https://github.com/tropicbliss/webimage.git` in an appropriate directory to clone the repo.
3. In the folder named `webimage`, run `cargo build --release`. The resulting executable file after compilation should be in the `target/release/` directory relative from the `webimage` folder. If you encounter any errors throughout the compilation process, read through the errors as they generally tell you exactly what to do.
