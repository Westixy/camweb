use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="/assets/camweb.svg" alt="CamWeb Logo" />
            <h1>{ "CamWeb" }</h1>
            <span class="subtitle">{ "Work in progress..." }</span>
        </main>
    }
}
