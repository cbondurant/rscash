use druid::{
	im::Vector,
	lens::LensExt,
	lens::Map,
	widget::{
		Checkbox, Controller, CrossAxisAlignment, Either, Flex, Label, List, Padding, Scroll,
		TabInfo, Tabs, TabsPolicy,
	},
	Data, Env, Event, Selector, Widget, WidgetExt,
};

use crate::{
	gnu_data::guid::GUID,
	rs_data::{
		account::Account,
		book::Book,
		page::{Page, TransactionFilter},
		transaction::Transaction,
	},
};

use super::transactions::TransactionWidget;

#[derive(Data, Clone, Copy, PartialEq, Eq)]
pub struct BookTabPolicy;

fn account_item() -> impl Widget<Account> {
	Flex::column()
		.with_child(
			Flex::row()
				.with_child(Checkbox::new("").lens(Account::children_hidden))
				.with_child(
					Label::new(|data: &Account, _env: &Env| data.name.to_string()).on_click(
						|ctx, data, _env| {
							ctx.submit_command(Selector::new("OpenAccount").with(data.id))
						},
					),
				),
		)
		.with_child(Either::new(
			|data, _env| data.children_hidden,
			Flex::row(),
			List::new(|| Padding::new((50., 0., 0., 0.), account_item()))
				.with_spacing(2.)
				.lens(Account::children),
		))
		.cross_axis_alignment(CrossAxisAlignment::Start)
}

impl TabsPolicy for BookTabPolicy {
	type Key = Page;

	type Input = Book;

	type BodyWidget = Box<dyn Widget<Self::Input>>;

	type LabelWidget = Label<Self::Input>;

	type Build = ();

	fn tabs_changed(&self, old_data: &Self::Input, data: &Self::Input) -> bool {
		old_data.pages != data.pages
	}

	fn tabs(&self, data: &Self::Input) -> Vec<Self::Key> {
		let mut pages = Vec::new();
		pages.push(Page::Accounts {
			selected_page: data.root_account.id,
		});
		for page in data.pages.iter() {
			pages.push(page.clone())
		}
		pages
	}

	fn tab_info(&self, key: Self::Key, data: &Self::Input) -> druid::widget::TabInfo<Self::Input> {
		match key {
			Page::Accounts { selected_page: _ } => TabInfo::new("Accounts", false),
			Page::Transactions(filter) => match filter {
				crate::rs_data::page::TransactionFilter::Account(account) => {
					TabInfo::new(data.get_account_name_path(&account), true)
				}
			},
		}
	}

	fn tab_body(&self, key: Self::Key, data: &Self::Input) -> Self::BodyWidget {
		match key {
			Page::Accounts { selected_page: _ } => Scroll::new(
				List::new(|| account_item()).lens(Book::root_account.then(Account::children)),
			)
			.expand_width()
			.boxed(),
			Page::Transactions(filter) => Flex::row()
				.with_child(List::new(|| {
					Label::new(|disc: &Transaction, _env: &Env| disc.description.clone())
				}))
				.lens(Book::transactions.map(
					move |transaction| {
						transaction
							.iter()
							.cloned()
							//.filter(|transaction| filter.transaction_filtered(transaction))
							.collect()
					},
					|transaction, second: Vector<Transaction>| *transaction = second.clone(),
				))
				.boxed(),
		}
	}

	fn tab_label(
		&self,
		_key: Self::Key,
		info: druid::widget::TabInfo<Self::Input>,
		_data: &Self::Input,
	) -> Self::LabelWidget {
		Label::new(info.name)
	}
}

pub struct TabsController;

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
				Some(guid) => data
					.pages
					.push_back(Page::Transactions(TransactionFilter::Account(*guid))),
				None => (),
			},
			_ => (),
		}
		child.event(ctx, event, data, env)
	}
}
