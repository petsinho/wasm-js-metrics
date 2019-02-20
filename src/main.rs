#[macro_use]
extern crate stdweb;

use std::rc::Rc;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    HtmlElement,
    document,
};

use stdweb::web::event::{
    KeyPressEvent,
    ClickEvent,
};

use stdweb::web::html_element::InputElement;

mod utils;

fn main() {
    println!("started...");
    stdweb::initialize();

    let output_div: HtmlElement = document().query_selector( ".output" ).unwrap().unwrap().try_into().unwrap();
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



  let body: HtmlElement = document().query_selector( ".bod" ).unwrap().unwrap().try_into().unwrap();
    body.add_event_listener( enclose!( () move |event: ClickEvent| {
        let target = event.target().unwrap();
        output_msg("clicked body ");
        js!{
            console.log("target", event.target)
        }
        // if event.key() == "Enter" {
        //     //TODO: save event            
        // }
    }));

    let text_entry: InputElement = document().query_selector( ".form input" ).unwrap().unwrap().try_into().unwrap();
    text_entry.add_event_listener( enclose!( (text_entry) move |event: KeyPressEvent| {
        if event.key() == "Enter" {
            println!("enter bitch");
            event.prevent_default();

            let text: String = text_entry.raw_value();
            if text.is_empty() == false {
                text_entry.set_raw_value("");
            }
        }
    }));

    stdweb::event_loop();
}
