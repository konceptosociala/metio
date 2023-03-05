mod editor;
mod scene;

use std::path::PathBuf;
use gtk::prelude::*;
use scene::*;
use editor::*;

fn main() {
    let application = gtk::Application::builder()
        .application_id("org.konceptosociala.redakt")
        .build();
    
    //~ application.connect_activate(build_editor);
    application.connect_activate(build_scene);
    application.run();
}
