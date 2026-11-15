use crate::{hosts, store::{self, Site}};
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, Entry, HeaderBar, Label, ListBox, ListBoxRow, Orientation};
use std::{cell::RefCell, rc::Rc};

pub fn run() {
    let app = Application::builder().application_id("it.local.host-manager").build();
    app.connect_activate(|app| {
        let css = gtk::CssProvider::new();
        css.load_from_data("window { background: #18181a; color: #f0eff4; } headerbar { min-height: 36px; padding: 0 8px; background: #18181a; border-bottom: 1px solid #2a2a2d; } headerbar label { font-weight: 600; } button.close-dot { min-width: 22px; min-height: 22px; padding: 0; border: none; border-radius: 999px; background: #303033; color: #d7d5dc; font-size: 16px; } button.close-dot:hover { background: #b42336; color: white; } entry { background: #1b1b1e; color: #f0eff4; border: 1px solid #38383e; border-radius: 10px; padding: 8px 10px; } entry:focus { border-color: #4edea3; } button { background: #2e2e33; color: #e4e1e6; border: 1px solid #38383e; border-radius: 9px; padding: 6px 12px; } button:hover { background: #39393c; } button.primary { background: #10b981; color: #002a1b; border-color: #10b981; font-weight: 600; } button.primary:hover { background: #4edea3; } list { background: #1b1b1e; border: 1px solid #353438; border-radius: 12px; } list row { padding: 8px; border-bottom: 1px solid #2a2a2d; } list row.blocked-row { background: #202923; } list row.unblocked-row { background: #1b1b1e; } list row:last-child { border-bottom: none; } label { color: #e4e1e6; } label.summary, label.hint { color: #bbcabf; } label.site-name { font-weight: 600; } label.blocked { color: #4edea3; font-weight: 600; } label.unblocked { color: #bbcabf; }");
        gtk::style_context_add_provider_for_display(&gtk::gdk::Display::default().unwrap(), &css, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        let sites = Rc::new(RefCell::new(store::load()));
        let root = GtkBox::new(Orientation::Vertical, 12);
        root.set_margin_top(18); root.set_margin_bottom(18); root.set_margin_start(18); root.set_margin_end(18);
        let add = GtkBox::new(Orientation::Horizontal, 8);
        let input = Entry::builder().placeholder_text("Aggiungi un sito (es. facebook.com)").hexpand(true).build();
        let add_button = Button::with_label("Aggiungi"); add_button.add_css_class("primary"); add.append(&input); add.append(&add_button); root.append(&add);
        let summary = Label::new(None); summary.add_css_class("summary"); summary.set_xalign(0.0); root.append(&summary);
        let list = ListBox::new(); list.add_css_class("boxed-list"); root.append(&list);
        let status = Label::new(Some("Le modifiche richiedono autorizzazione amministrativa.")); status.add_css_class("hint"); status.set_xalign(0.0); root.append(&status);
        let apply = Button::with_label("Applica a /etc/hosts"); apply.add_css_class("primary"); apply.set_halign(gtk::Align::End); root.append(&apply);
        let renderer: Rc<RefCell<Option<Box<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let refresh: Rc<dyn Fn()> = {
            let list = list.clone(); let summary = summary.clone(); let sites = sites.clone();
            let renderer_ref = renderer.clone();
            Rc::new(move || {
                while let Some(row) = list.row_at_index(0) { list.remove(&row); }
                let blocked = sites.borrow().iter().filter(|s| s.blocked).count();
                summary.set_text(&format!("{blocked} siti bloccati su {}", sites.borrow().len()));
                let snapshot = sites.borrow().clone();
                for (index, site) in snapshot.iter().enumerate() {
                    let row = ListBoxRow::new(); let line = GtkBox::new(Orientation::Horizontal, 10);
                    let name = Label::new(Some(&site.domain)); name.add_css_class("site-name"); name.set_xalign(0.0); name.set_hexpand(true);
                    let state = Label::new(Some(if site.blocked { "●  Bloccato" } else { "○  Sbloccato" }));
                    state.add_css_class(if site.blocked { "blocked" } else { "unblocked" });
                    row.add_css_class(if site.blocked { "blocked-row" } else { "unblocked-row" });
                    let toggle = Button::with_label(if site.blocked { "Sblocca" } else { "Blocca" });
                    let remove = Button::with_label("Rimuovi");
                    { let sites=sites.clone(); let renderer=renderer_ref.clone(); toggle.connect_clicked(move |_| { let mut sites=sites.borrow_mut(); if let Some(site)=sites.get_mut(index) { site.blocked = !site.blocked; } let _=store::save(&sites); drop(sites); if let Some(render)=renderer.borrow().as_ref() { render(); } }); }
                    { let sites=sites.clone(); let renderer=renderer_ref.clone(); remove.connect_clicked(move |_| { sites.borrow_mut().remove(index); let _=store::save(&sites.borrow()); if let Some(render)=renderer.borrow().as_ref() { render(); } }); }
                    line.append(&name); line.append(&state); line.append(&toggle); line.append(&remove); row.set_child(Some(&line)); list.append(&row);
                }
            })
        };
        *renderer.borrow_mut() = Some(Box::new({ let refresh=refresh.clone(); move || refresh() }));
        // Rebuilt after each action to keep rows and indices in sync.
        let refresh_fn = refresh.clone();
        add_button.connect_clicked({ let sites=sites.clone(); let input=input.clone(); let refresh=refresh.clone(); move |_| {
            let domain=input.text().trim().to_ascii_lowercase().trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/').to_string();
            if domain.is_empty() || domain.contains('/') || !domain.contains('.') { return; }
            if !sites.borrow().iter().any(|s| s.domain == domain) { sites.borrow_mut().push(Site {domain, blocked:true}); }
            input.set_text(""); refresh();
        }});
        apply.connect_clicked({ let sites=sites.clone(); let status=status.clone(); let refresh=refresh.clone(); move |_| {
            match hosts::apply(&sites.borrow()) { Ok(()) => { let _=store::save(&sites.borrow()); status.set_text("File hosts aggiornato."); }, Err(e) => status.set_text(&format!("Errore: {e}")) }
            refresh();
        }});
        // Persist intended state so unblocked sites survive restart.
        refresh_fn(); let _=store::save(&sites.borrow());
        let header = HeaderBar::new();
        header.set_show_title_buttons(false);
        header.set_title_widget(Some(&Label::new(Some("Host Manager"))));
        let close = Button::with_label("×");
        close.add_css_class("close-dot");
        header.pack_end(&close);
        let window = ApplicationWindow::builder().application(app).title("Host Manager").default_width(520).default_height(520).child(&root).build();
        window.set_titlebar(Some(&header));
        close.connect_clicked({ let window=window.clone(); move |_| window.close() });
        window.present();
    });
    app.run();
}

