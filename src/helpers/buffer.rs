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
