use crate::application::commit_info_localized;
use crate::messages::layout::utility_types::widget_prelude::*;
use crate::messages::prelude::*;

/// A dialog for displaying information on [BuildMetadata] viewable via *Help* > *About KiPaint* in the menu bar.
pub struct AboutKiPaintDialog {
	pub localized_commit_date: String,
	pub localized_commit_year: String,
}

impl DialogLayoutHolder for AboutKiPaintDialog {
	const ICON: &'static str = "GraphiteLogo";
	const TITLE: &'static str = "About KiPaint";

	fn layout_buttons(&self) -> Layout {
		let widgets = vec![TextButton::new("OK").emphasized(true).on_update(|_| FrontendMessage::DialogClose.into()).widget_instance()];

		Layout(vec![LayoutGroup::row(widgets)])
	}

	fn layout_column_2(&self) -> Layout {
		let links = [
			("GraphiteLogo", "Source Code", "https://github.com/mel0k1/KiPaint"),
			("Volunteer", "Report an Issue", "https://github.com/mel0k1/KiPaint/issues"),
			("Heart", "Upstream Project: Graphite", "https://graphite.art/donate/"),
		];
		let mut widgets = links
			.into_iter()
			.map(|(icon, label, url)| {
				TextButton::new(label)
					.icon(icon)
					.flush(true)
					.on_update(|_| FrontendMessage::TriggerVisitLink { url: url.into() }.into())
					.widget_instance()
			})
			.collect::<Vec<_>>();

		// Cloning here and below seems to be necessary to appease the borrow checker, as far as I can tell.
		let localized_commit_year = self.localized_commit_year.clone();
		widgets.push(
			TextButton::new("Licenses")
				.icon("License")
				.flush(true)
				.on_update(move |_| {
					DialogMessage::RequestLicensesDialogWithLocalizedCommitDate {
						localized_commit_year: localized_commit_year.clone(),
					}
					.into()
				})
				.widget_instance(),
		);

		Layout(vec![LayoutGroup::column(widgets)])
	}
}

impl LayoutHolder for AboutKiPaintDialog {
	fn layout(&self) -> Layout {
		Layout(vec![
			LayoutGroup::row(vec![TextLabel::new("About this release").bold(true).widget_instance()]),
			LayoutGroup::row(vec![TextLabel::new(commit_info_localized(&self.localized_commit_date)).multiline(true).widget_instance()]),
			LayoutGroup::row(vec![TextLabel::new(format!("Copyright © {} KiPaint contributors", self.localized_commit_year)).widget_instance()]),
		LayoutGroup::row(vec![TextLabel::new("Built on the Graphite engine (Apache-2.0). CLIP STUDIO PAINT is a trademark of Celsys, Inc.; KiPaint is not affiliated with Celsys.").multiline(true).widget_instance()]),
		])
	}
}
