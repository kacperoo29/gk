mod model;

use gloo_events::EventListener;
use model::shape::{ShapeState, ShapeStorage, ShapeType};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::*;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Msg {
    ShapeChanged { shape_type: ShapeType },
    ModeChanged { mode: Mode },
    MouseClicked { x: f64, y: f64 },
    MouseMove { x: f64, y: f64 },
    MouseUp { x: f64, y: f64 },
    MouseDown { x: f64, y: f64 },
    ClearScreen,
    NewShape,
    SubmitShape,
    ValueChanged { key: String, value: String },
    SaveToJson,
    LoadFromJson { value: String },
    JsonChanged { value: String },
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Draw,
    Select,
    Resize,
    Move,
}

struct App {
    shape_type: ShapeType,
    shape_storage: ShapeStorage,
    mode: Mode,
    is_dragging: bool,
    last_cursor_pos: (f64, f64),
    resize_anchor: (f64, f64),
    json: String
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            shape_type: ShapeType::Line,
            shape_storage: ShapeStorage::new(),
            mode: Mode::Draw,
            is_dragging: false,
            last_cursor_pos: (0.0, 0.0),
            resize_anchor: (0.0, 0.0),
            json: String::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let canvas_mouseclick_callback =
            ctx.link().callback(|event: MouseEvent| Msg::MouseClicked {
                x: event.offset_x() as f64,
                y: event.offset_y() as f64,
            });
        let canvas_mousemove_callback = ctx.link().callback(|event: MouseEvent| Msg::MouseMove {
            x: event.offset_x() as f64,
            y: event.offset_y() as f64,
        });
        let canvas_mouseup_callback = ctx.link().callback(|event: MouseEvent| Msg::MouseUp {
            x: event.offset_x() as f64,
            y: event.offset_y() as f64,
        });
        let canvas_mousedown_callback = ctx.link().callback(|event: MouseEvent| Msg::MouseDown {
            x: event.offset_x() as f64,
            y: event.offset_y() as f64,
        });
        let line_callback = ctx.link().callback(|_| Msg::ShapeChanged {
            shape_type: ShapeType::Line,
        });
        let rectangle_callback = ctx.link().callback(|_| Msg::ShapeChanged {
            shape_type: ShapeType::Rectangle,
        });
        let circle_callback = ctx.link().callback(|_| Msg::ShapeChanged {
            shape_type: ShapeType::Circle,
        });
        let draw_mode_callback = ctx
            .link()
            .callback(|_| Msg::ModeChanged { mode: Mode::Draw });
        let select_mode_callback = ctx
            .link()
            .callback(|_| Msg::ModeChanged { mode: Mode::Select });
        let resize_mode_callback = ctx
            .link()
            .callback(|_| Msg::ModeChanged { mode: Mode::Resize });
        let move_mode_callback = ctx
            .link()
            .callback(|_| Msg::ModeChanged { mode: Mode::Move });
        let value_changed_callback = ctx.link().callback(move |event: InputEvent| {
            let target = event.target().unwrap();
            let target: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            let value = target.value();
            let key = target.id();
            Msg::ValueChanged { key, value }
        });

        let prop_str = match self.shape_storage.get_highlighted() {
            Some(shape) => shape.get_prop_str(),
            None => String::new(),
        };
        let prop_list = prop_str.split("\n").filter(|str| !str.is_empty());

