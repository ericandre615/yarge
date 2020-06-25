// TODO: putting this here as it may prove to be a useful feature for taking screenshots
// or just debugging dynamically created textures/render targets/frame buffers
pub fn snapshot_texture(texture: &FontTexture, file_name: &str) {
    let size = texture.width * texture.height * 4;
    texture.bind();

    let mut img_buf: Vec<u8> = Vec::with_capacity(size as usize);// as *mut std::ffi::c_void;
    unsafe { img_buf.set_len(size as usize);}
    let mut img_ptr = img_buf.as_mut_ptr();
    let mut img_container = image::DynamicImage::new_rgba8(texture.width, texture.height).to_rgba();

    unsafe {
        gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
        gl::GetTexImage(gl::TEXTURE_2D, 0, gl::RGBA, gl::UNSIGNED_BYTE, img_ptr as *mut std::ffi::c_void);
    }
    texture.unbind();

    let mut x = 0;
    let mut y = 0;
    let len = img_buf.len() - 4;
    for pxc in (0..len).step_by(4) {
        x += 1;
        if x >= texture.width { x = 0; y += 1; }
        let px = pxc as u32;

        img_container.put_pixel(x, y, image::Rgba([
           img_buf[pxc + 0],
           img_buf[pxc + 1],
           img_buf[pxc + 2],
           255//img_buf[pxc + 3]
        ]));
    }

    img_container.save(file_name).unwrap();
}
