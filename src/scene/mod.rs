use gtk::prelude::*;

pub fn build_scene(application: &gtk::Application){
    let glade_src = include_str!("../../layouts/3d.glade");    
    let builder = gtk::Builder::from_string(glade_src);
    
    let window: gtk::Window = builder.object("gl_window").unwrap();
    
    let gl_area: gtk::GLArea = builder.object("gl_area").unwrap();
    
    //~ gl_area.connect_render(move |gl_area, _gl_context| {
        
    //~ });
    
    window.set_application(Some(application));
    window.maximize();
    window.show_all();
}
