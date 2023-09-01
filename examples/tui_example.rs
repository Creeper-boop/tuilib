use charflow;
use charflow::colors::{GREY, ORANGE, ORANGE_50, YELLOW};
use charflow::complex_elements::element_tree::{
    Element, ElementTree, Folder, Icon, Part, CLOSED_FOLDER, OPEN_FOLDER, SIMPLE_SET,
};
use charflow::elements;
use charflow::input::{Event, Input};
use charflow::input_callbacks::{ENTER, MOUSE_LEFT_PRESS, UPPERCASE_J, UPPERCASE_K};
use charflow::lines::LINES_HEAVY;
use charflow::tui::{ReactiveTUI, TUI};
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
    let (tui, tui_key_observer, tui_mouse_observer) = ReactiveTUI::new(UPPERCASE_J, UPPERCASE_K);

    // Start of element tree example.
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
        visible: true,
    };

    tui.lock()
        .unwrap()
        .elements
        .push(Arc::new(Mutex::new(element_tree)));
    // End of element tree example.

    // Start of text box example.
    let text_box = elements::TextBox {
        x: 35,
        y: 5,
        z: 0,
        width: 20,
        height: 15,
        text_color: Some(ORANGE),
        bg_color: Some(GREY),
        visible: true,
        text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent sed lorem sit amet elit ullamcorper gravida ut vitae dolor. Cras.".to_string()
    };

    tui.lock()
        .unwrap()
        .elements
        .push(Arc::new(Mutex::new(text_box)));
    // End of text box example.

    // Start of Button and box example.
    let line_box = elements::Box {
        x: 61,
        y: 5,
        z: 0,
        width: 13,
        height: 3,
        line_color: Some(YELLOW),
        bg_color: Some(GREY),
        visible: true,
        line_set: LINES_HEAVY,
    };

    let line_box_mutex = Arc::new(Mutex::new(line_box));

    tui.lock().unwrap().elements.push(line_box_mutex.clone());

    let button = elements::Button {
        x: 61,
        y: 9,
        z: 0,
        width: 13,
        height: 2,
        selected: false,
        action: Arc::new(move |data: Event| match data {
            Event::KeyEvent(key_event) => match key_event.code {
                ENTER => {
                    let mut line_box_mutex_lock = line_box_mutex.lock().unwrap();
                    if line_box_mutex_lock.bg_color == Some(GREY) {
                        line_box_mutex_lock.bg_color = Some(ORANGE_50)
                    } else {
                        line_box_mutex_lock.bg_color = Some(GREY)
                    }
                }
                _ => {}
            },
            Event::MouseEvent(mouse_event) => match mouse_event.code {
                MOUSE_LEFT_PRESS => {
                    let mut line_box_mutex_lock = line_box_mutex.lock().unwrap();
                    if line_box_mutex_lock.bg_color == Some(GREY) {
                        line_box_mutex_lock.bg_color = Some(ORANGE_50)
                    } else {
                        line_box_mutex_lock.bg_color = Some(GREY)
                    }
                }
                _ => {}
            },
        }),
        text_color: None,
        bg_color: Some(GREY),
        selected_text_color: Some(YELLOW),
        selected_bg_color: Some(ORANGE_50),
        enabled: true,
        visible: true,
        text: "This is a button.".to_string(),
    };

    let button_mutex = Arc::new(Mutex::new(button));

    tui.lock()
        .unwrap()
        .reactive_elements
        .push(button_mutex.clone());

    tui.lock().unwrap().elements.push(button_mutex);

    let second_button = elements::Button {
        x: 61,
        y: 12,
        z: 0,
        width: 13,
        height: 2,
        selected: false,
        text_color: None,
        bg_color: Some(GREY),
        selected_text_color: Some(YELLOW),
        selected_bg_color: Some(ORANGE_50),
        enabled: true,
        visible: true,
        action: Arc::new(move |_: Event| {}),
        text: "This is also a button!".to_string(),
    };

    let second_button_mutex = Arc::new(Mutex::new(second_button));

    tui.lock()
        .unwrap()
        .reactive_elements
        .push(second_button_mutex.clone());

    tui.lock().unwrap().elements.push(second_button_mutex);

    // End of Button and box example.

    // Start of text example.
    let text = elements::Text {
        x: 57,
        y: 5,
        z: 0,
        text_color: Some(ORANGE),
        bg_color: Some(GREY),
        visible: true,
        text: "-0\n-1\n-2\n-3\n-4\n-5\n-6\n-7\n-8\n-9\n10\n11\n12\n13\n14".to_string(),
    };

    tui.lock()
        .unwrap()
        .elements
        .push(Arc::new(Mutex::new(text)));
    // End of text example.

    let mut input = Input::new(true);

    input.key_observers.lock().unwrap().push(tui_key_observer);
    input
        .mouse_observers
        .lock()
        .unwrap()
        .push(tui_mouse_observer);

    loop {
        tui.lock().unwrap().update();
        input.update(Duration::from_millis(10));
    }
}
