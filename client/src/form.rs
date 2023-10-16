use gloo_console::log;
use gloo_net::http::Request;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn Form() -> Html {
    // name
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    // price
    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    // submit form data
    let onclick = Callback::from(move |_| {
        log!("Button clicked");
        let price = price_ref.cast::<HtmlInputElement>().unwrap();
        let name = name_ref.cast::<HtmlInputElement>().unwrap();
        log!(format!(
            "Adding product with name: {}, price: {}",
            name.value(),
            price.value()
        ));

        spawn_local(async move {
            let product = json!({
                "name": name.value(),
                "price": price.value().parse::<i32>().unwrap()
            });

            Request::post("http://localhost:3000/api/products")
                .json(&product)
                .expect("cannot convert to json")
                .send()
                .await
                .expect("cannot post data to url");
        })
    });

    html! {
        <div class="container">
            <h2>{"Add a product:"}</h2>
            <div>
                <label for="name">
                    {"Name:"}
                    <input
                        ref={name_ref_outer.clone()}
                        id="name"
                        class="formInput"
                        type="text"
                    />
                </label>
                <br/><br/>
                <label for="price">
                    {"Price:"}
                    <input
                        ref={price_ref_outer.clone()}
                        id="price"
                        class="formInput"
                        type="number"
                    />
            </label>
            <br/><br/>
            <button {onclick} class="btn-primary">{"Add Product"}</button>
            </div>
            <hr/>
        </div>
    }
}
