pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}
pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}
pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;
pub type DynamicArrayBuffer = DynamicBuffer<BufferTypeArray>;
pub type DynamicElementArrayBuffer = DynamicBuffer<BufferTypeElementArray>;

pub struct Buffer<B> where B: BufferType {
    vbo: gl::types::GLuint,
    _marker: ::std::marker::PhantomData<B>,
}

impl<B> Buffer<B> where B: BufferType {
    pub fn new() -> Buffer<B> {
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Buffer {
            vbo,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl<B> Drop for Buffer<B> where B: BufferType {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

//let default_max_buffer_size = (1000 * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr;
pub struct DynamicBuffer<B> where B: BufferType {
    vbo: gl::types::GLuint,
    max_buffer_size: gl::types::GLsizeiptr,
    pub buffer_offset: isize,
    _marker: ::std::marker::PhantomData<B>,
}

impl<B> DynamicBuffer<B> where B: BufferType {
    pub fn new(max_buffer_size: gl::types::GLsizeiptr) -> DynamicBuffer<B> {
        let mut vbo: gl::types::GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        DynamicBuffer {
            vbo,
            max_buffer_size,
            buffer_offset: 0,
            _marker: ::std::marker::PhantomData,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    pub fn set_buffer_data(&self) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                self.max_buffer_size,
                // TODO: check here first if there are issues, this seemed to do weird stuff before
                ::std::ptr::null(), // nullptr, null, //data.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    pub fn upload_draw_data<T>(&self, data: &[T]/*, offset: isize*/) {
        // need to cast isize to ? offset type = gl::types::GLintptr
        unsafe {
            gl::BufferSubData(
                B::BUFFER_TYPE,
                self.buffer_offset, // start at 0 go up by size of data, need to keep track of this?
                (data.len() * ::std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // data_size_in_bytes
                data.as_ptr() as *const gl::types::GLvoid,
            );
        }
    }

    pub fn set_buffer_offset(&mut self, offset: isize) {
        self.buffer_offset = offset;
    }

    pub fn reset_buffer_offset(&mut self) {
        self.buffer_offset = 0;
    }
}

impl<B> Drop for DynamicBuffer<B> where B: BufferType {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct VertexArray {
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        let mut vao: gl::types::GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }

        VertexArray { vao }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    fbo: gl::types::GLuint,
    texture: FrameBufferTexture,
    _marker: ::std::marker::PhantomData<B>,
}

impl FrameBuffer {
    pub fn new(screen_width: u32, screen_height: u32) -> Result<FrameBuffer, Error> {
        let mut fbo: gl::types::GLuint =  0;
        let texture = FrameBufferTexture::new(screen_width, screen_height);

        unsafe {
            gl::GenFramebuffers(1, &fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);
            // TODO: might need this later? for depth-testing/3D
            // but also probably want it to be a RenderBuffer or FrameBuffer<RenderBuffer>
            //gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo_depth);

            if gl::CheckFramebufferStatus(gl::FAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err("Failed to create FrameBuffer");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0)
        }

        Ok(FrameBuffer {
            fbo,
            texture,
            _marker: ::std::marker::PhantomData,
        })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &mut self.fbo);
        }
    }
}

#[derive(Debug)]
struct FrameBufferTexture {
    texture_handle: gl::types::GLuint,

}

impl FrameBufferTexture {
    pub fn new(screen_width: u32, screen_height: u32) -> FrameBufferTexture {
        let texture_handle = create_fb_texture(screen_width, screen_height);

        FrameBufferTexture {
            texture_handle,
        }
    }

    pub fn get_texture_handle(&self) -> gl::types::GLuint {
        self.texture_handle
    }

    pub fn set_size(&self, width: u32, height: u32) {
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null() //NULL
            );
        }
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn bind_to_unit(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot as gl::types::GLuint);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

// TODO: this is very much the same as in Texture minus image and a couple of different options
// most likely will move to use Texture, but be able to create a Texture::new without an image
// and also taking in TextureSettings
fn create_fb_texture(screen_width: u32, screen_height: u32) -> gl::types::GLuint {
    let mut texture_handle: gl::types::GLuint = 0;

    gl::ActiveTexture(gl::TEXTURE0);
    gl::GenTextures(1, &texture_handle);
    gl::BindTexture(gl::TEXTURE_2D, texture_handle);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA,
        screen_width as i32,
        screen_height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        std::ptr::null() // NULL // 0? null ptr? this is never clear around here
    );

    gl::BindTexture(GL_TEXTURE_2D, 0);

    texture_handle
}

impl Drop for FrameBufferTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture_handle);
        }
    }
}

