use core::ffi::c_void;
use roc_std::RocStr;
use std::fmt::Debug;
use std::mem::MaybeUninit;

mod ui;

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(msg: *mut RocStr, tag_id: u32) {
    match tag_id {
        0 => {
            eprintln!("Roc standard library hit a panic: {}", &*msg);
        }
        1 => {
            eprintln!("Application hit a panic: {}", &*msg);
        }
        _ => unreachable!(),
    }
    std::process::exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dbg(loc: *mut RocStr, msg: *mut RocStr, src: *mut RocStr) {
    eprintln!("[{}] {} = {}", &*loc, &*src, &*msg);
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_getppid() -> libc::pid_t {
    libc::getppid()
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_mmap(
    addr: *mut libc::c_void,
    len: libc::size_t,
    prot: libc::c_int,
    flags: libc::c_int,
    fd: libc::c_int,
    offset: libc::off_t,
) -> *mut libc::c_void {
    libc::mmap(addr, len, prot, flags, fd, offset)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_shm_open(
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::mode_t,
) -> libc::c_int {
    libc::shm_open(name, oflag, mode as libc::c_uint)
}

type BoxedModel = roc_std::RocBox<std::ffi::c_void>;

#[derive(Debug)]
#[repr(C)]
pub struct ProgramForHost {
    init_closure_data: Vec<u8>,
    render_closure_data: Vec<u8>,
    update_closure_data: Vec<u8>,
    model: MaybeUninit<BoxedModel>,
}

pub fn program_for_host() -> ProgramForHost {
    extern "C" {
        fn roc__mainForHost_1_exposed_generic(_: *mut u8);
        fn roc__mainForHost_1_exposed_size() -> isize;
        fn roc__mainForHost_0_size() -> isize;
        fn roc__mainForHost_1_size() -> isize;
        fn roc__mainForHost_2_size() -> isize;
    }
    let size = unsafe { roc__mainForHost_1_exposed_size() } as usize;
    let mut captures = Vec::with_capacity(size);
    captures.resize(size, 0);

    unsafe {
        roc__mainForHost_1_exposed_generic(captures.as_mut_ptr());
    }

    let init_size = unsafe { roc__mainForHost_0_size() } as usize;
    let render_size = unsafe { roc__mainForHost_1_size() } as usize;
    let update_size = unsafe { roc__mainForHost_2_size() } as usize;

    let mut ret = ProgramForHost {
        init_closure_data: Vec::with_capacity(init_size),
        render_closure_data: Vec::with_capacity(render_size),
        update_closure_data: Vec::with_capacity(update_size),
        model: MaybeUninit::uninit(),
    };

    let mut data_slice = captures.as_slice();
    ret.init_closure_data.extend(&data_slice[..init_size]);
    data_slice = &data_slice[init_size..];
    ret.render_closure_data.extend(&data_slice[..render_size]);
    data_slice = &data_slice[render_size..];
    ret.update_closure_data.extend(&data_slice[..update_size]);

    ret
}

impl ProgramForHost {
    pub fn init(&mut self, arg0: roc_app::Bounds) {
        extern "C" {
            fn roc__mainForHost_0_caller(
                arg0: *const roc_app::Bounds,
                closure_data: *mut u8,
                output: *mut BoxedModel,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_0_caller(
                &arg0,
                self.init_closure_data.as_mut_ptr(),
                output.as_mut_ptr(),
            );

            self.model = output;
        }
    }

    pub fn update(&mut self, arg1: roc_app::Event) {
        extern "C" {
            fn roc__mainForHost_2_caller(
                model: *const BoxedModel,
                arg1: *const roc_app::Event,
                closure_data: *mut u8,
                output: *mut BoxedModel,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_2_caller(
                self.model.as_mut_ptr(),
                &arg1,
                self.update_closure_data.as_mut_ptr(),
                output.as_mut_ptr(),
            );

            self.model = output;
        }
    }

    pub fn render(&mut self) -> roc_std::RocList<u8> {
        extern "C" {
            fn roc__mainForHost_1_caller(
                arg0: *const BoxedModel,
                closure_data: *mut u8,
                output: *mut RenderReturn,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_1_caller(
                self.model.as_mut_ptr(),
                self.render_closure_data.as_mut_ptr(),
                output.as_mut_ptr(),
            );

            let render_return = output.assume_init();

            self.model = MaybeUninit::new(render_return.model);

            render_return.root
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderReturn {
    pub model: BoxedModel,
    pub root: roc_std::RocList<u8>,
}
