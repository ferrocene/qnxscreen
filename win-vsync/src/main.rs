//! Window based VSync demo for QNX Screen
//!
//! Based on QNX example `win-vsync`.
//!
//! Ported to Rust by Ferrous Systems
//!
//! Copyright  2010, QNX Software Systems Ltd.  All Rights Reserved
//!
//! This source code has been published by QNX Software Systems Ltd.
//! (QSSL).  However, any use, reproduction, modification, distribution
//! or transfer of this software, or any software which includes or is
//! based upon any of this code, is only permitted under the terms of
//! the QNX Open Community License version 1.0 (see licensing.qnx.com for
//! details) or as otherwise expressly authorized by a written license
//! agreement from QSSL.  For more information, please email licensing@qnx.com.

const BAR_WIDTH: i32 = 10;

fn main() -> Result<(), anyhow::Error> {
    let screen_ctx = qnxscreen::Context::new(qnxscreen::ContextType::APPLICATION)?;

    let displays = screen_ctx.get_displays()?;

    let dimensions = displays[0].get_size()?;

    println!("Display is {} x {}", dimensions.0, dimensions.1);

    let bg_window = create_bg_window(&screen_ctx, dimensions)?;
    let group = bg_window.get_group_name()?;

    let bar_window = create_bar_window(&screen_ctx, &group, "bar")?;
    let hg_window = create_hg_window(&screen_ctx, &group, "hourglass")?;

    let mut bar_x_pos = 0;
    let mut seen_bar = false;
    let mut seen_hg = false;
    loop {
        'event: loop {
            let timeout = if seen_bar && seen_hg {
                None
            } else {
                Some(std::time::Duration::from_secs(1))
            };
            let Some(ev_type) = screen_ctx.get_event(timeout)? else {
                break 'event;
            };

            match ev_type {
                qnxscreen::events::EventType::Post(post) => {
                    let post_window = post.get_window()?;
                    let id = post_window.get_id()?;
                    drop(post_window);
                    if id == "bar" {
                        println!("Got a post event for the bar window");
                        seen_bar = true;
                    } else if id == "hourglass" {
                        println!("Got a post event for the hourglass window");
                        seen_hg = true;
                    } else {
                        println!("Got a post event for another window? id={:?}", id);
                    }

                    if seen_bar && seen_hg {
                        println!("Got both windows, making things visible!");
                        let size = hg_window.get_buffer_size()?;
                        hg_window.set_size(size.0, size.1)?;
                        hg_window.set_position(10, 10)?;

                        bar_window.set_position(0, 0)?;
                        bar_window.set_size(BAR_WIDTH, dimensions.1)?;

                        bg_window.set_size(dimensions.0, dimensions.1)?;

                        let mut z_order = 0;
                        bg_window.set_zorder(z_order)?;
                        z_order += 1;
                        bar_window.set_zorder(z_order)?;
                        z_order += 1;
                        hg_window.set_zorder(z_order)?;

                        // Make all windows visible. Since we waiting for the
                        // post event for all of these, we can be assured that
                        // all of them will appear on the screen. As usual, all
                        // the requests that were made so far have just been
                        // batched in a command buffer somewhere in libscreen.
                        // To ensure those are flushed out and applied, we must
                        // call screen_flush_context. We will pass in the
                        // SCREEN_WAIT_IDLE flag to make sure that we spend at
                        // least one vsync, or refresh period, with the bar at
                        // 0,0 before moving it by 1 to the right.
                        bg_window.set_visible(true)?;
                        bar_window.set_visible(true)?;
                        hg_window.set_visible(true)?;
                        screen_ctx.flush(true)?;
                    }
                }
                qnxscreen::events::EventType::Property(property) => {
                    println!("Got property {:?}", property.get_name());
                }
                e => {
                    println!("Got other event {:?}", e);
                }
            }
        }

        if seen_bar && seen_hg {
            bar_x_pos += 1;
            if bar_x_pos > (dimensions.1 - BAR_WIDTH) {
                bar_x_pos = 0;
            }
            bar_window.set_position(bar_x_pos, 0)?;
            screen_ctx.flush(true)?;
        }
    }
}

