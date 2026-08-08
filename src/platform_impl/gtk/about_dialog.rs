use gtk::prelude::*;

use crate::AboutMetadata;

/// Displays an about dialog using GTK with the provided metadata.
pub struct AboutDialog {
    metadata: AboutMetadata,
}

impl AboutDialog {
    /// Create a new about dialog using `metadata` but without showing it.
    pub fn new(metadata: AboutMetadata) -> AboutDialog {
        AboutDialog { metadata }
    }

    /// Show the about dialog.
    ///
    /// GTK is single-threaded, so when this is called on the main thread the
    /// dialog is shown directly and this blocks until it is closed. When
    /// called from another thread (e.g. the ksni tray service thread), the
    /// dialog is dispatched to the GTK main context instead and this returns
    /// immediately.
    pub fn show(&self) {
        let metadata = self.metadata.clone();
        if gtk::is_initialized_main_thread() {
            show_dialog(metadata);
        } else {
            gtk::glib::MainContext::default().invoke(move || show_dialog(metadata));
        }
    }
}

fn show_dialog(metadata: AboutMetadata) {
    let mut builder = gtk::AboutDialog::builder().modal(true).resizable(false);

    if let Some(name) = &metadata.name {
        builder = builder.program_name(name);
    }
    if let Some(version) = &metadata.full_version() {
        builder = builder.version(version);
    }
    if let Some(authors) = &metadata.authors {
        builder = builder.authors(authors.clone());
    }
    if let Some(comments) = &metadata.comments {
        builder = builder.comments(comments);
    }
    if let Some(copyright) = &metadata.copyright {
        builder = builder.copyright(copyright);
    }
    if let Some(license) = &metadata.license {
        builder = builder.license(license);
    }
    if let Some(website) = &metadata.website {
        builder = builder.website(website);
    }
    if let Some(website_label) = &metadata.website_label {
        builder = builder.website_label(website_label);
    }
    if let Some(icon) = &metadata.icon {
        builder = builder.logo(&icon.to_pixbuf());
    }

    let about = builder.build();

    about.run();

    unsafe {
        about.destroy();
    }
}
