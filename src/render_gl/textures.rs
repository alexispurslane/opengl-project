use std::marker::PhantomData;

pub struct TextureParameters {
    wrap_s: gl::types::GLint,
    wrap_t: gl::types::GLint,
    min_filter: gl::types::GLint,
    mag_filter: gl::types::GLint,
}

impl TextureParameters {
    pub fn default() -> Self {
        TextureParameters {
            wrap_s: gl::REPEAT as gl::types::GLint,
            wrap_t: gl::REPEAT as gl::types::GLint,
            min_filter: gl::LINEAR_MIPMAP_LINEAR as gl::types::GLint,
            mag_filter: gl::LINEAR as gl::types::GLint,
        }
    }
}

pub trait ColorDepth {
    fn get_gl_type() -> gl::types::GLenum;
}

type RGB8 = u8;
impl ColorDepth for RGB8 {
    fn get_gl_type() -> gl::types::GLenum {
        gl::UNSIGNED_BYTE
    }
}
type RGB16 = u16;
impl ColorDepth for RGB16 {
    fn get_gl_type() -> gl::types::GLenum {
        gl::UNSIGNED_SHORT
    }
}
type RGB32F = f32;
impl ColorDepth for RGB32F {
    fn get_gl_type() -> gl::types::GLenum {
        gl::FLOAT
    }
}

pub struct Texture<T: ColorDepth> {
    pub id: gl::types::GLuint,
    pub ty: gl::types::GLenum,
    pub parameters: TextureParameters,
    phantom: PhantomData<T>,
}

impl<T: ColorDepth> Texture<T> {
    pub fn new(ty: gl::types::GLenum, parameters: TextureParameters) -> Self {
        let mut texture: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }
        Self {
            id: texture,
            ty,
            parameters,
            phantom: PhantomData,
        }
    }

    pub fn new_with_bytes(
        ty: gl::types::GLenum,
        parameters: TextureParameters,
        bytes: &Vec<T>,
        width: u32,
        height: u32,
    ) -> Self {
        let tex = Self::new(ty, parameters);
        tex.bind();
        tex.load_texture_from_bytes(bytes, width, height);
        tex.unbind();
        tex
    }

    pub fn load_texture_from_bytes(&self, bytes: &Vec<T>, width: u32, height: u32) {
        unsafe {
            gl::TexParameteri(self.ty, gl::TEXTURE_WRAP_S, self.parameters.wrap_s);
            gl::TexParameteri(self.ty, gl::TEXTURE_WRAP_T, self.parameters.wrap_t);
            gl::TexParameteri(self.ty, gl::TEXTURE_MIN_FILTER, self.parameters.min_filter);
            gl::TexParameteri(self.ty, gl::TEXTURE_MAG_FILTER, self.parameters.mag_filter);
            gl::TexImage2D(
                self.ty,
                0,
                gl::RGB as gl::types::GLint,
                width as gl::types::GLsizei,
                height as gl::types::GLsizei,
                0,
                gl::RGB,
                T::get_gl_type(),
                bytes.as_ptr() as *const gl::types::GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn bind_to_texture_unit(&self, tex_unit: gl::types::GLenum) {
        unsafe {
            gl::ActiveTexture(tex_unit);
        }
        self.bind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.ty, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(self.ty, 0);
        }
    }
}

impl<T: ColorDepth> Drop for Texture<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
