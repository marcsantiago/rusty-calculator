use crate::calculator;
use crate::keyboard_event_helper::keyboard_code_to_character;
use gloo::console::{info, log};
use gloo::events::EventListener;
use js_sys::Math::log;
use std::char;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub enum Msg {
    Input(char),
    Clear,
    Delete,
    Percent,
    Equals,
    KeyEvent { event: KeyboardEvent },
}

#[derive(Debug, Default)]
pub struct App {
    calculator: calculator::calculate::Calculator,
    expression: String,
    answer: String,
    /// Holds the listener once it's stood up. Can't be done before rendering because... the document doesn't exist yet!
    pub kbd_listener: Option<EventListener>,
}

impl App {
    fn new() -> Self {
        App {
            calculator: calculator::calculate::Calculator::new(),
            expression: String::from("0"),
            answer: String::from("0"),
            kbd_listener: None,
        }
    }

    fn display_expression(&self) -> String {
        self.expression.clone()
    }

    fn display_answer(&self) -> String {
        self.answer.clone()
    }

    fn evaluate(&mut self) {
        let result = self.calculator.evaluate(self.expression.clone());
        let mut res = String::from("Error");
        if let Ok(answer) = result {
            res = answer.to_string();
        }
        self.answer = res;
    }

    fn evaluate_as_percent(&mut self) {
        let result = self.calculator.evaluate(self.expression.clone());
        let mut res = String::from("Error");
        if let Ok(answer) = result {
            res = (answer / 100.0).to_string();
        }
        self.answer = res;
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App::new()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        return match msg {
            Msg::Input(ch) => {
                if self.expression.len() == 1 {
                    if self.expression == "0" {
                        self.expression.clear();
                    }
                }
                self.expression.push(ch);
                true
            }
            Msg::Clear => {
                self.expression.clear();
                self.expression.push('0');
                self.answer.clear();
                self.answer.push('0');
                true
            }
            Msg::Delete => {
                self.expression.pop();
                if self.expression.len() == 0 {
                    self.expression.push('0');
                }
                true
            }
            Msg::Percent => {
                self.evaluate_as_percent();
                true
            }
            Msg::Equals => {
                self.evaluate();
                true
            }
            Msg::KeyEvent { event } => {
                let ch = keyboard_code_to_character(event);
                match ch {
                    'd' => {
                        self.expression.pop();
                        if self.expression.len() == 0 {
                            self.expression.push('0');
                        }
                    }
                    '=' => {
                        self.evaluate();
                    }
                    '%' => {
                        self.evaluate_as_percent();
                    }
                    '\0' => {}
                    _ => {
                        if self.expression.len() == 1 {
                            if self.expression == "0" {
                                self.expression.clear();
                            }
                        }
                        self.expression.push(ch);
                    }
                }
                true
            }
        };
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="calc-body">
            <div class="calc-screen">
                <div id="calc-operation">{self.display_expression()}</div>
                <div id="calc-typed">{self.display_answer()}</div>
            </div>
            <div class="calc-button-row">
                <button class="ac" onclick={ctx.link().callback(|_| Msg::Clear)}>{"AC"}</button>
                <button class="opt">{"+/-"}</button>
                <button class="opt" onclick={ctx.link().callback(|_| Msg::Percent)}>{"%"}</button>
                <button class="opt" onclick={ctx.link().callback(|_| Msg::Input('/'))}>{"/"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('7'))}>{"7"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('8'))}>{"8"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('9'))}>{"9"}</button>
                <button class="opt"  onclick={ctx.link().callback(|_| Msg::Input('*'))}>{"*"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('4'))}>{"4"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('5'))}>{"5"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('6'))}>{"6"}</button>
                <button class="opt"  onclick={ctx.link().callback(|_| Msg::Input('-'))}>{"-"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('1'))}>{"1"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('2'))}>{"2"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('3'))}>{"3"}</button>
                <button class="opt"  onclick={ctx.link().callback(|_| Msg::Input('+'))}>{"+"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('0'))}>{"0"}</button>
                <button onclick={ctx.link().callback(|_| Msg::Input('.'))}>{"."}</button>
                <button onclick={ctx.link().callback(|_| Msg::Delete)}>{"DEL"}</button>
                <button class="opt" onclick={ctx.link().callback(|_| Msg::Equals)}>{"="}</button>
            </div>
        </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // we only need to run the below stuff the first time
        if !first_render {
            return;
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let ct = ctx.link().to_owned();
        let listener = EventListener::new(&document, "keydown", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw().to_owned();
            ct.send_message(Msg::KeyEvent { event });
        });

        self.kbd_listener.replace(listener);
    }
}
