use gtk::*;
use pango::*;
use sourceview::*;

/// Ez a struktúra felelős a kódszerkesztő kezeléséért.
#[derive(Debug)]
pub struct Source {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: Buffer,
}

#[derive(Debug)]
pub struct Content {
    pub container: Paned,
    pub source: Source,
}

impl Source {
	pub fn new() -> Source {
		let buff = Buffer::new(None);
		let view = View::new_with_buffer(&buff);
		let container = ScrolledWindow::new(None, None);

		container.add(&view);

		Source::configure(&view, &buff);

		Source {container, buff, view}
	}

	fn configure(view: &View, buff: &Buffer) {
		WidgetExt::override_font(view, &FontDescription::from_string("monospace"));

		LanguageManager::new()
			.get_language("rust")
			.map(|rust| buff.set_language(&rust));

		let stylemanager = StyleSchemeManager::new();

		stylemanager
			.get_scheme("Builder")
			.or(stylemanager.get_scheme("Classic"))
			.map(|theme| buff.set_style_scheme(&theme));

		view.set_show_line_numbers(true);
	    view.set_monospace(true);
	    view.set_insert_spaces_instead_of_tabs(true);
	    view.set_indent_width(4);
	    view.set_smart_backspace(true);
	    view.set_right_margin(100);
	    view.set_left_margin(10);
	    view.set_show_right_margin(true);
	    view.set_background_pattern(BackgroundPatternType::Grid);
	}
}

impl Content {
	pub fn new() -> Content {
		let container = Paned::new(Orientation::Horizontal);
		let source = Source::new();

		container.pack1(&source.container, true, true);

		 Content {container, source}
	}
}