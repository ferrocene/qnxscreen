//! A simple example to show how to set up OpenGL ES rendering
//!
//!
//! Copyright (c) Ferrous Systems, 2023
//!
//! Copyright (c) AdaCore, 2023

#![deny(warnings)]

use anyhow::Context;
use glow::HasContext;

fn main() -> Result<(), anyhow::Error> {
    println!("Rust EGL init demo.");
    println!("Copyright (c) Ferrous Systems, 2023");

    // Create a screen context
    let ctx = qnxscreen::Context::new(
        qnxscreen::ContextType::APPLICATION | qnxscreen::ContextType::INPUT_MANAGER,
    )?;

    let window = ctx.create_window()?;

    // Lock us to 60 fps on a 60 Hz screen
    println!("Set swap interval to 1");
    window.set_swap_interval(1)?;

    // We want permission to draw on the window
    println!("Set window usage to OpenGL_ES3");
    window.set_usage(qnxscreen::UsageType::OPENGL_ES3)?;

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

    unsafe {
        let (egl_display, egl_context, egl_surface) = egl_init(&window)?;

        println!("Making EGL window surface current");
        let result =
            qnx_egl_sys::egl::MakeCurrent(egl_display, egl_surface, egl_surface, egl_context);
        check_egl_error(result)?;

        let glow_context = glow::Context::from_loader_function_cstr(|name| {
            qnx_egl_sys::egl::GetProcAddress(name.as_ptr()) as _
        });

        // Triangle copied from glow example

        let vertex_array = glow_context
            .create_vertex_array()
            .expect("Cannot create vertex array");
        glow_context.bind_vertex_array(Some(vertex_array));

        let program = glow_context
            .create_program()
            .expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"#version 300 es
        const vec2 verts[3] = vec2[3](
            vec2(0.5f, 1.0f),
            vec2(0.0f, 0.0f),
            vec2(1.0f, 0.0f)
        );
        out vec2 vert;
        void main() {
            vert = verts[gl_VertexID];
            gl_Position = vec4(vert - 0.5, 0.0, 1.0);
        }"#,
            r#"#version 300 es
        precision mediump float;
        in vec2 vert;
        out vec4 color;
        void main() {
            color = vec4(vert, 0.5, 1.0);
        }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = glow_context
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            glow_context.shader_source(shader, shader_source);
            glow_context.compile_shader(shader);
            if !glow_context.get_shader_compile_status(shader) {
                panic!("{}", glow_context.get_shader_info_log(shader));
            }
            glow_context.attach_shader(program, shader);
            shaders.push(shader);
        }

        glow_context.link_program(program);
        if !glow_context.get_program_link_status(program) {
            panic!("{}", glow_context.get_program_info_log(program));
        }

        for shader in shaders {
            glow_context.detach_shader(program, shader);
            glow_context.delete_shader(shader);
        }

        // Run forever
        loop {
            glow_context.use_program(Some(program));
            glow_context.clear_color(0.1, 0.2, 0.3, 1.0);
            glow_context.clear(glow::COLOR_BUFFER_BIT);
            glow_context.draw_arrays(glow::TRIANGLES, 0, 3);

            let result = qnx_egl_sys::egl::SwapBuffers(egl_display, egl_surface);
            check_egl_error(result)?;
        }
    }
}

unsafe fn egl_init(
    window: &qnxscreen::Window,
) -> Result<
    (
        qnx_egl_sys::egl::types::EGLDisplay,
        qnx_egl_sys::egl::types::EGLContext,
        qnx_egl_sys::egl::types::EGLSurface,
    ),
    anyhow::Error,
