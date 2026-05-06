//! A Rust demo of the QNX Screen API
//!
//! You get a bouncing Ferris moving around the screen at 1 pixel per frame.
//!
//! Copyright (c) Ferrous Systems, 2023

#![deny(warnings)]

use image::io::Reader as ImageReader;

/// Our Ferris graphic, in PNG format.
static IMAGE_BYTES: &[u8] = include_bytes!("ferris.png");

/// Number of sprites on screen
const NUM_SPRITES: usize = 10;

fn main() -> Result<(), anyhow::Error> {
    println!("Rust QNX screen demo.");
    println!("Copyright (c) Ferrous Systems, 2023");

    // Parse the embedded PNG
    let image = ImageReader::new(std::io::Cursor::new(IMAGE_BYTES))
        .with_guessed_format()?
        .decode()?
        .to_rgba8();

    // Create a screen context
    let ctx = qnxscreen::Context::new(
        qnxscreen::ContextType::APPLICATION | qnxscreen::ContextType::INPUT_MANAGER,
    )?;

    let displays = ctx.get_displays()?;
    let dimensions = displays[0].get_size()?;
    println!("Screen size: {:?}", dimensions);
    let root_window = ctx.create_window()?;
    root_window.set_swap_interval(2)?;
    root_window.set_usage(qnxscreen::UsageType::READ | qnxscreen::UsageType::WRITE)?;
    root_window.create_buffers(1)?;
    root_window.set_id("root")?;
    root_window.set_position(0, 0)?;
    root_window.set_screen_format(qnxscreen::FormatType::RGBA8888)?;
    root_window.set_visible(true)?;
    root_window.set_static(true)?;
    let buffers = root_window.get_buffers()?;
    // Because we post to the window each frame, buffers[0] is always the
    // best buffer to use.
    let stride = buffers[0].get_stride()?;
    let buffer_ptr = buffers[0].get_pointer()?;
    for y in 0..dimensions.1 {
        let screen_row: *mut u32 = unsafe { buffer_ptr.add((y as usize) * stride).cast() };
        for x in 0..dimensions.0 as usize {
            // Dark blue background
            unsafe {
                screen_row.add(x).write_volatile(0xFF000033);
            }
        }
    }
    // Post to make it show up
    root_window.post(&buffers[0], &[], qnxscreen::PostFlags::empty())?;
    root_window.create_group(None)?;
    let group_name = root_window.get_group_name()?;
    println!("Group name: {:?}", group_name);

    let mut sprites = Vec::new();

    for sprite_num in 0..NUM_SPRITES {
        sprites.push(Sprite::new(
            &ctx,
            &group_name,
            &format!("sprite{}", sprite_num),
            rand_pos(&dimensions),
            &image,
        )?);
    }

    // Track our frame rate
    let mut fps = FpsCounter::new();

    // Run forever
    'eventloop: loop {
        if let Some(ev_type) = ctx.get_event(None)? {
            use qnxscreen::events::EventType;
            match ev_type {
                EventType::Create(_create_ev) => {
                    println!("Got EventType::Create");
                }
                EventType::Property(property_ev) => {
                    println!("Got EventType::Property name={:?}", property_ev.get_name());
                }
                EventType::Close(_close_ev) => {
                    println!("Got EventType::Close");
                }
                EventType::Input(_input_ev) => {
                    println!("Got EventType::Input");
                }
                EventType::Jog(_jog_ev) => {
                    println!("Got EventType::Jog");
                }
                EventType::Pointer(pointer_ev) => {
                    println!("Got EventType::Pointer:");
                    let buttons = pointer_ev.get_buttons()?;
                    println!("\tbuttons: 0x{:x}", buttons);
                    println!("\tdevice=0x{:08x}", pointer_ev.get_device()?);
                    println!("\tmodifiers: {}", pointer_ev.get_modifiers()?);
                    println!(
                        "\tmouse_horizontal_wheel: {}",
                        pointer_ev.get_mouse_horizontal_wheel()?
                    );
                    println!("\tmouse_wheel: {}", pointer_ev.get_mouse_wheel()?);
                    println!("\tposition: {:?}", pointer_ev.get_position()?);
                    println!("\tsource_position: {:?}", pointer_ev.get_source_position()?);
                    if buttons.contains(qnxscreen::events::MouseButtons::LEFT) {
                        println!("Left button pressed, exiting demo...");
                        break 'eventloop;
                    }
                }
                EventType::Keyboard(keyboard_ev) => {
                    println!("Got EventType::Keyboard");
                    println!("\tdevice=0x{:08x}", keyboard_ev.get_device()?);
                    println!("\tkeycap=0x{:04x}", keyboard_ev.get_keycap()?);
                    println!("\tmodifiers={:?}", keyboard_ev.get_modifiers()?);
                    println!("\tscan=0x{:02x}", keyboard_ev.get_scan()?);
                    println!("\tsym={:?}", keyboard_ev.get_sym()?);
                    if keyboard_ev.get_keycap()? == qnxscreen::events::EventKeyboard::KEYCODE_ESCAPE
                    {
                        println!("ESC pressed, quitting");
                        break 'eventloop;
                    }
                }
                EventType::User(_user_ev) => {
                    println!("Got EventType::User");
                }
                EventType::Post(_post_ev) => {
                    println!("Got EventType::Post");
                }
                EventType::Display(_display_ev) => {
                    println!("Got EventType::Display");
                }
                EventType::Idle(_idle_ev) => {
                    println!("Got EventType::Idle");
                }
                EventType::Unrealize(_unrealize_ev) => {
                    println!("Got EventType::Unrealize");
                }
                EventType::Gamepad(_gamepad_ev) => {
                    println!("Got EventType::Gamepad");
                }
                EventType::Joystick(_joystick_ev) => {
                    println!("Got EventType::Joystick");
                }
                EventType::Device(_device_ev) => {
                    println!("Got EventType::Device");
                }
                EventType::InputControl(_inputcontrol_ev) => {
                    println!("Got EventType::InputControl");
                }
                EventType::Gesture(_gesture_ev) => {
                    println!("Got EventType::Gesture");
                }
                EventType::Manager(_manager_ev) => {
                    println!("Got EventType::Manager");
                }
                EventType::MultiTouchPretouch(_multitouchpretouch_ev) => {
                    println!("Got EventType::MultiTouchPretouch");
                }
                EventType::MultiTouchTouch(_multitouchtouch_ev) => {
                    println!("Got EventType::MultiTouchTouch");
                }
                EventType::MultiTouchMove(_multitouchmove_ev) => {
                    println!("Got EventType::MultiTouchMove");
                }
                EventType::MultiTouchRelease(_multitouchrelease_ev) => {
                    println!("Got EventType::MultiTouchRelease");
                }
            }
        }

        // Do the bounce
        for sprite in sprites.iter_mut() {
            if sprite.pos.x1() == 0 {
                sprite.delta_x = 1;
            }
            if sprite.pos.x2() >= dimensions.0 {
                sprite.delta_x = -1;
            }
            if sprite.pos.y1() == 0 {
                sprite.delta_y = 1;
            }
            if sprite.pos.y2() >= dimensions.1 {
                sprite.delta_y = -1;
            }

            // Do the slide
            sprite.pos.adjust_position(sprite.delta_x, sprite.delta_y);

            // Adjust the sprite
            sprite
                .window
                .set_position(sprite.pos.x1(), sprite.pos.y1())?;
        }

        // Update the screen with the image we just drew.
        ctx.flush(true)?;
        fps.tick();
    }

    println!("Bye!");

    Ok(())
}

