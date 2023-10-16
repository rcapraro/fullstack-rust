use router::switch;
use router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

mod form;
mod products;

mod router;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <h1 class= "title">{"Yew Product App"}</h1>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
