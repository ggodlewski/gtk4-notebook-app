use std::sync::Arc;
use gtk::{prelude::*, Notebook};
use gtk::{Application, ApplicationWindow};
use gtk::gio::{MenuModel, Menu, MenuItem, ActionEntry, SimpleActionGroup};

#[derive(Default, Debug)]
pub struct GtkUi {
    app: Application
}

impl GtkUi {
    pub fn new() -> Self {
        GtkUi {
            app: Application::builder().application_id("org.gtk_rs.HelloNotebookApp").build()
        }
    }

    pub fn file_actions(self: Arc<Self>, notebook: Arc<gtk::Notebook>) -> SimpleActionGroup {
        let file_actions = SimpleActionGroup::new();
        file_actions.add_action_entries([
            ActionEntry::builder("new")
                .activate(move |_group, _action, _variant| {
                    self.clone().create_tab(&notebook, "new tab");
                })
                .build(),
        ]);
    
        file_actions
    }

    fn create_window(self: Arc<Self>) -> Option<gtk::Notebook> {
        let notebook = gtk::Notebook::new();
        notebook.set_group_name(Some("0"));

        let file_menu = Menu::new();
        file_menu.insert_item(0, &MenuItem::new(Some("New"), Some("file.new")));

        let menu = Menu::new();
        menu.insert_submenu(0, Some("File"), &file_menu);
        let menu_model: MenuModel = menu.into();

        let menubar = gtk::PopoverMenuBar::from_model(Some(&menu_model));

        let window_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        window_box.append(&menubar);
        window_box.append(&notebook);

        let window = ApplicationWindow::builder()
            .application(&self.app)
            .default_width(640)
            .default_height(480)
            .title("My GTK App")
            .child(&window_box)
            .show_menubar(true)
            .build();

        let file_actions = self.clone().file_actions(Arc::new(notebook.clone()));
        window.insert_action_group("file", Some(&file_actions));
    
        {
            let window = window.clone();
            notebook.connect_page_removed(move |notebook: &gtk::Notebook, _widget, _no| {
                println!("connect_page_removed {} pages", notebook.n_pages());
                if notebook.n_pages() == 0 {
                    window.destroy();
                }
            });
        }

        notebook.connect_create_window(move |_notebook, widget| {
            println!("connect_create_window {:#?}", widget);
            self.clone().create_window()
        });

        window.connect_destroy(|app_win: &ApplicationWindow| {
            println!("connect_destroy {:#?}", app_win);
        });
    
        window.show();

        Some(notebook)
    }

    fn create_tab(self: Arc<Self>, notebook: &Notebook, label: &str) {
        let tab_label = gtk::Label::new(Some(label));
        let tab = gtk::Box::new(gtk::Orientation::Vertical, 0);
        
        let tab_content = gtk::Label::new(Some("TODO"));
        tab.append(&tab_content);

        notebook.append_page(&tab, Some(&tab_label));
        notebook.set_tab_reorderable(&tab, true);
        notebook.set_tab_detachable(&tab, true);
    }

    fn build_ui(self: Arc<Self>, _app: &Application) {
        let notebook = self.clone().create_window();
        if let Some(notebook) = notebook {
            for i in 0..2 {
                self.clone().create_tab(&notebook, format!("Tab {}", i).as_str());
            }
        }
    }

    pub fn start(self: Arc<GtkUi>) -> i32 {
        let app = self.app.clone();
        app.clone().connect_activate(move |app| self.clone().build_ui(app));
        let exit_code = app.run();
        exit_code.value()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui: Arc<GtkUi> = Arc::new(GtkUi::new());
    std::process::exit(ui.start());
}
