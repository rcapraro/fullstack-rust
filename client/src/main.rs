use yew::prelude::*;

mod form;
mod products;

use form::Form;
use products::Products;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <h1 class= "title">{"Yew Product App"}</h1>
            <Form/>
            <Products/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
