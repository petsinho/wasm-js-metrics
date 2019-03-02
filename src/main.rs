#[macro_use]
extern crate stdweb;
extern crate time;

use std::rc::Rc;

use time::{Timespec, Tm};

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, HtmlElement};

use stdweb::web::event::{ClickEvent, KeyPressEvent, MouseMoveEvent};

use stdweb::web::html_element::InputElement;

mod utils;

pub struct EventInfo {
    pub element_name: String,
    pub event_type: String,
    created_at: Option<i64>,
    completed_at: Option<i64>,
}

pub struct Coords {
    pub x: u32,
    pub y: u32,
}

impl EventInfo {
    fn created_at(&self) -> Option<Tm> {
        match self.created_at {
            None => None,
            Some(t) => Some(time::at(Timespec::new(t, 0))),
        }
    }

    fn completed_at(&self) -> Option<Tm> {
        match self.completed_at {
            None => None,
            Some(t) => Some(time::at(Timespec::new(t, 0))),
        }
    }
}

fn main() {
    println!("started...");
    stdweb::initialize();

    let output_div: HtmlElement = document()
        .query_selector(".output")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let output_msg = Rc::new(move |msg: &str| {
        let elem = document().create_element("p").unwrap();
        elem.set_text_content(msg);
        if let Some(child) = output_div.first_child() {
            output_div.insert_before(&elem, &child).unwrap();
        } else {
            output_div.append_child(&elem);
        }
    });
    output_msg("> Starting...");

    let body: HtmlElement = document()
        .query_selector(".bod")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    body.add_event_listener(enclose!( () move |event: ClickEvent| {
        let target = event.target().unwrap();
        output_msg("clicked body ");
        js!{
            console.log("target", event.target)
        }
        // if event.key() == "Enter" {
        //     //TODO: save event
        // }
    }));

    body.add_event_listener(enclose!( () move |event: MouseMoveEvent| {
        let posx = event.client_x();
        let posy = event.client_y();

        js!{
            console.log("mouse moved @ {}, {} ", @{posx}, @{posy})
        }
    }));

    let text_entry: InputElement = document()
        .query_selector(".form input")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    text_entry.add_event_listener(enclose!( (text_entry) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let text: String = text_entry.raw_value();
            if text.is_empty() == false {
                text_entry.set_raw_value("");
            }
        }
    }));

    stdweb::event_loop();
}