        let selected_shape = self.shape_storage.get_selected();
        let file_cb = ctx.link().callback(|value: String| Msg::LoadFromJson { value });
        html! {
            <div id="container">
                <div style="width: 100%;height: 620px; margin: 0">
                    <canvas
                        id="canvas"
                        width="800"
                        height="600"
                        onclick={canvas_mouseclick_callback}
                        onmousedown={canvas_mousedown_callback}
                        onmouseup={canvas_mouseup_callback}
                        onmousemove={canvas_mousemove_callback}
                        style="border: 1px solid black;float: left" />
                    <div style="float: left; margin-left: 20px">
                        <h2 style="margin-top: 0">{format!("Current shape type: {:?}", self.shape_type)}</h2>
                        <h2 style="margin-top: 0">{format!("Current mouse mode: {:?}", self.mode)}</h2>
                        <ul>
                        {prop_list.map(|prop| {
                            html! {
                                <li>{prop}</li>
                            }
                        }).collect::<Html>()}
                        </ul>
                        if let Some(shape) = selected_shape {
                            <div>
                                <h2>{"Selected shape"}</h2>
                                <h4>{format!("Shape type: {:?}", shape.get_type())}</h4>
                                {shape.get_props().iter().map(|prop| {
                                    html! {
                                        <div>
                                            <label>{format!("{}: ", prop.0)}</label>
                                            <input
                                                id={prop.0.clone()}
                                                type="number"
                                                oninput={value_changed_callback.clone()}
                                                value={prop.1.clone()} />
                                        </div>
                                    }
                                }).collect::<Html>()}
                                if shape.get_state() == ShapeState::New {
                                    <button onclick={ctx.link().callback(|_| Msg::SubmitShape)}>{ "Create shape" }</button>
                                }
                            </div>
                        }
                    </div>
                </div>
                <label>{"Mode"}</label>
                <div>
                    <button onclick={draw_mode_callback}>{"Draw"}</button>
                    <button onclick={select_mode_callback}>{"Select"}</button>
                    <button onclick={resize_mode_callback}>{"Resize"}</button>
                    <button onclick={move_mode_callback}>{"Move"}</button>
                </div>
                <label>{"Shape"}</label>
                <div>
                    <button onclick={line_callback}>{"Line"}</button>
                    <button onclick={rectangle_callback}>{"Rectangle"}</button>
                    <button onclick={circle_callback}>{"Circle"}</button>
                </div>
                <label>{"Command"}</label>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::ClearScreen)}>{"Clear"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::NewShape)}>{"New"}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SaveToJson)}>{"Save"}</button>
                    // <button onclick={ctx.link().callback(|_| Msg::LoadFromJson)}>{"Load"}</button>
                    <input type="file" onchange={ctx.link().callback(move |event: Event| {
                        let file_cb = file_cb.clone();
                        let target = event.target().unwrap();
                        let target: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                        let file = target.files().unwrap().get(0).unwrap();
                        let file_reader = web_sys::FileReader::new().unwrap();
                        file_reader.read_as_text(&file).unwrap();
                        log::info!("file: {:?}", file);
                        let listener = EventListener::new(&file_reader, "load", move |event| {
                            log::info!("event: {:?}", event);
                            let target = event.target().unwrap();
                            let target: web_sys::FileReader = target.dyn_into().unwrap();
                            let result = target.result().unwrap();
                            let result: String = result.as_string().unwrap();
                            file_cb.emit(result);
                        });
                        listener.forget();

                        Msg::None
                    })} />
                </div>
                <div>
                    <span>{"JSON: "}</span>
                    <textarea id="json"
                        width="800"
                        height="600"
                        onchange={ctx.link().callback(|e: Event| Msg::JsonChanged {value: e.target_unchecked_into::<HtmlInputElement>().value()})}
                        value={self.json.clone()}/>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShapeChanged { shape_type } => {
                self.shape_type = shape_type;

                return true;
            }
            Msg::MouseClicked { x, y } => {
                match self.mode {
                    Mode::Draw => {
                        let shape = self.shape_storage.get_or_create_shape(self.shape_type);
                        shape.add_point(x, y);
                    }
                    Mode::Resize | Mode::Move | Mode::Select => {
                        self.shape_storage.intersect_and_select(x, y);
                    }
                }

                return true;
            }
            Msg::MouseMove { x, y } => {
                match self.mode {
                    Mode::Draw => {
                        let shape = self.shape_storage.get_current_mut();
                        if shape.is_some() {
                            let shape = shape.unwrap();
                            if shape.get_state() == ShapeState::Drawing {
                                shape.set_end(x, y);
                            }
                        }
                    }
                    Mode::Select => {
                        self.shape_storage.intersect_and_highlight(x, y);
                    }
                    Mode::Resize => {
                        if self.is_dragging {
                            let shape = self.shape_storage.get_selected_mut();
                            if let Some(shape) = shape {
                                shape.resize(
                                    (x - self.last_cursor_pos.0, y - self.last_cursor_pos.1),
                                    self.resize_anchor,
                                );
                                self.last_cursor_pos = (x, y);
                                self.resize_anchor = (x, y);
                            }
                        } else {
                            self.shape_storage.intersect_and_highlight(x, y);
                        }
                    }
                    Mode::Move => {
                        if self.is_dragging {
                            let shape = self.shape_storage.get_selected_mut();
                            if shape.is_some() {
                                let shape = shape.unwrap();
                                shape.move_by(
                                    x - self.last_cursor_pos.0,
                                    y - self.last_cursor_pos.1,
                                );
                                self.last_cursor_pos = (x, y);
                            }
                        } else {
                            self.shape_storage.intersect_and_highlight(x, y);
                        }
                    }
                }

                return true;
            }
            Msg::ClearScreen => {
                self.shape_storage.clear();

                return true;
            }
            Msg::ModeChanged { mode } => {
                self.mode = mode;

                return true;
            }
            Msg::ValueChanged { key, value } => {
                let shape = self.shape_storage.get_selected_mut();
                if shape.is_some() {
                    let shape = shape.unwrap();
                    shape.set_prop(&key, &value);
                }

                true
            }
            Msg::MouseUp { x, y } => {
                match self.mode {
                    Mode::Resize | Mode::Move => {
                        self.is_dragging = false;
                        self.last_cursor_pos = (x, y);
                        self.resize_anchor = (x, y);
                    }
                    _ => {}
                }

                true
            }
            Msg::MouseDown { x, y } => {
                match self.mode {
                    Mode::Resize | Mode::Move => {
                        self.is_dragging = true;
                        self.last_cursor_pos = (x, y);
                        self.resize_anchor = (x, y);
                    }
                    _ => {}
                }

                true
            }
            Msg::NewShape => {
                self.shape_storage.new_shape(self.shape_type);

                true
            }
            Msg::SubmitShape => {
                self.shape_storage.submit_shape();

                true
            }
            Msg::SaveToJson => {
                self.json = self.shape_storage.serialize_to_json();
                let a = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("a")
                    .unwrap();
                a.set_attribute(
                    "href",
                    &format!("data:text/json;charset=utf-8,{}", self.json.clone()),
                )
                .unwrap();
                a.set_attribute("download", "shapes.json").unwrap();
                let a_element = a.dyn_into::<HtmlElement>().unwrap();
                a_element.click();
                a_element.remove();

                true
            }
            Msg::LoadFromJson { value } => {
                log::info!("value: {}", &value);
                self.shape_storage.deserialize_from_json(&value);
                self.json = value;

                true
            }
            Msg::JsonChanged { value } => {
                self.json = value;

                true
            }
            Msg::None => false,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let canvas = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector("#canvas")
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let rendering_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        rendering_context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for shape in self.shape_storage.get_shapes() {
            if shape.is_drawable() {
                shape.draw(&rendering_context);
            }
        }

        if let Some(shape) = self.shape_storage.get_highlighted() {
            if shape.is_drawable() {
                shape.draw_highlighted(&rendering_context);
            }
        }

        if let Some(shape) = self.shape_storage.get_selected() {
            if shape.is_drawable() {
                shape.draw_selected(&rendering_context);
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
