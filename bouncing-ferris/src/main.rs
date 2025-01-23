//! A Rust demo of the QNX Screen API
//!
//! You get a bouncing Ferris moving around the screen at 1 pixel per frame.
//!
//! Copyright (c) Ferrous Systems, 2023

#![deny(warnings)]

use image::io::Reader as ImageReader;

/// Our Ferris graphic, in PNG format.
static IMAGE_BYTES: &[u8] = include_bytes!("ferris.png");

fn main() -> Result<(), anyhow::Error> {
    println!("Rust QNX screen demo.");
    println!("Copyright (c) Ferrous Systems, 2023");

    // Parse the embedded PNG
    let image = ImageReader::new(std::io::Cursor::new(IMAGE_BYTES))
        .with_guessed_format()?
        .decode()?
        .to_rgba8();
    let image_width = image.width() as i32;
    let image_height = image.height() as i32;

    // Create a screen context
    let ctx = qnxscreen::Context::new(
        qnxscreen::ContextType::APPLICATION | qnxscreen::ContextType::INPUT_MANAGER,
    )?;

    let window = ctx.create_window()?;

    // Lock us to 60 fps on a 60 Hz screen
    println!("Set swap interval to 1");
    window.set_swap_interval(1)?;

    // We want permission to draw on the window
    println!("Set window usage to NATIVE | WRITE");
    window.set_usage(qnxscreen::UsageType::NATIVE | qnxscreen::UsageType::WRITE)?;

    // We want double buffering
    println!("Create 2 buffers");
    window.create_buffers(2)?;

    // 32-bit per pixel, with 8-bit alpha
    println!("Set pixel format to RGBA8888");
    window.set_screen_format(qnxscreen::FormatType::RGBA8888)?;

    // How big is the screen?
    println!("Get window size");
    let dimensions = window.get_size()?;
    println!("Screen size: {}", dimensions);

    let mut delta_x: i32 = 1;
    let mut delta_y: i32 = 1;

    // Bouncing image starts at the top left corner of the screen.
    let mut image_pos = qnxscreen::Rect::new((0, 0), image_width - 1, image_height - 1);

    // Run forever
    loop {
        let buffers = window.get_buffers()?;
        // Because we post to the screen with PostFlags::IDLE, buffers[0] is
        // always the best buffer to use.
        let stride = buffers[0].get_stride()?;
        let buffer_ptr = buffers[0].get_pointer()?;

        // Do the bounce
        if image_pos.x1() == 0 {
            delta_x = 1;
        }
        if image_pos.x2() == dimensions.width() - 1 {
            delta_x = -1;
        }
        if image_pos.y1() == 0 {
            delta_y = 1;
        }
        if image_pos.y2() == dimensions.height() - 1 {
            delta_y = -1;
        }

        // Do the slide
        image_pos.adjust_position(delta_x, delta_y);

        // Manually blit the image to the screen.
        //
        // Really we should create a small 'window' containing just the image,
        // then move the window around the screen. Then we could have multiple
        // windows sliding around.
        for y in 0..image_height {
            let screen_y = (image_pos.y1() + y) as usize;
            let screen_row = unsafe { buffer_ptr.add(screen_y * stride) };
            for x in 0..image_width {
                let im_px = image.get_pixel(x as u32, y as u32);
                let screen_x = ((image_pos.x1() + x) as usize) * 4;
                unsafe {
                    screen_row.add(screen_x).write(im_px.0[2]);
                    screen_row.add(screen_x + 1).write(im_px.0[1]);
                    screen_row.add(screen_x + 2).write(im_px.0[0]);
                    screen_row.add(screen_x + 3).write(im_px.0[3]);
                }
            }
        }

        // Update the screen with the image we just drew.
        window.post(&buffers[0], &[image_pos], qnxscreen::PostFlags::IDLE)?;
    }
}
