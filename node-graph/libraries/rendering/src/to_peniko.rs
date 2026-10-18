use core_types::BlendMode;
use core_types::color::SRGBA8;
use vello::peniko;

pub trait BlendModeExt {
	fn to_peniko(&self) -> peniko::Mix;
}

pub trait ToPenikoColor {
	fn to_peniko_color(&self) -> peniko::Color;
}

impl ToPenikoColor for SRGBA8 {
	#[inline(always)]
	fn to_peniko_color(&self) -> peniko::Color {
		peniko::Color::from_rgba8(self.red, self.green, self.blue, self.alpha)
	}
}

/// Maps a blend mode to its full peniko equivalent, including the Porter-Duff compose step used by modes that modify the backdrop's alpha.
pub fn to_peniko_blend_mode(blend_mode: &BlendMode) -> peniko::BlendMode {
	match blend_mode {
		// Erase and Multiply Alpha only scale the backdrop's coverage, which Porter-Duff compose operations express directly
		BlendMode::Erase => peniko::BlendMode::new(peniko::Mix::Normal, peniko::Compose::DestOut),
		BlendMode::MultiplyAlpha => peniko::BlendMode::new(peniko::Mix::Normal, peniko::Compose::DestIn),
		blend_mode => peniko::BlendMode::new(blend_mode.to_peniko(), peniko::Compose::SrcOver),
	}
}

impl BlendModeExt for BlendMode {
	fn to_peniko(&self) -> peniko::Mix {
		match self {
			// Normal group
			BlendMode::Normal => peniko::Mix::Normal,
			// Darken group
			BlendMode::Darken => peniko::Mix::Darken,
			BlendMode::Multiply => peniko::Mix::Multiply,
			BlendMode::ColorBurn => peniko::Mix::ColorBurn,
			// Lighten group
			BlendMode::Lighten => peniko::Mix::Lighten,
			BlendMode::Screen => peniko::Mix::Screen,
			BlendMode::ColorDodge => peniko::Mix::ColorDodge,
			// Contrast group
			BlendMode::Overlay => peniko::Mix::Overlay,
			BlendMode::SoftLight => peniko::Mix::SoftLight,
			BlendMode::HardLight => peniko::Mix::HardLight,
			// Inversion group
			BlendMode::Difference => peniko::Mix::Difference,
			BlendMode::Exclusion => peniko::Mix::Exclusion,
			// Component group
			BlendMode::Hue => peniko::Mix::Hue,
			BlendMode::Saturation => peniko::Mix::Saturation,
			BlendMode::Color => peniko::Mix::Color,
			BlendMode::Luminosity => peniko::Mix::Luminosity,
			_ => todo!(),
		}
	}
}
