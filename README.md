# Rust full stack application

This sample project demonstrates a full stack application built in Rust.

It uses [Axum](https://github.com/tokio-rs/axum) for the server-side and [Yew](https://yew.rs)  for the client side.

## Server

### Install crates

```shell
cargo add axum
cargo add tokio -F full
cargo add tracing
cargo add tracing-subscriber
cargo add serde
cargo add serde_json
cargo add dotenv
cargo add sqlx -F sqlx/runtime-tokio, postgres, migrate, json, macros
cargo add tower
cargo add tower-http -F cors
```

### Watch

```shell 
cargo install cargo-watch
```

Then:

```shell 
cargo watch -x run
```

## Client

### Install [Yew](https://yew.rs) tools and crate

### Tools

```shell
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Libraries

- Yew client-side rendering:

```shell
cargo add yew -F csr
```

- Other libraries
```shell
cargo add serde
cargo add serde_json
cargo add wasm-bindgen-futures
cargo add reqwest -F json
cargo add web-sys -F HtmlInputElement
cargo add gloo-console
```

### Running

- Create an index.html
- Create a skeleton app in `main.rs` with at least

```rust
#[function_component]
fn App() -> Html {
    
    html! {
        <div>
            <h1>{"Yew Product App"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- Run `trunk serve`

