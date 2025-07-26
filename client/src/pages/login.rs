use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="border rounded-sm items-center flex flex-col space-y-3 absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-fit p-10 shadow-xl/20">
            <div class="space-x-3">
                <label for="username">{ "Username:" }</label>
                <input type="text" name="username" required=true />
            </div>

            <div class="space-x-3">
                <label for="password">{ "Password:" }</label>
                <input type="password" name="password" required=true />
            </div>

            <button type="submit">{ "Log in" }</button>
        </div>
    }
}
