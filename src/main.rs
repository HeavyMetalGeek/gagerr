pub mod components;
pub mod models;

use crate::components::list_table::ListTable;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let items = Vec::new();
    html!(
        <>
        <ListTable items={items}/>
        </>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
