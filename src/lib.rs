// #[link(name = "EGL")]
// #[link(name = "GLESv3")]
// extern {}

extern crate libc;

#[macro_use]
extern crate log;

// extern crate egl;
extern crate servo;

use servo::BrowserId;
use servo::Servo;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::compositing::windowing::{WindowEvent, WindowMethods};
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D};
use servo::gl;
use servo::ipc_channel::ipc;
use servo::msg::constellation_msg::{Key, KeyModifiers};use std::rc::Rc;
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::LoadData;
use servo::servo_config::opts;
use servo::servo_config::resource_files::{set_resources_path, resources_dir_path};
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::DevicePixel;
use servo::style_traits::cursor::Cursor;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

mod egl {
    pub type khronos_utime_nanoseconds_t = super::khronos_utime_nanoseconds_t;
    pub type khronos_uint64_t = super::khronos_uint64_t;
    pub type khronos_ssize_t = super::khronos_ssize_t;
    pub type EGLNativeDisplayType = super::EGLNativeDisplayType;
    pub type EGLNativePixmapType = super::EGLNativePixmapType;
    pub type EGLNativeWindowType = super::EGLNativeWindowType;
    pub type EGLint = super::EGLint;
    pub type NativeDisplayType = super::EGLNativeDisplayType;
    pub type NativePixmapType = super::EGLNativePixmapType;
    pub type NativeWindowType = super::EGLNativeWindowType;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

}
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_uint64_t = libc::uint64_t;
pub type khronos_ssize_t = libc::c_long;
pub type EGLint = libc::int32_t;
pub type EGLNativeDisplayType = *const libc::c_void;
pub type EGLNativePixmapType = *const libc::c_void;     // FIXME: egl_native_pixmap_t instead

#[cfg(target_os = "windows")]
pub type EGLNativeWindowType = winapi::HWND;
#[cfg(target_os = "linux")]
pub type EGLNativeWindowType = *const libc::c_void;
#[cfg(target_os = "android")]
pub type EGLNativeWindowType = *const libc::c_void;
#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub type EGLNativeWindowType = *const libc::c_void;

thread_local! {
    static SERVO: RefCell<Option<(Servo<Callbacks>,BrowserId)>> = RefCell::new(None);
}

#[no_mangle]
pub extern "C" fn servo_version() -> *const c_char {
    let servo_version = servo::config::servo_version();
    let text = CString::new(format!("{}", servo_version)).unwrap();
    let ptr = text.as_ptr();
    std::mem::forget(text);
    ptr
}

#[no_mangle]
pub extern "C" fn init(wakeup: extern fn(), log_external: extern fn(*const c_char), width: u32, height: u32) {

    let _ = Logger::init(log_external);

    set_resources_path(Some("/sdcard/servo/resources/".to_owned()));

    let opts = opts::default_opts();
    opts::set_defaults(opts);

    let e = egl::Egl;
    let gl = unsafe {
        gl::GlesFns::load_with(|addr| {
            let addr = CString::new(addr.as_bytes()).unwrap();
            let addr = addr.as_ptr();
            e.GetProcAddress(addr) as *const c_void
        })
    };

    gl.clear_color(0.0, 1.0, 0.0, 1.0);
    gl.clear(gl::COLOR_BUFFER_BIT);
    gl.finish();

    let callbacks = Rc::new(Callbacks {
        waker: Box::new(SimpleEventLoopWaker(wakeup)),
        gl: gl.clone(),
        size: (width, height),
    });

    let mut servo = servo::Servo::new(callbacks.clone());

    let url = ServoUrl::parse("https://servo.org").unwrap();
    let (sender, receiver) = ipc::channel().unwrap();
    servo.handle_events(vec![WindowEvent::NewBrowser(url, sender)]);
    let browser_id = receiver.recv().unwrap();
    servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    SERVO.with(|s| {
        *s.borrow_mut() = Some((servo, browser_id));
    });
}

#[no_mangle]
pub extern "C" fn on_event_loop_awaken_by_servo() {
    SERVO.with(|s| {
        s.borrow_mut().as_mut().map(|&mut (ref mut s, _)| s.handle_events(vec![]));
    });
}

#[no_mangle]
pub extern "C" fn load_url(url: *const c_char) {
    SERVO.with(|s| {
        let url = unsafe { CStr::from_ptr(url) };
        if let Ok(url) = url.to_str() {
            if let Ok(url) = ServoUrl::parse(url) {
                s.borrow_mut().as_mut().map(|&mut (ref mut servo, id)| {
                    servo.handle_events(vec![WindowEvent::LoadUrl(id,url)]);
                });
            }
        }
    });
}

pub struct SimpleEventLoopWaker(extern fn());

impl EventLoopWaker for SimpleEventLoopWaker {
    fn clone(&self) -> Box<EventLoopWaker + Send> {
        Box::new(SimpleEventLoopWaker(self.0))
    }
    fn wake(&self) {
        (self.0)();
    }
}

struct Callbacks {
    waker: Box<EventLoopWaker>,
    gl: Rc<gl::Gl>,
    // flush_cb: extern fn(),
    size: (u32, u32),
}

impl WindowMethods for Callbacks {
    fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
        true
    }

    fn present(&self) {
        // let e = egl::Egl;
        // unsafe {
        //     let display = e.GetCurrentDisplay();
        //     let surface = e.GetCurrentSurface(egl::DRAW as EGLint);
        //     e.SwapBuffers(display, surface);
        // }
    }

    fn supports_clipboard(&self) -> bool {
        false
    }

    fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
        self.waker.clone()
    }

    fn gl(&self) -> Rc<gl::Gl> {
        self.gl.clone()
    }

    fn hidpi_factor(&self) -> ScaleFactor<f32, DeviceIndependentPixel, DevicePixel> {
        let factor = 2.0;
        ScaleFactor::new(factor)
    }

    fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
        let scale_factor = 2;
        TypedSize2D::new(scale_factor * self.size.0, scale_factor * self.size.1)
    }

    fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
        TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
    }

    fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
        TypedSize2D::new(self.size.0 as f32, self.size.1 as f32)
    }

    fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
        (Size2D::new(self.size.0, self.size.1), Point2D::new(0, 0))
    }

    fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) { chan.send(true).ok(); }
    fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {}
    fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {}
    fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {}
    fn set_page_title(&self, _id: BrowserId, _title: Option<String>) {}
    fn status(&self, _id: BrowserId, _status: Option<String>) {}
    fn load_start(&self, _id: BrowserId) {}
    fn load_end(&self, _id: BrowserId) {}
    fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {}
    fn head_parsed(&self, _id: BrowserId) {}
    fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {}
    fn set_cursor(&self, _cursor: Cursor) { }
    fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {}
    fn handle_key(&self, _id: Option<BrowserId>, _ch: Option<char>, _key: Key, _mods: KeyModifiers) { }
}



use log::{set_logger, Log, LogRecord, LogMetadata, LogLevelFilter, LogLevel};

struct Logger(extern fn(*const c_char));

impl Logger {
    pub fn init(callback: extern fn(*const c_char)) {
        set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(Logger(callback))
        });
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let msg = format!("LOG: [{}] [{}] [{}]", record.level(), record.args(), record.target());
            let text = CString::new(msg.to_owned()).unwrap();
            let ptr = text.as_ptr();
            self.0(ptr);
        }
    }
}
