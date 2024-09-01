use crate::components::items_collection::CollectionOfProduct;
use leptos::*;
use crate::models::item::Item;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

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

                <h2>"Protection"</h2>
                <div class="collection">
                    <CollectionOfProduct products={Item::get_protections()} />
                </div>

                <h2>"Palos Derecha"</h2>
                <div class="collection">
                    <CollectionOfProduct products={Item::get_right_sticks()} />
                </div>

                <h2>"Palos Izquierda"</h2>
                <div class="collection">
                    <CollectionOfProduct products={Item::get_left_sticks()} />
                </div>

                <h2>"Portero"</h2>
                <div class="collection">
                    <CollectionOfProduct products={Item::get_portero()} />
                </div>

                <h2>"Línea"</h2>
                <div class="collection">
                    <CollectionOfProduct products={Item::get_linea()} />
                </div>
            </div>
        </ErrorBoundary>
    }
}
