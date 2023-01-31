use druid::{widget::Tabs, AppLauncher, Widget, WidgetExt, WindowDesc};
use gnu_data::gnc_v2::GCNv2;
use libflate::gzip::Decoder;
use rs_data::book::Book;
use std::{fs::File, io::Write};
use widgets::book_tabs::{BookTabPolicy, TabsController};
mod gnu_data;
mod rs_data;
mod widgets;

fn build_app() -> impl Widget<Book> {
	Tabs::for_policy(BookTabPolicy).controller(TabsController)
}

fn main() {
	let source = File::open("test.gnucash").unwrap();
	let reader = Decoder::new(source).unwrap();

	let mut deserialized: GCNv2 = match serde_xml_rs::from_reader(reader) {
		Ok(val) => val,
		Err(e) => {
			println!("{e:?}");
			return;
		}
	};

	let mut j_file = File::create("test.json").unwrap();
	let text = serde_json::to_string_pretty(&deserialized).unwrap();
	j_file.write_all(text.as_str().as_bytes()).unwrap();

	println!("{:?}", deserialized.books[0].accounts.len());

	let new_book: rs_data::book::Book = deserialized.books.remove(0).into();

	let main_win = WindowDesc::new(build_app());

	AppLauncher::with_window(main_win)
		.launch(new_book)
		.expect("Failed to launch");
}
