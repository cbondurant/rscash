use druid::{im::Vector, Widget};

use crate::rs_data::transaction::Transaction;

pub struct TransactionWidget;

impl Widget<Vector<Transaction>> for TransactionWidget {
	fn event(
		&mut self,
		ctx: &mut druid::EventCtx,
		event: &druid::Event,
		data: &mut Vector<Transaction>,
		env: &druid::Env,
	) {
		todo!()
	}

	fn lifecycle(
		&mut self,
		ctx: &mut druid::LifeCycleCtx,
		event: &druid::LifeCycle,
		data: &Vector<Transaction>,
		env: &druid::Env,
	) {
		todo!()
	}

	fn update(
		&mut self,
		ctx: &mut druid::UpdateCtx,
		old_data: &Vector<Transaction>,
		data: &Vector<Transaction>,
		env: &druid::Env,
	) {
		todo!()
	}

	fn layout(
		&mut self,
		ctx: &mut druid::LayoutCtx,
		bc: &druid::BoxConstraints,
		data: &Vector<Transaction>,
		env: &druid::Env,
	) -> druid::Size {
		todo!()
	}

	fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Vector<Transaction>, env: &druid::Env) {
		todo!()
	}
}
