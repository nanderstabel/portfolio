// use gloo::console::log;
// use serde::Serialize;
// use stylist::{style, yew::styled_component, Style};
// use yew::prelude::*;
// use yew::ContextProvider;
// use yew_router::prelude::*;

// mod auth_form;
// mod components;
// mod display_auth;
// mod display_count;
// mod increment_count;
// mod router;
// mod stores;
// use auth_form::AuthForm;
// use components::atoms::main_title::{Color, MainTtile};
// use components::atoms::struct_hello::StructHello;
// use increment_count::IncrementCount;
// // use components::molecules::{custom_form::CustomForm, struct_counter::StructCounter};
// use display_auth::DisplayAuth;
// use display_count::DisplayCount;
// use std::ops::Deref;
// // use crate::components::molecules::custom_form::Data;
// use crate::router::{switch, Route};

// mod login_form;

// // #[derive(Serialize)]
// // struct MyObject {
// //     username: String,
// //     favorite_language: String,
// // }

// // const STYLE_FILE: &str = include_str!("main.css");

// // #[derive(Clone, PartialEq, Debug, Default)]
// // pub struct User {
// //     pub username: String,
// //     pub favorite_language: String,
// // }

// // #[styled_component(App)]
// // pub fn app() -> Html {
// //     let stylesheet = Style::new(STYLE_FILE).unwrap();

// //     let name = "Nander";
// //     let my_object = MyObject {
// //         username: name.to_owned(),
// //         favorite_language: "Rust".to_owned(),
// //     };

// //     // log!("my name is ", name);
// //     // log!(serde_json::to_string_pretty(&my_object).unwrap());
// //     let class = "my_titles";
// //     let message: Option<&str> = None;

// //     let tasks = vec!["record video", "do groceries", "pet Yuhan"];

// //     let user_state = use_state(User::default);
// //     let main_title_load = Callback::from(|message: String| log!(message));
// //     let first_load = use_state(|| true);

// //     use_effect(move || {
// //         // this code will run on
// //         // - first render
// //         // - all re-renders
// //         // if auth token exists and if its our first render
// //         // get all the users todo tasks
// //         if *first_load {
// //             // this is only run when the component loads for the first time
// //             // this shouldn't run every time we refresh the state and re-render
// //             // do whatever data load we need to do
// //             first_load.set(false);
// //         }

// //         || {}
// //     });

// //     let custom_from_submit = {
// //         let user_state = user_state.clone();
// //         Callback::from(move |data: Data| {
// //             let mut user = user_state.deref().clone();
// //             user.username = data.username;
// //             user.favorite_language = data.favorite_language;
// //             user_state.set(user);
// //         })
// //     };
// //     html! {
// //         <>
// //             // <ContextProvider<User> context={user_state.deref().clone()}>
// //             //     <MainTtile title="Hi there!" color={Color::Ok} on_load={main_title_load}/>
// //             //     <CustomForm onsubmit={custom_from_submit}/>
// //             //     // <p class={css!("color: red; font-size: 75px;")}>{"more text"}</p>
// //             //     <BrowserRouter>
// //             //         <Switch<Route> render={switch} />
// //             //     </BrowserRouter>

// //             // </ContextProvider<User>>

// //             <div>
// //                 // <StructHello message={"Hello from lib.rs".to_owned()} />
// //                 <StructCounter />
// //             </div>

// //             // if class == "my_titles" {
// //             //     <p>{"Hi there!"}</p>
// //             // } else {
// //             //     <p>{"I'm not a titles"}</p>
// //             // }

// //             // if let Some(message) = message {
// //             //     <p>{message}</p>
// //             // } else {
// //             //     <p>{"no messages to see today"}</p>

// //             // }

// //             // <ul>
// //             //     {list_to_html(tasks)}
// //             // </ul>
// //         </>
// //     }
// // }

// // fn list_to_html(list: Vec<&str>) -> Vec<Html> {
// //     list.iter().map(|t| html! {<li>{t}</li>}).collect()
// // }

// pub struct App {}

// impl Component for App {
//     type Message = ();

//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <div>
//                 <h1>{"App"}</h1>
//                 <AuthForm />
//                 <DisplayAuth />
//             </div>
//         }
//     }
// }

mod login_form;
mod store;
use login_form::LoginForm;
use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{"App"}</h1>
            <LoginForm />

        </div>
    }
}
