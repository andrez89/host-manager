use crate::{
    hosts,
    i18n::{Language, Text},
    store::{self, Site},
};
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, Entry, HeaderBar, Label, ListBox,
    ListBoxRow, Orientation, ScrolledWindow, Switch, prelude::*,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub fn run() {
    let app = Application::builder()
        .application_id("it.local.host-manager")
        .build();
    app.connect_activate(|app| {
        let language = Language::system();
        let css = gtk::CssProvider::new();
        css.load_from_data(
            "window { background: #18181a; color: #f0eff4; } \
             headerbar { min-height: 36px; padding: 0 8px; background: #18181a; border-bottom: 1px solid #2a2a2d; } \
             headerbar label { font-weight: 600; } \
             button.close-dot { min-width: 22px; min-height: 22px; padding: 0; border: none; border-radius: 999px; background: #303033; color: #d7d5dc; font-size: 16px; } \
             button.close-dot:hover { background: #b42336; color: white; } \
             entry { min-height: 34px; background: #1b1b1e; color: #f0eff4; border: 1px solid #38383e; border-radius: 10px; padding: 0 10px; } \
             entry:focus { border-color: #4edea3; } \
             button { min-height: 34px; padding: 0 10px; background: #2e2e33; color: #e4e1e6; border: 1px solid #38383e; border-radius: 9px; } \
             button:hover { background: #39393c; } \
             button.primary { background: #10b981; color: #002a1b; border-color: #10b981; font-weight: 600; } \
             button.primary:hover { background: #4edea3; } \
             list { background: #1b1b1e; border: 1px solid #353438; border-radius: 12px; } \
             list row { padding: 8px; border-bottom: 1px solid #2a2a2d; } \
             list row.blocked-row { background: #202923; } \
             list row.unblocked-row { background: #1b1b1e; } \
             list row:last-child { border-bottom: none; } \
             label { color: #e4e1e6; } \
             label.summary, label.hint { color: #bbcabf; } \
             label.site-name { font-weight: 600; } \
             label.blocked { color: #4edea3; font-weight: 600; } \
             label.unblocked { color: #ffb95f; } \
             switch { min-width: 30px; min-height: 12px; margin: 0 4px; padding: 0; border-radius: 999px; } \
             switch > slider { min-width: 10px; min-height: 10px; margin: 1px; border-radius: 999px; } \
             switch:checked { background: #10b981; } \
             button.locked { color: #ffb95f; }",
        );
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(),
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let sites = Rc::new(RefCell::new(store::load()));
        let root = GtkBox::new(Orientation::Vertical, 12);
        root.set_margin_top(14);
        root.set_margin_bottom(16);
        root.set_margin_start(16);
        root.set_margin_end(16);

        let add = GtkBox::new(Orientation::Horizontal, 8);
        let input = Entry::builder()
            .placeholder_text(language.text(Text::AddPlaceholder))
            .hexpand(true)
            .build();
        let add_button = Button::with_label(language.text(Text::Add));
        add_button.add_css_class("primary");
        add.append(&input);
        add.append(&add_button);
        root.append(&add);

        let toolbar = GtkBox::new(Orientation::Horizontal, 6);
        let summary = Label::new(None);
        summary.add_css_class("summary");
        summary.set_xalign(0.0);
        summary.set_hexpand(true);
        let block_all = Button::with_label(language.text(Text::BlockAll));
        let unblock_all = Button::with_label(language.text(Text::UnblockAll));
        let hide_locked = Rc::new(Cell::new(store::load_hide_locked()));
        let visibility = Button::new();
        visibility.set_child(Some(&gtk::Image::from_icon_name(if hide_locked.get() {
            "view-reveal-symbolic"
        } else {
            "view-conceal-symbolic"
        })));
        visibility.set_tooltip_text(Some(language.text(if hide_locked.get() {
            Text::ShowLocked
        } else {
            Text::HideLocked
        })));
        toolbar.append(&summary);
        toolbar.append(&visibility);
        toolbar.append(&block_all);
        toolbar.append(&unblock_all);
        root.append(&toolbar);

        let list = ListBox::new();
        list.add_css_class("boxed-list");
        let scroller = ScrolledWindow::builder()
            .vexpand(true)
            .min_content_height(100)
            .max_content_height(300)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&list)
            .build();
        root.append(&scroller);

        let status = Label::new(Some(language.text(Text::AdminRequired)));
        status.add_css_class("hint");
        status.set_xalign(0.0);
        root.append(&status);
        let apply = Button::with_label(language.text(Text::ApplyHosts));
        apply.add_css_class("primary");
        apply.set_halign(gtk::Align::End);
        root.append(&apply);

        let renderer: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let refresh: Rc<dyn Fn()> = {
            let list = list.clone();
            let summary = summary.clone();
            let sites = sites.clone();
            let hide_locked = hide_locked.clone();
            let renderer_ref = renderer.clone();
            Rc::new(move || {
                while let Some(row) = list.row_at_index(0) {
                    list.remove(&row);
                }
                let snapshot = sites.borrow().clone();
                let blocked = snapshot.iter().filter(|site| site.blocked).count();
                summary.set_text(&language.blocked_summary(blocked, snapshot.len()));

                for (index, site) in snapshot.iter().enumerate() {
                    if hide_locked.get() && site.locked {
                        continue;
                    }
                    let row = ListBoxRow::new();
                    let line = GtkBox::new(Orientation::Horizontal, 9);
                    let dot = Label::new(Some(if site.blocked { "●" } else { "●" }));
                    dot.add_css_class(if site.blocked { "blocked" } else { "unblocked" });
                    let name = Label::new(Some(&site.domain));
                    name.add_css_class("site-name");
                    name.set_xalign(0.0);
                    name.set_hexpand(true);
                    let state = Label::new(Some(language.text(if site.blocked {
                        Text::On
                    } else {
                        Text::Off
                    })));
                    state.add_css_class(if site.blocked { "blocked" } else { "unblocked" });
                    row.add_css_class(if site.blocked {
                        "blocked-row"
                    } else {
                        "unblocked-row"
                    });
                    let toggle = Switch::builder()
                        .active(site.blocked)
                        .sensitive(!site.locked)
                        .width_request(36)
                        .height_request(20)
                        .valign(gtk::Align::Center)
                        .tooltip_text(language.text(if site.locked {
                            Text::UnlockToToggle
                        } else {
                            Text::ToggleSite
                        }))
                        .build();
                    let locked = Button::new();
                    locked.set_child(Some(&gtk::Image::from_icon_name(if site.locked {
                        "changes-prevent-symbolic"
                    } else {
                        "changes-allow-symbolic"
                    })));
                    locked.set_tooltip_text(Some(language.text(if site.locked {
                        Text::Unlock
                    } else {
                        Text::Lock
                    })));
                    if site.locked {
                        locked.add_css_class("locked");
                    }
                    let remove = Button::with_label(language.text(Text::Remove));

                    {
                        let sites = sites.clone();
                        let renderer = renderer_ref.clone();
                        toggle.connect_state_set(move |_, active| {
                            if let Some(site) = sites.borrow_mut().get_mut(index) {
                                if !site.locked {
                                    site.blocked = active;
                                }
                            }
                            let _ = store::save(&sites.borrow());
                            if let Some(render) = renderer.borrow().as_ref() {
                                render();
                            }
                            glib::Propagation::Stop
                        });
                    }
                    {
                        let sites = sites.clone();
                        let renderer = renderer_ref.clone();
                        locked.connect_clicked(move |_| {
                            if let Some(site) = sites.borrow_mut().get_mut(index) {
                                site.locked = !site.locked;
                            }
                            let _ = store::save(&sites.borrow());
                            if let Some(render) = renderer.borrow().as_ref() {
                                render();
                            }
                        });
                    }
                    {
                        let sites = sites.clone();
                        let renderer = renderer_ref.clone();
                        remove.connect_clicked(move |_| {
                            sites.borrow_mut().remove(index);
                            let _ = store::save(&sites.borrow());
                            if let Some(render) = renderer.borrow().as_ref() {
                                render();
                            }
                        });
                    }

                    line.append(&dot);
                    line.append(&name);
                    line.append(&state);
                    line.append(&toggle);
                    line.append(&locked);
                    line.append(&remove);
                    row.set_child(Some(&line));
                    list.append(&row);
                }
            })
        };
        *renderer.borrow_mut() = Some(Box::new({
            let refresh = refresh.clone();
            move || refresh()
        }));

        visibility.connect_clicked({
            let hidden = hide_locked.clone();
            let visibility = visibility.clone();
            let refresh = refresh.clone();
            move |_| {
                let hide = !hidden.get();
                hidden.set(hide);
                let _ = store::save_hide_locked(hide);
                visibility.set_child(Some(&gtk::Image::from_icon_name(if hide {
                    "view-reveal-symbolic"
                } else {
                    "view-conceal-symbolic"
                })));
                visibility.set_tooltip_text(Some(language.text(if hide {
                    Text::ShowLocked
                } else {
                    Text::HideLocked
                })));
                refresh();
            }
        });
        add_button.connect_clicked({
            let sites = sites.clone();
            let input = input.clone();
            let refresh = refresh.clone();
            move |_| {
                let domain = input
                    .text()
                    .trim()
                    .to_ascii_lowercase()
                    .trim_start_matches("https://")
                    .trim_start_matches("http://")
                    .trim_end_matches('/')
                    .to_string();
                if domain.is_empty() || domain.contains('/') || !domain.contains('.') {
                    return;
                }
                if !sites.borrow().iter().any(|site| site.domain == domain) {
                    sites.borrow_mut().push(Site {
                        domain,
                        blocked: false,
                        locked: false,
                    });
                }
                input.set_text("");
                refresh();
            }
        });
        for (button, blocked) in [(&block_all, true), (&unblock_all, false)] {
            button.connect_clicked({
                let sites = sites.clone();
                let refresh = refresh.clone();
                move |_| {
                    store::set_unlocked_sites(&mut sites.borrow_mut(), blocked);
                    let _ = store::save(&sites.borrow());
                    refresh();
                }
            });
        }
        apply.connect_clicked({
            let sites = sites.clone();
            let status = status.clone();
            let refresh = refresh.clone();
            move |_| {
                match hosts::apply(&sites.borrow()) {
                    Ok(()) => {
                        let _ = store::save(&sites.borrow());
                        status.set_text(language.text(Text::ApplySuccess));
                    }
                    Err(error) => status.set_text(&format!(
                        "{} {error}",
                        language.text(Text::ApplyError)
                    )),
                }
                refresh();
            }
        });
        refresh();
        let _ = store::save(&sites.borrow());

        let header = HeaderBar::new();
        header.set_show_title_buttons(false);
        header.set_title_widget(Some(&Label::new(Some("Host Manager"))));
        let close = Button::with_label("×");
        close.add_css_class("close-dot");
        header.pack_end(&close);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Host Manager")
            .default_width(560)
            .default_height(520)
            .child(&root)
            .build();
        window.set_titlebar(Some(&header));
        close.connect_clicked({
            let window = window.clone();
            move |_| window.close()
        });
        window.present();
    });
    app.run();
}
