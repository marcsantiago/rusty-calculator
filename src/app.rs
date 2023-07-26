use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    // let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    // Note: Args is not needed, it was here to demo commands in the tauri.main.rs file
                    // commands allow the front end to call rust code
                    // let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    // let new_msg = invoke("greet_from_tauri", args).await.as_string().unwrap();

                    let new_msg = format!(
                        "Hello, {}! You've been greeted from Rust!",
                        name.to_string()
                    );

                    greet_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    // let greet = {
    //     let name = name.clone();
    //     let greet_input_ref = greet_input_ref.clone();
    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default();
    //         name.set(
    //             greet_input_ref
    //                 .cast::<web_sys::HtmlInputElement>()
    //                 .unwrap()
    //                 .value(),
    //         );
    //     })
    // };

    html! {
    <div class="calc-body">
        <div class="calc-screen">
            <div id="calc-operation">{"1234 + 567 + "}</div>
            <div id="calc-typed">{890}</div>
        </div>
        <div class="calc-button-row">
            <button class="ac">{"AC"}</button>
            <button class="opt">{"+/-"}</button>
            <button class="opt">{"%"}</button>
            <button class="opt">{"/"}</button>
            <button>{"7"}</button>
            <button>{"8"}</button>
            <button>{"9"}</button>
            <button class="opt">{"*"}</button>
            <button>{"4"}</button>
            <button>{"5"}</button>
            <button>{"6"}</button>
            <button class="opt">{"-"}</button>
            <button>{"1"}</button>
            <button>{"2"}</button>
            <button>{"3"}</button>
            <button class="opt">{"+"}</button>
            <button>{"0"}</button>
            <button>{"."}</button>
            <button>{"DEL"}</button>
            <button class="opt">{"="}</button>
        </div>
    </div>
    }
}
