mod app;
mod message;
mod styles;
mod widgets;

// Punto de entrada de la aplicación
use app::MyApp;

fn main() -> iced::Result {
    iced::run("OcoRust", MyApp::update, MyApp::view)
}
