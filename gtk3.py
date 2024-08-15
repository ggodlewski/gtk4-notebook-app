#!/usr/bin/env python3

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

class BrowserWindow(Gtk.Window):
    def __init__(self):
        super().__init__(title="GTK3 Tabs Example")
        self.set_default_size(800, 600)

        # Create a Gtk.Box to hold the components
        self.box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL)
        self.add(self.box)

        # Create a Gtk.Notebook for the tabs
        self.notebook = Gtk.Notebook()
        self.notebook.set_group_name("aaa")
        self.notebook.set_scrollable(True)
        self.notebook.connect("create-window", self.on_create_window)
        self.notebook.connect("page-removed", self.notebook_page_removed, self)
        self.box.pack_start(self.notebook, True, True, 0)

    def notebook_page_removed (self, notebook, child, page, window):
        #destroy the sub window after the notebook is empty
        if notebook.get_n_pages() == 0:
            window.destroy()

    def new_tab(self, label_text="New Tab"):
        # Create a label to display in the tab
        label = Gtk.Label(label=label_text)

        # Create a label for the tab title
        tab_label = Gtk.Label(label="New Tab")

        # Add the label to the notebook with the tab_box as the tab label
        self.notebook.append_page(label, tab_label)
        self.notebook.set_tab_reorderable(label, True)
        self.notebook.set_tab_detachable(label, True)
        self.notebook.set_current_page(-1)

    def on_close_tab(self, button):
        page_num = self.notebook.page_num(button.get_parent().get_parent())
        if page_num != -1:
            self.notebook.remove_page(page_num)

    def on_create_window(self, notebook, page, x, y):
        new_window = BrowserWindow()
        new_window.move(x, y)
        new_window.show_all()
        return new_window.notebook

def main():
    app = Gtk.Application(application_id='com.example.GtkTabs')
    app.connect('activate', on_activate)
    app.run(None)

def on_activate(app):
    win = BrowserWindow()
    win.set_application(app)
    # Add multiple tabs at startup
    win.new_tab("Welcome to the GTK3 Tabs Example!")
    win.new_tab("Second Tab Content")
    win.new_tab("Third Tab Content")
    win.show_all()

if __name__ == "__main__":
    main()
