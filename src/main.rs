use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{pixels::Color, sys::KeyCode};
use std::ffi::CString;
use std::time::Duration;

mod view;
use view::game_view::GameView;

mod helper;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window_width = 800;
    let window_height = 600;
    let window_sdl = video_subsystem
        .window("Pollux", window_width, window_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let gl_context = window_sdl.gl_create_context().unwrap();

    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vertex_shader = view::shader::Shader::from_vert_source(
        &CString::new(include_str!("model/triangle.vrt")).unwrap(),
    )
    .unwrap();

    let fragment_shader = view::shader::Shader::from_frag_source(
        &CString::new(include_str!("model/triangle.frag")).unwrap(),
    )
    .unwrap();

    let shader_program =
        view::program::Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

    shader_program.set_used();

    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vertex_buffer_object: gl::types::GLuint = 0;

    unsafe { gl::GenBuffers(1, &mut vertex_buffer_object) };

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };

    let mut vertex_array_object: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);
    };

    unsafe {
        gl::BindVertexArray(vertex_array_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        //canvas.present();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vertex_array_object);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window_sdl.gl_swap_window();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
