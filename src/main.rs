use data::{account::Account, book::Book, gnc_v2::GCNv2};
use druid::{
	widget::{Label, List, Scroll},
	AppLauncher, Env, Widget, WidgetExt, WindowDesc,
};
use libflate::gzip::Decoder;
use std::fs::File;
mod data;

fn build_app() -> impl Widget<Book> {
	Scroll::new(
		List::new(|| Label::new(|data: &Account, _env: &Env| data.name.to_string()))
			.lens(Book::accounts),
	)
}

fn main() {
	let source = File::open("test.gnucash").unwrap();
	let reader = Decoder::new(source).unwrap();

	let deserialized: GCNv2 = match serde_xml_rs::from_reader(reader) {
		Ok(val) => val,
		Err(e) => {
			println!("{e:?}");
			return;
		}
	};

	println!("{:?}", deserialized.books[0].accounts.len());

	let main_win = WindowDesc::new(build_app);

	AppLauncher::with_window(main_win)
		.launch(deserialized.books[0].clone())
		.expect("Failed to launch");
}
