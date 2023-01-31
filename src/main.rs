use druid::{
	widget::{Controller, Tabs},
	AppLauncher, Command, Event, Key, Selector, Widget, WidgetExt, WindowDesc,
};
use gnu_data::{gnc_v2::GCNv2, guid::GUID};
use libflate::gzip::Decoder;
use rs_data::{
	book::Book,
	page::{Page, TransactionFilter},
};
use std::{fs::File, io::Write};
use widgets::book_tabs::BookTabPolicy;
mod gnu_data;
mod rs_data;
mod widgets;

struct TabsController;
impl Controller<Book, Tabs<BookTabPolicy>> for TabsController {
	fn event(
		&mut self,
		child: &mut Tabs<BookTabPolicy>,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut Book,
		env: &druid::Env,
	) {
		match event {
			Event::Command(command) => match command.get(Selector::<GUID>::new("OpenAccount")) {
				Some(guid) => data.pages.push_back(Page::Transactions {
					filter: TransactionFilter::Account(*guid),
				}),
				None => (),
			},
			_ => (),
		}
		child.event(ctx, event, data, env)
	}
}

fn build_app() -> impl Widget<Book> {
	Tabs::for_policy(BookTabPolicy)
		.controller(TabsController)
		.expand_width()
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
