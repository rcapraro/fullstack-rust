use yew::prelude::*;
use web_sys::HtmlInputElement;

#[function_component]
pub fn Form() -> Html {

    html! {
        <div class="container">
            <h2>{"Add a product"}</h2>
            <div>
                <label for="name" class="">
                    {"Name:"}
                    <input
                        id="name"
                        class="formInput"
                        type="text"
                    />
                </label>
                <br/><br/>
                <label for="price" class="">
                    {"Price:"}
                    <input
                        id="price"
                        class="formInput"
                        type="number"
                    />
            </label>
            <br/><br/>
            <button class="btn-primary">{"Add Product"}</button>
            </div>
            <hr/>
        </div>
    }
}