/// Represents a graphical object we can move around the screen
struct Sprite<'ctx> {
    pos: qnxscreen::Rect,
    window: qnxscreen::Window<'ctx>,
    delta_x: i32,
    delta_y: i32,
}

impl<'ctx> Sprite<'ctx> {
    fn new(
        ctx: &'ctx qnxscreen::Context,
        group_name: &str,
        new_window_id: &str,
        position: (i32, i32),
        source: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Result<Sprite<'ctx>, anyhow::Error> {
        let image_width = source.width() as usize;
        let image_height = source.height() as usize;

        // Convert the image from RGBA tuples to ARGB packed u32s
        let mut converted_image = Vec::new();
        for y in 0..image_height {
            for x in 0..image_width {
                let im_px: &image::Rgba<u8> = source.get_pixel(x as u32, y as u32);
                // We have (R, G, B, A) and we want ARGB: u32
                let bytes = [
                    im_px.0[3], // Alpha
                    im_px.0[0], // Red
                    im_px.0[1], // Green
                    im_px.0[2], // Blue
                ];
                let word = u32::from_be_bytes(bytes);
                converted_image.push(word);
            }
        }

        let window = ctx.create_child_window()?;

        // We want permission to draw on the window
        println!("Set window usage to READ | WRITE");
        window.set_usage(qnxscreen::UsageType::READ | qnxscreen::UsageType::WRITE)?;

        // Set the name, which appears in /dev/screen
        println!("Set window name");
        window.set_id(new_window_id)?;

        println!("Set window visible");
        window.set_visible(true)?;

        println!("Set window position");
        window.set_position(position.0, position.1)?;

        println!("Join window group");
        window.join_group(&group_name)?;

        // Match the image size
        println!("Set window size");
        window.set_static(true)?;
        window.set_size(image_width as i32, image_height as i32)?;
        window.set_buffer_size((image_width as i32, image_height as i32))?;

        // We don't need double buffering
        println!("Create 1 buffer");
        window.create_buffers(1)?;

        // 32-bit per pixel, with 8-bit alpha
        println!("Set pixel format to RGBA8888");
        window.set_screen_format(qnxscreen::FormatType::RGBA8888)?;

        println!("Get window buffers");
        let buffers = window.get_buffers()?;
        // Because we post to the window each frame, buffers[0] is always the
        // best buffer to use.
        println!("Get window stride");
        let stride = buffers[0].get_stride()?;
        println!("Get window pointer");
        let buffer_ptr = buffers[0].get_pointer()?;
        println!("Got window pointer {buffer_ptr:p}");
        for y in 0..image_height {
            let buffer_row: *mut u32 = unsafe { buffer_ptr.add(y * stride).cast() };
            let image_row = &converted_image[y * image_width..(y + 1) * image_width];
            for x in 0..image_width as usize {
                let pixel = image_row[x];
                unsafe {
                    buffer_row.add(x).write_volatile(pixel);
                }
            }
        }

        println!("Window post");
        // need to post to make it visible
        window.post(&buffers[0], &[], qnxscreen::PostFlags::empty())?;
        println!("Sprite built!");

        Ok(Sprite {
            window,
            pos: qnxscreen::Rect::new(position, image_width as i32, image_height as i32),
            delta_x: 1,
            delta_y: 1,
        })
    }
}

