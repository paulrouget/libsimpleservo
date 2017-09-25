extern crate servo;
// extern crate core_foundation;

use std::ffi::CString;
use servo::gl;

use servo::BrowserId;
use servo::Servo;
use servo::servo_config::resource_files::set_resources_path;
use servo::servo_config::opts;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::compositing::windowing::{WindowEvent, WindowMethods};
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D};
use servo::ipc_channel::ipc;
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::LoadData;
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::DevicePixel;
use servo::style_traits::cursor::Cursor;
use servo::msg::constellation_msg::{Key, KeyModifiers};use std::rc::Rc;

// use core_foundation::base::TCFType;
// use core_foundation::string::CFString;
// use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};

use std::os::raw::c_char;
use std::os::raw::c_void;
use std::str;

use std::cell::RefCell;

thread_local! {
    static SERVO: RefCell<Option<Servo<Callbacks>>> = RefCell::new(None);
}

#[no_mangle]
pub extern "C" fn servo_version() -> *const c_char {
    let version = CString::new(servo::config::servo_version()).unwrap();
    let ptr = version.as_ptr();
    std::mem::forget(version);
    ptr
}

#[no_mangle]
pub extern "C" fn init(flush_cb: extern fn(), wakeup: extern fn(), width: u32, height: u32) {

    // let gl = unsafe {
    //     gl::GlFns::load_with(|addr| {
    //         let symbol_name: CFString = str::FromStr::from_str(addr).unwrap();
    //         let framework_name: CFString = str::FromStr::from_str("com.apple.opengl").unwrap();
    //         let framework = CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef());
    //         let symbol = CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef());
    //         symbol as *const c_void
    //     })
    // };

    // gl.clear_color(0.0, 1.0, 0.0, 1.0);
    // gl.clear(gl::COLOR_BUFFER_BIT);
    // gl.finish();

    // // Maybe we run from an app bundle
    // let p = std::env::current_exe().unwrap();
    // let p = p.parent().unwrap();
    // let p = p.parent().unwrap().join("Resources");
    // if !p.exists() {
    //     panic!("Can't file resources directory: {}", p.to_str().unwrap());
    // }
    // let path = p.to_str().unwrap().to_string();
    // set_resources_path(Some(path));

    // let opts = opts::default_opts();
    // opts::set_defaults(opts);

    // let callbacks = Rc::new(Callbacks {
    //     waker: Box::new(SimpleEventLoopWaker(wakeup)),
    //     gl: gl.clone(),
    //     flush_cb,
    //     size: (width, height),
    // });

    // let mut servo = servo::Servo::new(callbacks.clone());

    // let url = ServoUrl::parse("https://www.xamarin.com/forms").unwrap();
    // let (sender, receiver) = ipc::channel().unwrap();
    // servo.handle_events(vec![WindowEvent::NewBrowser(url, sender)]);
    // let browser_id = receiver.recv().unwrap();
    // servo.handle_events(vec![WindowEvent::SelectBrowser(browser_id)]);

    // SERVO.with(|s| {
    //     *s.borrow_mut() = Some(servo);
    // });
}

#[no_mangle]
pub extern "C" fn ping() {
    SERVO.with(|s| {
        s.borrow_mut().as_mut().map(|servo| servo.handle_events(vec![]));
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
    flush_cb: extern fn(),
    size: (u32, u32),
}

impl WindowMethods for Callbacks {
    fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
        true
    }

    fn present(&self) {
        (self.flush_cb)();
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
