use yew::prelude::*;

mod products;
mod form;

use products::Products;

#[function_component]
fn App() -> Html {

    html! {
        <div class="container">
            <h1 class= "title">{"Yew Product App"}</h1>
            <Products/>
            <p></p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
