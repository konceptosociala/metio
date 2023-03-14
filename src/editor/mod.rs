use gtk::{CssProvider, StyleContext};
use gtk::prelude::*;
use sourceview4::{prelude::*};
use sourceview4 as gtksv;

pub fn build_code_editor(application: &gtk::Application){
    let glade_src = include_str!("../../layouts/editor.glade");    
    let builder = gtk::Builder::from_string(glade_src);
    
    let language_manager = gtksv::LanguageManager::new();
    //~ let language = language_manager.guess_language(Some("src/desl.lang"), None).unwrap();
    let language = language_manager.language("rust").unwrap();
    
    let buffer = gtksv::Buffer::new(None::<&gtk::TextTagTable>);
    buffer.set_highlight_syntax(true);
    buffer.set_language(Some(&language));
    
    let container: gtk::Box = builder.object("box").unwrap();
    
    let source_view = gtksv::View::with_buffer(&buffer);
    source_view.set_monospace(true);
    source_view.set_background_pattern(sourceview4::BackgroundPatternType::Grid);
    source_view.set_show_line_numbers(true);
    source_view.set_highlight_current_line(true);
    source_view.set_tab_width(4);
    source_view.set_hexpand(true);
    container.add(&source_view);
    container.set_child_position(&source_view, 0);
    
    let source_map: gtksv::Map = builder.object("source_map").unwrap();
    source_map.set_view(&source_view);
    
    let window: gtk::ApplicationWindow = builder.object("window").unwrap();
    window.set_application(Some(application));
    window.maximize();
    window.show_all();
}

pub fn build_editor(application: &gtk::Application){
    let glade_src = include_str!("../../layouts/layout.glade");
    let builder = gtk::Builder::from_string(glade_src);
    
    let window: gtk::ApplicationWindow = builder.object("main_window").unwrap();
    
    let provider = CssProvider::new();
    provider.load_from_path("themes/Orchis-Orange/Orchis-Orange-Dark/gtk-3.0/gtk.css").unwrap();
    
    let context = window.style_context();
    StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    
    window.set_application(Some(application));
    window.maximize();
    window.show_all();
}
