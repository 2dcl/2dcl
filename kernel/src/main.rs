use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    http::ResponseBuilder,
    webview::WebViewBuilder,
};

pub fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        // .with_decorations(false)
        // There are actually three layer of background color when creating webview window.
        // The first is window background...
        .with_transparent(true)
        .with_visible(false)
        .with_title("2dcl kernel")
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new(window)
        .unwrap()
        // The second is on webview...
        .with_transparent(true)
        .with_visible(false)
        // And the last is in html.
        .with_custom_protocol("wry".into(), move |request| {
            let path = request
                .uri()
                .replace("wry://", "")
                .replace("?ws=ws://127.0.0.1:8080", "");

            match path.as_str() {
                "index" => ResponseBuilder::new()
                    .mimetype("text/html")
                    .body(include_str!("index.html").as_bytes().to_vec()),
                _ => ResponseBuilder::new()
                    .status(400)
                    .body("".as_bytes().to_vec()),
            }
        })
        .with_url("wry://index?ws=ws://127.0.0.1:8080")
        .unwrap();

    #[cfg(debug_assertions)]
    let webview = webview.with_devtools(true);
    let webview = webview.build().unwrap();

    #[cfg(debug_assertions)]
    webview.open_devtools();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
