use http_server::{
    application::Application,
    transport::{Transport, TransportTrait},
};
use std::sync::Arc;
fn main() {
    let app = Application::get;
    let app_arc = Arc::new(app);

    Transport::new(8080).listen(app_arc).unwrap();
}
