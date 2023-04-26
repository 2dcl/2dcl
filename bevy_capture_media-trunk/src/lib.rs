#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[allow(clippy::type_complexity)]
pub mod data;
pub mod formats;
#[cfg(any(feature = "gif", feature = "png"))]
mod image_utils;
mod management;
mod render;
#[cfg(target_arch = "wasm32")]
mod web_utils;

mod plugin {
	use bevy::app::{App, CoreSet, Plugin};
	use bevy::prelude::*;
	use bevy::render::{RenderApp, RenderSet};

	use super::*;

	#[derive(Resource, PartialEq, Eq)]
	pub enum CaptureState {
		Idle,
		TakingScreenshot,
		Finished,
	}

	pub struct BevyCapturePlugin;
	impl Plugin for BevyCapturePlugin {
		fn build(&self, app: &mut App) {
			let tracking_tracker = data::ActiveRecorders::default();
			let data_smuggler = data::SharedDataSmuggler::default();

			app.add_event::<data::StartTrackingCamera>()
				.add_event::<data::StopTrackingCamera>()
				.insert_resource(tracking_tracker)
				.insert_resource(data_smuggler.clone())
				.insert_resource(CaptureState::Idle)
				.add_system(management::clean_cameras.in_base_set(CoreSet::First))
				.add_system(management::move_camera_buffers.in_base_set(CoreSet::First))
				.add_system(management::sync_tracking_cameras.in_base_set(CoreSet::PostUpdate))
				.add_system(
					management::start_tracking_orthographic_camera.in_base_set(CoreSet::PostUpdate),
				);

			#[cfg(feature = "gif")]
			{
				app.add_event::<formats::gif::CaptureGifRecording>()
					.add_system(
						formats::gif::capture_gif_recording.in_base_set(CoreSet::PostUpdate),
					);

				#[cfg(not(target_arch = "wasm32"))]
				app.add_system(
					management::clean_unmonitored_tasks::<formats::gif::SaveGifRecording>
						.in_base_set(CoreSet::Last),
				);
			}
			#[cfg(feature = "png")]
			{
				app.add_event::<formats::png::SavePngFile>()
					.add_system(formats::png::save_single_frame.in_base_set(CoreSet::PostUpdate));

				#[cfg(not(target_arch = "wasm32"))]
				app.add_system(
					management::clean_unmonitored_tasks::<formats::png::SaveFrameTask>
						.in_base_set(CoreSet::Last),
				);
			}

			let render_app = app.get_sub_app_mut(RenderApp)
				.expect("bevy::capture_media will not work without the render app. Either enable this sub app, or disable bevy::capture_media");

			render_app
				.insert_resource(data_smuggler)
				.add_system(render::smuggle_frame.in_set(RenderSet::Render));
		}
	}
}

pub use data::MediaCapture;
pub use plugin::BevyCapturePlugin;
pub use plugin::CaptureState;
