use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::auth_store::AuthStore;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let (_, auth_dispatch) = use_store::<AuthStore>();

    let onchange_username = {
        let dispatch = auth_dispatch.clone();
        Callback::from(move |event: Event| {
            let username = event.target_unchecked_into::<HtmlInputElement>().value();
            let username = if username.is_empty() {
                None
            } else {
                Some(username)
            };
            dispatch.reduce_mut(|store| store.username = username);
        })
    };

    let onchange_password = auth_dispatch.reduce_mut_callback_with(|store, event: Event| {
        let password = event.target_unchecked_into::<HtmlInputElement>().value();

        store.password = Some(password).filter(String::is_empty);

        // store.password = if password.is_empty() {
        //     None
        // } else {
        //     Some(password)
        // };
    });

    html! {
        <form>
            <h2>{"Login"}</h2>
            <div>
                <div>
                    <label for="username">{"username"}</label>
                </div>
                <div>
                <input type="text" id="username" placeholder="username" onchange={onchange_username} />
                </div>
            </div>
            <div>
                <div>
                    <label for="password">{"password"}</label>
                </div>
                <div>
                <input type="password" id="password" placeholder="password" onchange={onchange_password} />
                </div>
            </div>
            <div>
                <button>{"login"}</button>
            </div>

        </form>
    }
}