struct FpsCounter {
    last_time: std::time::Instant,
    frames: u32,
}

impl FpsCounter {
    const PRINT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

    /// Make a new FPS counter
    fn new() -> FpsCounter {
        FpsCounter {
            last_time: std::time::Instant::now(),
            frames: 0,
        }
    }

    /// Call this once per frame.
    ///
    /// Every `PRINT_INTERVAL` it will print the FPS and CPU usage to stdout.
    fn tick(&mut self) {
        let delta = self.last_time.elapsed();
        if self.frames == 0 {
            self.last_time = std::time::Instant::now();
            self.frames += 1;
        } else {
            if delta >= Self::PRINT_INTERVAL {
                let fps = self.frames as f64 / delta.as_secs_f64();
                println!(
                    "{frames} frames in {delta_ms} ms is {fps:2.1} fps",
                    frames = self.frames,
                    delta_ms = delta.as_millis(),
                );
                self.frames = 0;
            } else {
                self.frames += 1;
            }
        }
    }
}

getrandom::register_custom_getrandom!(qnx_random);

fn qnx_random(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    use std::io::prelude::*;
    let mut dev = std::fs::File::open("/dev/urandom").map_err(|_e| {
        let open_error = std::num::NonZeroU32::new(getrandom::Error::CUSTOM_START + 0).unwrap();
        getrandom::Error::from(open_error)
    })?;
    dev.read_exact(buf).map_err(|_e| {
        let read_error = std::num::NonZeroU32::new(getrandom::Error::CUSTOM_START + 1).unwrap();
        getrandom::Error::from(read_error)
    })?;
    Ok(())
}

/// Generate a random position on the screen
fn rand_pos(dimensions: &(i32, i32)) -> (i32, i32) {
    use rand::distr::{Uniform, Distribution};
    let x_rand = Uniform::try_from(0..dimensions.0).unwrap();
    let y_rand = Uniform::try_from(0..dimensions.1).unwrap();
    let mut rng = rand::rng();
    (x_rand.sample(&mut rng), y_rand.sample(&mut rng))
}
