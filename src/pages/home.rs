
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
                        srcset="images/logo.jpeg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="images/logo.jpeg"
                        alt="Shop Logo"
                        height="150"
                        width="150"
                    />
                </picture>
                <div class="collection">
                    <h2>"Protection"</h2>
                    <div>
                        <CollectionOfProduct products=Item::get_protections() />
                    </div>

                    <h2>"Palos Derecha"</h2>
                    <div>
                        <CollectionOfProduct products=Item::get_right_sticks() />
                    </div>

                    <h2>"Palos Izquierda"</h2>
                    <div>
                        <CollectionOfProduct products=Item::get_left_sticks() />
                    </div>

                    <h2>"Portero"</h2>
                    <div>
                        <CollectionOfProduct products=Item::get_portero() />
                     </div>

                    <h2>"Línea"</h2>
                    <div>
                        <CollectionOfProduct products=Item::get_linea() />
                    </div>
                </div>
            </div>
    }
}
