use druid::{
	widget::{
		Checkbox, CrossAxisAlignment, Either, Flex, Label, List, Padding, Scroll, TabInfo,
		TabsPolicy,
	},
	Data, Env, Selector, Widget, WidgetExt,
};

use crate::{
	gnu_data::guid::GUID,
	rs_data::{account::Account, book::Book, page::Page},
};

#[derive(Data, Clone, Copy, PartialEq, Eq)]
pub struct BookTabPolicy;

fn account_item() -> impl Widget<Account> {
	Flex::column()
		.with_child(
			Flex::row()
				.with_child(Checkbox::new("").lens(Account::children_hidden))
				.with_child(Label::new(|data: &Account, _env: &Env| {
					data.name.to_string()
				}))
				.on_click(|ctx, data, env| {
					ctx.submit_command(Selector::new("OpenAccount").with(data.id))
				}),
		)
		.with_child(Either::new(
			|data, _env| data.children_hidden,
			Flex::row(),
			List::new(|| Padding::new((50., 0., 0., 0.), account_item()))
				.with_spacing(2.)
				.lens(Account::children),
		))
		.cross_axis_alignment(CrossAxisAlignment::Start)
		.expand_width()
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
		pages.push(Page::Accounts);
		for page in data.pages.iter() {
			pages.push(page.clone())
		}
		pages
	}

	fn tab_info(&self, key: Self::Key, data: &Self::Input) -> druid::widget::TabInfo<Self::Input> {
		match key {
			Page::Accounts => TabInfo::new("Accounts", false),
			Page::Transactions { filter } => match filter {
				crate::rs_data::page::TransactionFilter::Account(account) => {
					TabInfo::new(data.get_account_name_path(&account), true)
				}
			},
		}
	}

	fn tab_body(&self, key: Self::Key, data: &Self::Input) -> Self::BodyWidget {
		match key {
			Page::Accounts => Scroll::new(account_item().lens(Book::root_account)).boxed(),
			Page::Transactions { filter } => Flex::row().boxed(),
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
