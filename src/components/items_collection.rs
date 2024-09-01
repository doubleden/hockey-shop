use crate::components::item_cell::ProductItem;
use leptos::*;
use crate::models::item::Item;

#[component]
pub fn CollectionOfProduct(products: Vec<Item>) -> impl IntoView {
    view! {
        <div class="collection-view">
            {products.into_iter()
                .map(|product| view! { <ProductItem product={&product} /> })
                .collect_view()}
        </div>
    }
}