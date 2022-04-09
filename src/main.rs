#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use rand::{prelude::ThreadRng, Rng};
use std::{cell::Ref, rc::Rc, sync::Mutex};

fn get_random_item(v: Ref<Vec<String>>) -> String {
    let peek: usize = rand::thread_rng().gen_range(0..v.len());
    v.get(peek).unwrap().clone()
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI.");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");

    let mut window = Default::default();

    nwg::Window::builder()
        .size((500, 300))
        .title("Randomizer")
        .build(&mut window)
        .expect("Failed to create a window");

    let mut listbox = nwg::ListBox::<String>::default();

    nwg::ListBox::builder()
        .parent(&window)
        .build(&mut listbox)
        .expect("Failed to create a listbox");

    let mut text_input = Default::default();

    nwg::TextInput::builder()
        .parent(&window)
        .build(&mut text_input)
        .unwrap();

    let mut button_add = Default::default();
    let mut button_remove = Default::default();
    let mut button_pick = Default::default();

    nwg::Button::builder()
        .text("Add")
        .parent(&window)
        .build(&mut button_add)
        .unwrap();

    nwg::Button::builder()
        .text("Remove")
        .parent(&window)
        .build(&mut button_remove)
        .unwrap();

    nwg::Button::builder()
        .text("Pick")
        .parent(&window)
        .build(&mut button_pick)
        .unwrap();

    let layout = Default::default();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child_item(nwg::GridLayoutItem::new(&listbox, 0, 0, 3, 5))
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
                    nwg::stop_thread_dispatch();
                }
            }
            E::OnButtonClick => match &handle {
                _h if _h == &button_add => {
                    let new_item = text_input.text();
                    if listbox.len() > usize::MAX {
                        return;
                    }
                    listbox.push(new_item);
                    text_input.set_text("");
                }
                _h if _h == &button_remove => {
                    if let Some(selected) = listbox.selection() {
                        listbox.remove(selected);
                    }
                }
                _h if _h == &button_pick => {
                    if listbox.len() == 0 {
                        nwg::modal_error_message(
                            &events_window.handle,
                            "Error",
                            "0 options in the list",
                        );
                        return;
                    }
                    nwg::modal_info_message(
                        &events_window.handle,
                        "Pick",
                        &get_random_item(listbox.collection()),
                    );
                }
                _ => unreachable!(),
            },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
