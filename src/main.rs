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
    
    //~ let glade_src = include_str!("../layouts/layout.glade");    
    //~ let builder = gtk::Builder::from_string(glade_src);
    
    //~ let window: gtk::Window = builder.object("gl_window").unwrap();
    //~ let gl_area: gtk::GLArea = builder.object("gl_area").unwrap();
    
    //~ let despero = Despero::init();
    
    //~ gl_area.connect_render(move |gl_area, _gl_context| {
        
    //~ });
    
    //~ window.set_application(Some(application));
    //~ window.maximize();
    //~ window.show_all();
    
    application.connect_activate(build_editor);
    application.run();
}
