use std::sync::Arc;

use gloo_net::http::Request;
use web_sys::wasm_bindgen::JsCast;
use yew::{platform::spawn_local, prelude::*};

#[function_component(Login)]
pub fn login() -> Html {
    let login_on_click = Callback::from(|_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let username = document
            .get_element_by_id("username")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        let password = document
            .get_element_by_id("password")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();

        spawn_local(async move {
            let login_request = portfolio_common::LoginRequest { username, password };

            let response = Request::post("/api/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&login_request).unwrap())
                .unwrap()
                .send()
                .await
                .unwrap();

            web_sys::console::log_1(&"Response: {response}".into());

            if response.status() == 200 {
                web_sys::console::log_1(&"Login successful".into());
            } else {
                web_sys::console::log_1(&format!("Login failed: {}", response.status()).into());
            }
        });
    });

    html! {
        <div class="border rounded-sm items-center flex flex-col space-y-3 absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-fit p-10 shadow-xl/20">
            <div class="space-x-3">
                <label for="username">{ "Username:" }</label>
                <input type="text" id="username" name="username" required=true />
            </div>

            <div class="space-x-3">
                <label for="password">{ "Password:" }</label>
                <input type="password" id="password" name="password" required=true />
            </div>

            <button type="submit" onclick={login_on_click}>{ "Log in" }</button>
        </div>
    }
}
