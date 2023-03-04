use std::{ffi::CString, time::Instant};

mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Simulator", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    unsafe {
        gl::Viewport(0, 0, 800, 800);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();
    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        0.0, 0.0, 0.0, // vertex
        1.0, 0.0, 0.0, // color
        -1.0, -1.0, // UV
        0.0, 1.0, 0.0, // vertex
        1.0, 0.0, 0.0, // color
        -1.0, 1.0, // UV
        1.0, 0.0, 0.0, // vertex
        1.0, 0.0, 0.0, // color
        1.0, -1.0, // UV
        // ----
        1.0, 1.0, 0.0, // vertex
        1.0, 0.0, 0.0, // color
        -1.0, -1.0, // UV
        1.0, 0.0, 0.0, // vertex
        1.0, 0.0, 0.0, // color
        -1.0, 1.0, // UV
        0.0, 1.0, 0.0, // vertex
        1.0, 0.0, 0.0, // color
        1.0, -1.0, // UV
    ];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as _,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // layout (location = 0)
        gl::VertexAttribPointer(
            // index of generic vertex attribute
            0,
            // number of components per generic vertex attr
            3,
            // data type
            gl::FLOAT,
            // normalized?
            gl::FALSE,
            // stride (between consecutive attrs)
            (8 * std::mem::size_of::<f32>()) as _,
            // offset of the first component
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1); // layout (location = 1)
        gl::VertexAttribPointer(
            // index of generic vertex attribute
            1,
            // number of components per generic vertex attr
            3,
            // data type
            gl::FLOAT,
            // normalized?
            gl::FALSE,
            // stride (between consecutive attrs)
            (8 * std::mem::size_of::<f32>()) as _,
            // offset of the first component
            (3 * std::mem::size_of::<f32>()) as _,
        );

        gl::EnableVertexAttribArray(2); // layout (location = 2)
        gl::VertexAttribPointer(
            // index of generic vertex attribute
            2,
            // number of components per generic vertex attr
            2,
            // data type
            gl::FLOAT,
            // normalized?
            gl::FALSE,
            // stride (between consecutive attrs)
            (8 * std::mem::size_of::<f32>()) as _,
            // offset of the first component
            (6 * std::mem::size_of::<f32>()) as _,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::BLEND);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    loop {
        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return,
                _ => {}
            }
        }

        shader_program.set_used();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                // starting index in enabled arrays
                0,
                // number of indices to be rendered
                6,
            )
        }

        window.gl_swap_window();

        // let duration = Instant::now() - start;
        let duration = start.elapsed();
        let fps = 1. / duration.as_secs_f64();
        println!("fps = {fps}");
    }
}
