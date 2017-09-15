extern crate libc;
extern crate servo;

use std::ffi::CString;
use servo::gl;
use servo::BrowserId;
use servo::Servo;
use servo::servo_config::resource_files::set_resources_path;
use servo::servo_config::opts;
use servo::compositing::compositor_thread::EventLoopWaker;
use servo::compositing::windowing::WindowMethods;
use servo::euclid::{Point2D, ScaleFactor, Size2D, TypedPoint2D, TypedRect, TypedSize2D};
use servo::ipc_channel::ipc;
use servo::net_traits::net_error_list::NetError;
use servo::script_traits::LoadData;
use servo::servo_geometry::DeviceIndependentPixel;
use servo::servo_url::ServoUrl;
use servo::style_traits::DevicePixel;
use servo::style_traits::cursor::Cursor;
use servo::msg::constellation_msg::{Key, KeyModifiers};use std::rc::Rc;


#[no_mangle]
pub extern "C" fn servo_version() -> *const i8 {
    println!("rust: servo_version() called");
    let version = CString::new(servo::config::servo_version()).unwrap();
    version.as_ptr()
}

// extern fn init() -> {
//     let path = std::env::current_dir().unwrap().join("resources");
//     if !path.exists() {
//         panic!("Can't file resources directory");
//     }
//     let path = path.to_str().unwrap().to_string();
//     set_resources_path(Some(path));

//     let mut opts = opts::default_opts();
//     opts::set_defaults(opts);

//     let callbacks = Rc::new(Callbacks {
//         waker: Box::new(SimpleEventLoopWaker),
//         gl: gl,
//     });

//     let mut servo = Box::new(servo::Servo::new(callbacks.clone()));
//     &mut *
// }

// pub fn ping() {
//     SERVO.lock().unwrap().borrow_mut().handle_events(vec![]);
// }

// pub struct SimpleEventLoopWaker;

// impl EventLoopWaker for SimpleEventLoopWaker {
//     fn clone(&self) -> Box<EventLoopWaker + Send> {
//         Box::new(SimpleEventLoopWaker)
//     }
//     fn wake(&self) {
//     }
// }

// struct Callbacks {
//     waker: Box<EventLoopWaker>,
//     gl: Rc<gl::Gl>,
// }

// impl WindowMethods for Callbacks {
//     fn prepare_for_composite(&self, _width: usize, _height: usize) -> bool {
//         true
//     }

//     fn present(&self) {
//         // FIXME
//     }

//     fn supports_clipboard(&self) -> bool {
//         false
//     }

//     fn create_event_loop_waker(&self) -> Box<EventLoopWaker> {
//         self.waker.clone()
//     }

//     fn gl(&self) -> Rc<gl::Gl> {
//         self.gl.clone()
//     }

//     fn hidpi_factor(&self) -> ScaleFactor<f32, DeviceIndependentPixel, DevicePixel> {
//         let factor = 2.0;
//         ScaleFactor::new(factor)
//     }

//     fn framebuffer_size(&self) -> TypedSize2D<u32, DevicePixel> {
//         let (width, height) = (200.0, 200.0);
//         let scale_factor = 2.0; 
//         TypedSize2D::new(scale_factor * width, scale_factor * height)
//     }

//     fn window_rect(&self) -> TypedRect<u32, DevicePixel> {
//         TypedRect::new(TypedPoint2D::new(0, 0), self.framebuffer_size())
//     }

//     fn size(&self) -> TypedSize2D<f32, DeviceIndependentPixel> {
//         let (width, height) = (200.0, 200.0);
//         TypedSize2D::new(width as f32, height as f32)
//     }

//     fn client_window(&self, _id: BrowserId) -> (Size2D<u32>, Point2D<i32>) {
//         let (width, height) = (200.0, 200.0);
//         let (x, y) = (0.0, 0.0);
//         (Size2D::new(width, height), Point2D::new(x as i32, y as i32))
//     }

//     fn allow_navigation(&self, _id: BrowserId, _url: ServoUrl, chan: ipc::IpcSender<bool>) { chan.send(true).ok(); }
//     fn set_inner_size(&self, _id: BrowserId, _size: Size2D<u32>) {}
//     fn set_position(&self, _id: BrowserId, _point: Point2D<i32>) {}
//     fn set_fullscreen_state(&self, _id: BrowserId, _state: bool) {}
//     fn set_page_title(&self, _id: BrowserId, title: Option<String>) {}
//     fn status(&self, _id: BrowserId, _status: Option<String>) {}
//     fn load_start(&self, _id: BrowserId) {}
//     fn load_end(&self, _id: BrowserId) {}
//     fn load_error(&self, _id: BrowserId, _: NetError, _url: String) {}
//     fn head_parsed(&self, _id: BrowserId) {}
//     fn history_changed(&self, _id: BrowserId, _entries: Vec<LoadData>, _current: usize) {}
//     fn set_cursor(&self, _cursor: Cursor) { }
//     fn set_favicon(&self, _id: BrowserId, _url: ServoUrl) {}
//     fn handle_key(&self, _id: Option<BrowserId>, _ch: Option<char>, _key: Key, _mods: KeyModifiers) { }
// }
