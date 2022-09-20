use std::{
    fs::File,
    io::{stdout, BufWriter, Read, Write},
};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("fficpp/include/cvfn.h");
        fn resize_raw(buf: &[u8], w: usize, h: usize) -> Vec<u8>;
    }
}

pub fn cc(w: usize, h: usize) {
    let mut f =
        File::open("/home/lapis/project/ffi/ffiapp/pic/su.jpg").expect("could not open fle");
    let mut buf = vec![];
    f.read_to_end(&mut buf).expect("could not load buffer");
    let resized_buf = ffi::resize_raw(&buf, w, h);

    let mut file = File::create("/home/lapis/project/ffi/ffiapp/pic/resize.jpg").unwrap();
    file.write_all(&resized_buf);
    // let mut writer = BufWriter::new(stdout());
    // writer.write_all(&resized_buf).unwrap();
}