> {
    let egl_display = qnx_egl_sys::egl::GetDisplay(qnx_egl_sys::egl::DEFAULT_DISPLAY);

    if egl_display == qnx_egl_sys::egl::NO_DISPLAY {
        map_egl_error().context("Could not get EGL default display")?;
    }

    let result =
        qnx_egl_sys::egl::Initialize(egl_display, core::ptr::null_mut(), core::ptr::null_mut());
    check_egl_error(result).context("Error initializing EGL")?;

    let config_attrs: [i32; 15] = [
        qnx_egl_sys::egl::RED_SIZE as _,
        8,
        qnx_egl_sys::egl::GREEN_SIZE as _,
        8,
        qnx_egl_sys::egl::BLUE_SIZE as _,
        8,
        qnx_egl_sys::egl::ALPHA_SIZE as _,
        8,
        qnx_egl_sys::egl::STENCIL_SIZE as _,
        8,
        qnx_egl_sys::egl::SURFACE_TYPE as _,
        qnx_egl_sys::egl::WINDOW_BIT as _,
        qnx_egl_sys::egl::RENDERABLE_TYPE as _,
        qnx_egl_sys::egl::OPENGL_ES3_BIT as _,
        qnx_egl_sys::egl::NONE as _,
    ];

    let mut egl_config: qnx_egl_sys::egl::types::EGLConfig = std::ptr::null();
    let mut egl_config_count = 0;

    let result = qnx_egl_sys::egl::ChooseConfig(
        egl_display,
        config_attrs.as_ptr(),
        (&mut egl_config) as _,
        1,
        &mut egl_config_count,
    );
    check_egl_error(result).context("Error Choosing EGL configuration")?;

    let context_attrs: [i32; 3] = [
        qnx_egl_sys::egl::CONTEXT_CLIENT_VERSION as _,
        2,
        qnx_egl_sys::egl::NONE as _,
    ];

    let egl_context = qnx_egl_sys::egl::CreateContext(
        egl_display,
        egl_config,
        qnx_egl_sys::egl::NO_CONTEXT,
        context_attrs.as_ptr(),
    );
    if egl_context == qnx_egl_sys::egl::NO_CONTEXT {
        map_egl_error().context("Could not create EGL context")?;
    }

    let sys_window = window.window();

    let surface_attrs: [i32; 3] = [
        qnx_egl_sys::egl::RENDER_BUFFER as _,
        qnx_egl_sys::egl::BACK_BUFFER as _,
        qnx_egl_sys::egl::NONE as _,
    ];

    let egl_surface = qnx_egl_sys::egl::CreateWindowSurface(
        egl_display,
        egl_config,
        sys_window,
        surface_attrs.as_ptr(),
    );

    if egl_surface == qnx_egl_sys::egl::NO_SURFACE {
        map_egl_error().context("Could not create EGL window surface")?;
    }

    Ok((egl_display, egl_context, egl_surface))
}

#[derive(Debug, thiserror::Error)]
enum EGLError {
    #[error("The last function succeeded without error.")]
    Success,
    #[error("EGL is not initialized, or could not be initialized, for the specified EGL display connection.")]
    NotInitialized,
    #[error("EGL cannot access a requested resource (for example a context is bound in another thread).")]
    BadAccess,
    #[error("EGL failed to allocate resources for the requested operation.")]
    BadAlloc,
    #[error("An unrecognized attribute or attribute value was passed in the attribute list.")]
    BadAttribute,
    #[error("An EGLContext argument does not name a valid EGL rendering context.")]
    BadContext,
    #[error("An EGLConfig argument does not name a valid EGL frame buffer configuration.")]
    BadConfig,
    #[error("The current surface of the calling thread is a window, pixel buffer or pixmap that is no longer valid.")]
    BadCurrentSurface,
    #[error("An EGLDisplay argument does not name a valid EGL display connection.")]
    BadDisplay,
    #[error("An EGLSurface argument does not name a valid surface (window, pixel buffer or pixmap) configured for GL rendering.")]
    BadSurface,
    #[error("Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).")]
    BadMatch,
    #[error("One or more argument values are invalid.")]
    BadParameter,
    #[error("A NativePixmapType argument does not refer to a valid native pixmap.")]
    BadNativePixmap,
    #[error("A NativeWindowType argument does not refer to a valid native window.")]
    BadNativeWindow,
    #[error("A power management event has occurred. The application must destroy all contexts and reinitialise OpenGL ES state and objects to continue rendering.")]
    ContextLost,
}

fn check_egl_error(result: qnx_egl_sys::egl::types::EGLBoolean) -> Result<(), EGLError> {
    if result == qnx_egl_sys::egl::TRUE {
        Ok(())
    } else {
        map_egl_error()
    }
}

fn map_egl_error() -> Result<(), EGLError> {
    let error_code = unsafe { qnx_egl_sys::egl::GetError() as u32 };
    Err(match error_code {
        qnx_egl_sys::egl::SUCCESS => EGLError::Success,
        qnx_egl_sys::egl::NOT_INITIALIZED => EGLError::NotInitialized,
        qnx_egl_sys::egl::BAD_ACCESS => EGLError::BadAccess,
        qnx_egl_sys::egl::BAD_ALLOC => EGLError::BadAlloc,
        qnx_egl_sys::egl::BAD_ATTRIBUTE => EGLError::BadAttribute,
        qnx_egl_sys::egl::BAD_CONTEXT => EGLError::BadContext,
        qnx_egl_sys::egl::BAD_CONFIG => EGLError::BadConfig,
        qnx_egl_sys::egl::BAD_CURRENT_SURFACE => EGLError::BadCurrentSurface,
        qnx_egl_sys::egl::BAD_DISPLAY => EGLError::BadDisplay,
        qnx_egl_sys::egl::BAD_SURFACE => EGLError::BadSurface,
        qnx_egl_sys::egl::BAD_MATCH => EGLError::BadMatch,
        qnx_egl_sys::egl::BAD_PARAMETER => EGLError::BadParameter,
        qnx_egl_sys::egl::BAD_NATIVE_PIXMAP => EGLError::BadNativePixmap,
        qnx_egl_sys::egl::BAD_NATIVE_WINDOW => EGLError::BadNativeWindow,
        qnx_egl_sys::egl::CONTEXT_LOST => EGLError::ContextLost,
        _ => panic!("eglGetError() returned unmapped error"),
    })
}
