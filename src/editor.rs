use dioxus::prelude::*;
use dioxus_desktop::use_eval;
use crate::constants::LOREM_IPSUM;
use crate::enums::{Direction, Glyph};
use crate::editor_state::EditorState;
use keyboard_types::{Code, Key, Modifiers};


macro_rules! code_events {
    ($event:ident, $editor:ident as $alias:ident,
     $(
         $type:ident => [
             $(for $match:pat => $fun:expr),+
         ]

     ),+) => {
        $(
        match $event.data.$type() {
            $(
                $match => {
                    $editor.with_mut(|$alias| { $fun });
                    $event.stop_propagation();
                    return;
                }
            )+
            _ => (),
        })+
    }
}


pub fn Editor(cx: Scope) -> Element {

    let editor_state = use_ref(cx, || EditorState::from(LOREM_IPSUM));
    let line_style = r#"
min-height: 15px;
outline: none;
"#;

    let editor_style = r#"
flex: 1;
outline: none;
font-family: Courier; display: grid;
padding: 3px; border: 1px solid black;
margin: 5px;
grid-column-gap: 5px;
grid-auto-rows: fit-content;

grid-template-columns: minmax(40px,max-content) auto;
grid-template_areas: "l c";
"#;

    let handle_clicks = move |event: Event<MouseData>| {
        let runner = use_eval(cx);
        let coords = event.page_coordinates();
        let x = coords.x;
        let y = coords.y;

        to_owned![runner, editor_state];
        let script = format!(r#"
ran = document.caretRangeFromPoint({x},{y});
el = ran.startContainer;
par = el.parentElement;
return [
parseInt(par.getAttribute('line')) || -1,
[...par.childNodes].indexOf(el)+1,
];

"#);
        cx.spawn(async move {
            if let Ok(res) = runner(script).await {

                let line = res.get(0).unwrap().as_i64().unwrap();
                let cursor = res.get(1).unwrap().as_i64().unwrap();

                if line < 0 { return }

                editor_state.with_mut(|e| {e.set_cursor(line as usize, cursor as usize) });

                println!("{line}x{cursor}");

            }
        });

    };

    let handle_global_keys = move |event: Event<KeyboardData>| {
        if event.modifiers().contains(Modifiers::META) && event.code() == Code::KeyA {
            editor_state.with_mut(|e| e.insert_pill("C-A"));
            event.stop_propagation();
            return;
        }
        code_events![
            event, editor_state as e,

            code => [

                for Code::F1 => e.insert_pill("F1"),
                for Code::F2 => e.insert_pill("F2"),
                for Code::F3 => e.insert_pill("F3"),
                for Code::F4 => e.insert_pill("F4"),

                for Code::Delete => e.delete(Direction::Forward),
                for Code::Backspace => e.delete(Direction::Backward),
                for Code::Space => e.insert_char(char::from_u32(0x00A0).unwrap()),

                for Code::ArrowUp => e.go_to_line(Direction::Backward),
                for Code::ArrowDown => e.go_to_line(Direction::Forward),
                for Code::ArrowRight => e.move_cursor(Direction::Forward),
                for Code::ArrowLeft => e.move_cursor(Direction::Backward),
                for Code::Enter => e.next_line_or_new()
            ],
            key => [
                for Key::Character(n) => e.insert(&n)
            ]
        ];
    };

    let (current_line, current_position) = editor_state.with(|e| {(e.current_line, e.cursor_position)});

    cx.render(rsx! {
        div {
            style: "{editor_style}",
            tabindex: 0,
            autofocus: true,
            onkeydown: handle_global_keys,
            editor_state.read().iter().map(|(line_number, line)| {
                let current = current_line == line_number;
                let background = if current { "pink" } else { "grey" };
                rsx! {
                    div{
                        style: "padding-right: 5px; background: {background}; text-align: right;",
                        "{line_number}:"
                    },
                    div {
                        style: "{line_style}",
                        id: "L{line_number}",
                        "line": "{line_number}",
                        onmousedown: handle_clicks,

                        for renderable in line.as_vec().iter() {
                            match renderable {
                                Glyph::Char(_) => rsx!{ "{renderable}" },
                                Glyph::Cursor => rsx!{
                                    span {
                                        style: "position: relative; left: -0px;",
                                    svg {
                                        "viewBox": "0 0 12 16",
                                        height: "14pt",
                                        style: "margin-bottom: -4px;",
                                        path {
                                            d: "M2 1h8v14H2z"
                                        }
                                    }
                                    }
                                },
                                Glyph::HTMLNode(value) => rsx!{
                                    span {
                                        dangerous_inner_html: "{value}"
                                    }
                                },
                                _ => rsx!{""}
                            }
                        },
                        if line.as_vec().is_empty() {
                            rsx!("")
                        }

                    },
                }})
        },
        div {
            "Line: {current_line} Position: {current_position}"
        }
    })
}
