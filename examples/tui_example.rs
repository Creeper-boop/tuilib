use ctiui;
use ctiui::colors::{GREY, ORANGE};
use ctiui::element_tree::{ElementTree, Folder, Part, CLOSED_FOLDER, OPEN_FOLDER, SIMPLE_SET};
use ctiui::elements;
use ctiui::lines::LINES_HEAVY;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let mut tui = ctiui::tui::TUI::new();
    tui.elements.push(Arc::new(Mutex::new(elements::Text {
        x: 5,
        y: 5,
        z: 1,
        text_color: None,
        bg_color: None,
        enabled: true,
        text: "This is most definitely not a textbox".to_string(),
    })));

    tui.elements.push(Arc::new(Mutex::new(elements::Box {
        x: 3,
        y: 3,
        z: 0,
        width: 20,
        height: 20,
        line_color: Option::from(ORANGE),
        bg_color: Option::from(GREY),
        enabled: true,
        line_set: LINES_HEAVY,
    })));

    tui.elements.push(Arc::new(Mutex::new(elements::Box {
        x: 5,
        y: 5,
        z: 2,
        width: 3,
        height: 3,
        line_color: None,
        bg_color: None,
        enabled: true,
        line_set: LINES_HEAVY,
    })));

    tui.elements.push(Arc::new(Mutex::new(ElementTree {
        x: 40,
        y: 1,
        z: 3,
        width: 0,
        height: 8,
        icon_map: HashMap::from([
            ("closed".to_string(), CLOSED_FOLDER),
            ("open".to_string(), OPEN_FOLDER),
        ]),
        line_set: SIMPLE_SET,
        elements: Folder {
            name: "".to_string(),
            icon_closed_id: "closed".to_string(),
            icon_open_id: "open".to_string(),
            is_open: true,
            children: vec![
                Part::Folder(Folder {
                    name: "asd".to_string(),
                    icon_closed_id: "closed".to_string(),
                    icon_open_id: "open".to_string(),
                    is_open: true,
                    children: vec![Part::Folder(Folder {
                        name: "asd".to_string(),
                        icon_closed_id: "closed".to_string(),
                        icon_open_id: "open".to_string(),
                        is_open: true,
                        children: vec![Part::Folder(Folder {
                            name: "asd".to_string(),
                            icon_closed_id: "closed".to_string(),
                            icon_open_id: "open".to_string(),
                            is_open: true,
                            children: vec![Part::Folder(Folder {
                                name: "asd".to_string(),
                                icon_closed_id: "closed".to_string(),
                                icon_open_id: "open".to_string(),
                                is_open: true,
                                children: vec![Part::Folder(Folder {
                                    name: "asd".to_string(),
                                    icon_closed_id: "closed".to_string(),
                                    icon_open_id: "open".to_string(),
                                    is_open: true,
                                    children: vec![Part::Folder(Folder {
                                        name: "asd".to_string(),
                                        icon_closed_id: "closed".to_string(),
                                        icon_open_id: "open".to_string(),
                                        is_open: true,
                                        children: vec![Part::Folder(Folder {
                                            name: "asd".to_string(),
                                            icon_closed_id: "closed".to_string(),
                                            icon_open_id: "open".to_string(),
                                            is_open: true,
                                            children: vec![Part::Folder(Folder {
                                                name: "asd".to_string(),
                                                icon_closed_id: "closed".to_string(),
                                                icon_open_id: "open".to_string(),
                                                is_open: true,
                                                children: vec![Part::Folder(Folder {
                                                    name: "asd".to_string(),
                                                    icon_closed_id: "closed".to_string(),
                                                    icon_open_id: "open".to_string(),
                                                    is_open: true,
                                                    children: vec![Part::Folder(Folder {
                                                        name: "asd".to_string(),
                                                        icon_closed_id: "closed".to_string(),
                                                        icon_open_id: "open".to_string(),
                                                        is_open: true,
                                                        children: vec![],
                                                    })],
                                                })],
                                            })],
                                        })],
                                    })],
                                })],
                            })],
                        })],
                    })],
                }),
                Part::Folder(Folder {
                    name: "asd".to_string(),
                    icon_closed_id: "closed".to_string(),
                    icon_open_id: "open".to_string(),
                    is_open: false,
                    children: vec![],
                }),
            ],
        },
        element_color: None,
        line_color: None,
        bg_color: None,
        enabled: true,
    })));

    let mut input = ctiui::input::Input::new(true);
    loop {
        tui.update();
        input.update(Duration::from_millis(10));
    }
}
