#![windows_subsystem = "windows"]

extern crate Crusty;

fn main() {
    let app = Crusty::Application::New();
    app.Run();
}
