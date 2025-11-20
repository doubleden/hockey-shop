
use leptos::*;
use crate::models::item::Item;
use crate::components::items_collection::CollectionOfProduct;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
            <div class="container">

                <picture class="mainIMG">
                    <source
                        srcset="images/logo.png"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="images/logo.png"
                        alt="Shop Logo"
                        height="150"
                        width="150"
                    />
                </picture>
                <div class="collection">
                    <h2 class="sticky-header">"Sticks for order"</h2>
                    <div>
                        <CollectionOfProduct products=Item::get_sticks_for_order() />
                    </div>
                </div>
            </div>
    }
}