/// Creates the background window in this sample. This window is essentially the
/// yellow background. Since it never changes, it is only necessary to fill a
/// single buffer and post once. A random group name is generated. All child
/// windows wishing to be joined to this application window must use this group
/// name. `dimensions` contains the dimensions of the window. The window can be
/// created with any buffer size. The windowing system will simply scale the
/// contents if the buffers are larger or smaller than how the window appears on
/// the screen. However, it is preferable to make the buffer size match the
/// on-screen dimensions of the window. Finally, `screen_ctx` should contain the
/// handle to the screen context to be used to create the application window.
fn create_bg_window(
    screen_ctx: &qnxscreen::Context,
    dimensions: (i32, i32),
) -> Result<qnxscreen::Window, qnxscreen::Error> {
    let screen_win = screen_ctx.create_window()?;
    screen_win.create_group(None)?;
    let group_name = screen_win.get_group_name()?;
    println!("Created window group {:?}", group_name);
    // Next, we set the visibility of the window to false. We don't want the
    // sample to build up on the screen. We want the first screen to contain
    // all windows, including the background, vertical bar, and the hour glass.
    // To achieve this, we will make all windows invisible until all the
    // windows have been created and are ready to be displayed
    screen_win.set_visible(true)?;
    // We will use a trick to fill the entire window with a solid color without
    // requiring a large buffer to back it up, and without requiring scaling to
    // be applied by the windowing system. Part of the trick is to set the window
    // color to the desired yellow.
    screen_win.set_colour(0xFFFFFF00)?;
    // The second part of the trick is to create a 1x1 buffer. The windowing
    // system currently doesn't support visible windows without at least one
    // buffer, so we will create the smallest buffer possible. The format and
    // usage don't matter since we will never use that buffer. We also set the
    // source viewport size to match the on-screen dimensions of the window.
    // This will avoid scaling.
    screen_win.set_buffer_size((1, 1))?;
    screen_win.set_source_size((dimensions.0, dimensions.1))?;
    screen_win.set_source_position((-dimensions.0, -dimensions.1))?;

    // Until the windowing system allows windows with 0 buffers to be visible,
    // we still need to create and post our 1x1 buffer in order to make the
    // application window visible when we're ready to switch its visibility.
    screen_win.create_buffers(1)?;
    let buffers = screen_win.get_buffers()?;
    let rect = qnxscreen::Rect::new((0, 0), 1, 1);
    screen_win.post(&buffers[0], &[rect], qnxscreen::PostFlags::empty())?;

    Ok(screen_win)
}

/// This function will be used to create the child window that will be used for
/// the vertical blue bar. It also never changes, it is only necessary to fill a
/// single buffer and post it once. The group argument contains the name of the
/// group that was created for the application window and that the child window
/// should join. id is a string that will be used to set the id string property,
/// which will be used by the sample to identify this child window as the bar
/// window. Dims contains the on-screen dimensions of the window.
fn create_bar_window<'ctx>(
    screen_ctx: &'ctx qnxscreen::Context,
    group: &str,
    id: &str,
) -> Result<qnxscreen::Window<'ctx>, qnxscreen::Error> {
    // We create a child window and make it join the group created by our
    // background window. Since there will be two child windows, one for the
    // vertical bar and one for the hour glass, we will set the id string of
    // the window so that the event loop in our sample can distinguish them.
    let screen_win = screen_ctx.create_child_window()?;
    screen_win.join_group(group)?;
    screen_win.set_id(id)?;
    // We set the visibility to false so that all windows can be turned on
    // at once. Setting the visibility is a responsibility of the parent. When
    // writing applications that use child windows, parents must set the
    // visibility of each child window to true when appropriate. The default
    // visibility for child windows is false, and processes creating child
    // windows should not change the visibility themselves.
    screen_win.set_visible(false)?;
    // Since the vertical bar is solid blue, we will use the same trick we
    // used for the background window. We start by setting the window color
    // to blue.
    screen_win.set_colour(0xff0000ff)?;
    // Next, we set the buffer size to 1x1.
    screen_win.set_buffer_size((1, 1))?;
    // Finally we position the source rectangle completely outside the buffer.
    screen_win.set_source_position((-1, -1))?;
    //  We still need to post the buffer before the window can become visible.
    //  Since we didn't draw anything in the buffer we are posting garbage.
    //  However, we've ensured that the contents of the buffer will not be used
    //  by the windowing system by positioning the source viewport outside the
    //  buffer, so it doesn't matter.
    screen_win.create_buffers(1)?;
    let buffers = screen_win.get_buffers()?;
    let rect = qnxscreen::Rect::new((0, 0), 1, 1);
    screen_win.post(&buffers[0], &[rect], qnxscreen::PostFlags::empty())?;

    Ok(screen_win)
}

