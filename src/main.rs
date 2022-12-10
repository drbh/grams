use livesplit_hotkey::{Hook, KeyCode as HKeyCode, Modifiers};
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    process::Command,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use wry::application::window::Window;
use wry::{
    application::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

// Message struct that will be used to parse json
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    color: String,
    text: String,
}

// rust path to users Library/Application Support
const APP_DATA_DIR: &str = "/Library/Application Support/Grams";

fn main() -> wry::Result<()> {
    // make sure we have our APP_DATA_DIR to write to
    let home_dir = std::env::var("HOME").unwrap();
    std::fs::create_dir_all(format!("{}{}/", home_dir, APP_DATA_DIR))?;

    let user_agent_string = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36".to_string();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Grams")
        .with_inner_size(PhysicalSize {
            width: 1500,
            height: 1800,
        })
        .with_position(PhysicalPosition { x: 0, y: 0 })
        .with_content_protection(false)
        .with_transparent(false)
        .build(&event_loop)?;

    // a mutex atomic counter
    let counter = Arc::new(AtomicUsize::new(0));

    // get current time as a ISO 8601 string
    let app_start_time = chrono::Local::now().to_rfc3339();

    let handler = move |_window: &Window, req: String| match req.as_str() {
        "new-window" => {
            println!("new-window")
        }
        "close" => {
            println!("close")
        }
        _ if req.starts_with("change-title") => {
            let title = req.replace("change-title ", "");

            // parse json to get messages
            let messages: Vec<Message> = serde_json::from_str(&title).unwrap_or_default();

            // get value of counter
            let counter_value = counter.load(Ordering::SeqCst);

            if counter_value % 2 == 0 && messages.len() > 0 {
                // using the std library get the home directory
                let home_dir = std::env::var("HOME").unwrap();
                let user_path = format!(
                    "{}{}/{}-messages.txt",
                    home_dir, APP_DATA_DIR, app_start_time
                );

                // write mesaage to file
                let mut file = std::fs::File::create(user_path.as_str()).unwrap();

                // write each message as a line
                for message in messages.iter() {
                    file.write_all(message.text.as_bytes()).unwrap();
                    file.write_all(b"\r\n").unwrap();
                }
            }

            if messages.len() > counter_value {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }
        _ => {}
    };

    // read full.js as a string
    let full_js = include_str!("../full.js");

    let _webview = WebViewBuilder::new(window)?
        .with_user_agent(&user_agent_string)
        .with_devtools(true)
        .with_clipboard(true)
        .with_initialization_script(full_js)
        .with_ipc_handler(handler)
        .with_url("https://chat.openai.com")?
        .build()?;

    let hook = Hook::new().expect("couldn't create hook");
    hook.register(HKeyCode::Quote.with_modifiers(Modifiers::META), move || {
        Command::new("osascript")
            .arg("-e")
            .arg("tell application \"grams.app\"")
            .arg("-e")
            .arg("if not running then")
            .arg("-e")
            .arg("run")
            .arg("-e")
            .arg("delay 0.25")
            .arg("-e")
            .arg("end if")
            .arg("-e")
            .arg("activate")
            .arg("-e")
            .arg("end tell")
            .output()
            .expect("failed to execute process");
    })
    .unwrap_or(());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
