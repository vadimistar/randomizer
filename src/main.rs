#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate rand;

use nwg::{ControlHandle, ListBox};
use rand::Rng;
use std::{cell::Ref, rc::Rc};

fn get_random_item(v: Ref<Vec<String>>) -> String {
    let peek: usize = rand::thread_rng().gen_range(0..v.len());
    v.get(peek).unwrap().clone()
}

fn on_window_close() {
    nwg::stop_thread_dispatch();
}

fn on_button_add_click(text_input: &nwg::TextInput, listbox: &ListBox<String>) {
    let new_item = text_input.text();
    listbox.push(new_item);
    text_input.set_text("");
}

fn on_button_remove_click(listbox: &ListBox<String>) {
    if let Some(selected) = listbox.selection() {
        listbox.remove(selected);
    }
}

fn on_button_peek_click(listbox: &ListBox<String>, parent: ControlHandle) {
    if listbox.len() == 0 {
        nwg::modal_error_message(&parent, "Error", "0 options in the list");
    } else {
        nwg::modal_info_message(
            &parent,
            "Pick",
            &get_random_item(listbox.collection()),
        );
    }
}

fn build_window() -> nwg::Window {
    let mut window = Default::default();

    nwg::Window::builder()
        .size((500, 300))
        .title("Randomizer")
        .build(&mut window)
        .expect("Failed to create a window");

    window
}

fn build_list_box(parent: ControlHandle) -> nwg::ListBox<String> {
    let mut list_box = nwg::ListBox::<String>::default();

    nwg::ListBox::builder()
        .parent(parent)
        .build(&mut list_box)
        .expect("Failed to create a listbox");

    list_box
}

fn build_text_input(parent: ControlHandle) -> nwg::TextInput {
    let mut text_input = Default::default();

    nwg::TextInput::builder()
        .parent(parent)
        .build(&mut text_input)
        .unwrap();

    text_input
} 

fn build_button(parent: ControlHandle, text: &str) -> nwg::Button {
    let mut button = Default::default();

    nwg::Button::builder()
        .text(text)
        .parent(parent)
        .build(&mut button)
        .unwrap();

    button
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI.");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");
    
    let window = build_window();
    let list_box = build_list_box(window.handle);
    let text_input = build_text_input(window.handle);    

    let button_add = build_button(window.handle, "Add");
    let button_remove = build_button(window.handle, "Remove");
    let button_pick = build_button(window.handle, "Pick");

    let layout = Default::default();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child_item(nwg::GridLayoutItem::new(&list_box, 0, 0, 3, 5))
        .child_item(nwg::GridLayoutItem::new(&text_input, 0, 5, 3, 1))
        .child(0, 6, &button_add)
        .child(1, 6, &button_remove)
        .child(2, 6, &button_pick)
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => {
                if &handle == &events_window as &nwg::Window {
                    on_window_close()
                }
            }
            E::OnButtonClick => match &handle {
                _h if _h == &button_add => 
                    on_button_add_click(&text_input, &list_box),
                _h if _h == &button_remove => 
                    on_button_remove_click(&list_box),
                _h if _h == &button_pick => 
                    on_button_peek_click(&list_box, events_window.handle),
                _ => unreachable!(),
            },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
