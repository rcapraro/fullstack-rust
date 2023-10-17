use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
}

#[function_component]
pub fn Products() -> Html {
    let data: UseStateHandle<Vec<Product>> = use_state(Vec::new);
    {
        let data_clone = data.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                    let fetched_data = Request::get("http://localhost:3000/api/products")
                        .send()
                        .await
                        .expect("cannot get data from url")
                        .json::<Vec<Product>>()
                        .await
                        .expect("cannot convert to json");

                    data_clone.set(fetched_data);
                });
            || () // no destructor to run
        });
    }

    let products = data
        .iter()
        .map(|product| html! {
                <ul>
                    <li key={product.id}>{format!("Name: {}, Price: {}", product.name, product.price)}</li>
                </ul>
              }
        ).collect::<Html>();

    html! {
        <div class="container">
            <button class="btn-primary">
                <Link<Route> to={Route::AddProduct}>{ "Add new Product" }</Link<Route>>
            </button>
            <h2>{"List of Products: "} {data.len()} </h2>
            <p>{products}</p>
        </div>
    }
}
