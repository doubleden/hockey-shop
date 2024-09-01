
use leptos::*;
use crate::models::item::Item;
use crate::components::items_collection::CollectionOfProduct;
use stylers::style;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let styler = style! { "Home",
        div.container {
            padding-left: 20px;
            padding-right: 20px;
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
        }

        mainIMG.img {
            border-radius: 20px;
        }
    };
    
    view! { class = styler,
            <div class="container">

                <picture class="mainIMG">
                    <source
                        srcset="https://siamster.com/img/logoShop.WEBP"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://siamster.com/img/logoShop.WEBP"
                        alt="Shop Logo"
                        height="150"
                        width="150"
                    />
                </picture>
                <div>
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
