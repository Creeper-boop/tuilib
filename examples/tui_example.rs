use charflow;
use charflow::colors::{GREY, ORANGE};
use charflow::complex_elements::element_tree::{
    Element, ElementTree, Folder, Icon, Part, CLOSED_FOLDER, OPEN_FOLDER, SIMPLE_SET,
};
use charflow::elements;
use charflow::input::Input;
use charflow::lines::LINES_HEAVY;
use charflow::tui::{StaticTUI, TUI};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn folder(name: String, is_open: bool, parts: Option<Vec<Part>>) -> Folder {
    Folder {
        name,
        icon_closed_id: "closed".to_string(),
        icon_open_id: "open".to_string(),
        is_open,
        children: if let Some(children) = parts {
            children
        } else {
            Vec::new()
        },
    }
}

fn element(name: String) -> Element {
    Element {
        name,
        icon_id: "element".to_string(),
    }
}

fn main() {
    let tui = StaticTUI::new();

    let mut icon_map = HashMap::new();
    icon_map.insert("closed".to_string(), CLOSED_FOLDER);
    icon_map.insert("open".to_string(), OPEN_FOLDER);
    icon_map.insert(
        "element".to_string(),
        Icon {
            color: None,
            char: '#',
        },
    );

    let element_tree = ElementTree {
        x: 10,
        y: 5,
        z: 0,
        width: 20,
        height: 15,
        icon_map,
        line_set: SIMPLE_SET,
        elements: folder(
            "main".to_string(),
            true,
            Some(vec![
                Part::Folder(folder("images".to_string(), false, None)),
                Part::Element(element("text_file.txt".to_string())),
                Part::Folder(folder(
                    "folder".to_string(),
                    true,
                    Some(vec![
                        Part::Element(element("text_file.txt".to_string())),
                        Part::Element(element("text_file.txt".to_string())),
                    ]),
                )),
                Part::Element(element("text_file.txt".to_string())),
                Part::Folder(folder(
                    "folder".to_string(),
                    true,
                    Some(vec![
                        Part::Element(element("text_file.txt".to_string())),
                        Part::Element(element("text_file.txt".to_string())),
                        Part::Element(element("text_file.txt".to_string())),
                        Part::Element(element("text_file.txt".to_string())),
                    ]),
                )),
            ]),
        ),
        element_color: Some(ORANGE),
        line_color: None,
        bg_color: Some(GREY),
        enabled: true,
    };

    tui.lock()
        .unwrap()
        .elements
        .push(Arc::new(Mutex::new(element_tree)));

    let text_box = elements::TextBox {
        x: 35,
        y: 5,
        z: 0,
        width: 20,
        height: 15,
        fg_color: Some(ORANGE),
        bg_color: Some(GREY),
        enabled: true,
        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent sed lorem sit amet elit ullamcorper gravida ut vitae dolor. Cras.".to_string()
    };

    tui.lock()
        .unwrap()
        .elements
        .push(Arc::new(Mutex::new(text_box)));

    let mut input = Input::new(true);
    loop {
        tui.lock().unwrap().update();
        input.update(Duration::from_millis(10));
    }
}