/// This function creates the child window that will be used for the hour glass.
/// The window is created much in the same way as the other window. The main
/// difference is that the hour glass window is not a solid colored rectangle.
/// Therefore, we will need to draw the shape into its buffer before posting
/// once. The group argument contains the name of the group that was created for
/// the application window and that the child window should join. id is a string
/// that will be used to set the id string property of the window, which will be
/// used in the event loop to identify this child window as the hour glass
/// window. Dims contains the on-screen dimensions of the window.
fn create_hg_window<'ctx>(
    screen_ctx: &'ctx qnxscreen::Context,
    group: &str,
    id: &str,
) -> Result<qnxscreen::Window<'ctx>, qnxscreen::Error> {
    const HG_SIZE: i32 = 100;

    // Again, create a child window and make it join the group created by our
    // background window. Set the id string of the window so that the event
    // loop in our sample can distinguish the hour glass from the bar.
    let screen_win = screen_ctx.create_child_window()?;
    screen_win.join_group(group)?;
    screen_win.set_id(id)?;
    // The hour glass in this child window will never change. We can use the
    // static window property to tell the windowing system that the contents of
    // the buffer isn't going to change and we don't expect to be posting. This
    // allows the windowing system to do anything it can to optimize the work
    // required to put this window on the screen.
    screen_win.set_static(true)?;
    // Set the visibility to false so that all windows can be turned on at once.
    screen_win.set_visible(false)?;
    // The other windows were all created with the default pixel format. Since
    // the buffer contents are not going to be used for those windows, it
    // doesn't really matter what format is used. On the other hand, the hour
    // glass shape will have transparency with it, so we need a pixel format
    // with an alpha channel. We will choose RGBA8888.
    screen_win.set_screen_format(qnxscreen::FormatType::RGBA8888)?;
    // We will draw to the window buffer using a pointer. In order to get this
    // pointer, we must specify the usage to write.
    screen_win.set_usage(qnxscreen::UsageType::WRITE)?;
    // By default, RGBA8888 formats will have the transparency mode set to
    // source over. The windowing system assumes that if an application chooses
    // rgba over rgbx, it's because it wants to do some blending. However, it is
    // good practice to set the transparency mode.
    screen_win.set_transparency_mode(qnxscreen::TransparencyMode::SourceOver)?;

    // The hour glass shape is 100x100. We simply have to set the buffer size
    // to match those dimensions. The source rectangle will default to 100x100
    // once we set the buffer size, so there is no need to set it. The on-screen
    // dimensions of the child window will also default to 100x100. Parents are
    // responsible for setting the position and size of child windows. We must
    // not set those properties here, and let the event loop do that once all
    // windows are ready to be made visible.
    screen_win.set_buffer_size((HG_SIZE, HG_SIZE))?;

    // Next we allocate memory for our window buffer by calling
    // screen_create_window_buffers. Once the buffer is created we can get a
    // handle to this buffer by querying the windows's render buffers property.
    screen_win.create_buffers(1)?;
    let buffers = screen_win.get_buffers()?;

    // We need a pointer to this buffer to fill in the hour glass shape. We can
    // get it by querying the pointer property of the buffer handle we just got.
    // If you forgot to add read or write to the usage, this will return NULL.
    let mut ptr = buffers[0].get_pointer()?;

    // Stride is the number of bytes between pixels on different rows. That is,
    // if a pixel at position (x,y) is at ptr, the pixel at location (x,y+1) will
    // be at ptr+stride. There is no guarantee that each line is 400 bytes in this
    // case. Drivers often have constraints that will require the stride to be
    // larger than width * bytes per pixel.
    let stride = buffers[0].get_stride()?;

    // Once we have a pointer we can draw what we want in the buffer. In our
    // case, the hour glass shape is simple enough that we can calculate it.
    // We'll simply adjust the alpha channel to be transparent or opaque based
    // on a test that determines if a pixel is inside or outside the hour glass.
    for y in 0..100 {
        // i
        for x in 0..100 {
            // j
            unsafe {
                ptr.offset(x * 4).write_volatile(0xa0);
                ptr.offset((x * 4) + 1).write_volatile(0xa0);
                ptr.offset((x * 4) + 2).write_volatile(0xa0);
                ptr.offset((x * 4) + 3).write_volatile({
                    let bound = (HG_SIZE as isize) - y;
                    if (x >= y && x <= bound) || (x >= bound && x <= y) {
                        0xFF
                    } else {
                        0x00
                    }
                });
            }
        }
        unsafe {
            ptr = ptr.add(stride);
        }
    }

    // Finally, we post the buffer. This will allow the hour glass child window
    // to become visible when the event loop decides to make it visible. It is
    // customary for the first post to have a single dirty rect that covers the
    // entire buffer.
    let rect = qnxscreen::Rect::new((0, 0), HG_SIZE, HG_SIZE);
    screen_win.post(&buffers[0], &[rect], qnxscreen::PostFlags::empty())?;

    Ok(screen_win)
}
