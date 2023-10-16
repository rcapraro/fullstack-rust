use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;

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
        let deps: Vec<Product> = vec![];

        use_effect_with(deps, move |deps| {
            if deps.is_empty() {
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
            }
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
            <h2>{"List of products: "} {data.len()}{" products"}</h2>
            <p>{products}</p>
        </div>
    }
}
