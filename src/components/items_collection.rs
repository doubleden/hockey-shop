use crate::components::item_cell::ProductItem;
use leptos::*;
use crate::models::item::Item;
use stylers::style;

#[component]
pub fn CollectionOfProduct(products: Vec<Item>) -> impl IntoView {
    let styler = style! {"CollectionOfProduct",
        div.collection-view {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            justify-content: center;
        }
    };

    view! { class = styler,
        <div class="collection-view">
            {products
                .into_iter()
                .map(|product| view! { <ProductItem product=&product /> })
                .collect_view()}
        </div>
    }
}