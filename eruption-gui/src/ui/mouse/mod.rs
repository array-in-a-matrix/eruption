/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::constants;
use glib::clone;
use gtk::prelude::*;

mod hwdevices;

type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug, thiserror::Error)]
pub enum MouseError {
    #[error("Communication with the Eruption daemon failed")]
    CommunicationError,
    // #[error("Invalid layout type specified")]
    // InvalidLayout,
}

/// Initialize page "Mouse"
pub fn initialize_mouse_page(builder: &gtk::Builder) -> Result<()> {
    let mouse_device = hwdevices::get_mouse_device()?;

    let notification_box_global: gtk::Box = builder.get_object("notification_box_global").unwrap();

    let mouse_name_label: gtk::Label = builder.get_object("mouse_device_name_label").unwrap();
    let drawing_area: gtk::DrawingArea = builder.get_object("drawing_area_mouse").unwrap();

    // device name and status
    let make_and_model = mouse_device.get_make_and_model();
    mouse_name_label.set_label(&format!("{} {}", make_and_model.0, make_and_model.1));

    // drawing area / mouse indicator
    drawing_area.connect_draw(move |da: &gtk::DrawingArea, context: &cairo::Context| {
        if let Err(_e) = mouse_device.draw_mouse(&da, &context) {
            notification_box_global.show();
        } else {
            notification_box_global.hide();
        }

        gtk::Inhibit(false)
    });

    glib::timeout_add_local(
        (1000 / constants::TARGET_FPS as u32) / 2,
        clone!(@strong drawing_area => move || {
            drawing_area.queue_draw();
            Continue(true)
        }),
    );

    Ok(())
}
