mod model;

use model::shape::ShapeType;
use web_sys::*;
use yew::prelude::*;



struct App {
    shape_type: ShapeType
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            shape_type: ShapeType::Line
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let canvas_callback = Callback::from(|event: MouseEvent| {
            log::info!("canvas event: {:?}", event.client_x());
        });

        html! {
            <div>
                <canvas width={"800"} height={"600"} onclick={canvas_callback} style={"border: 1px solid black"}/>
                <div>
                    <button>{"Line"}</button>
                    <button>{"Rectangle"}</button>
                    <button>{"Circle"}</button>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
