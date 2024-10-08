use leptos::*;
use crate::models::item::{Item, ItemStatus};
use crate::components::buy_button::WhatsappButton;

#[component]
pub fn ProductItem(product: Item) -> impl IntoView {
    let whatsapp_button =  {
        view! { <WhatsappButton item = product.clone() /> }
    };

    let is_button_hidden = match product.status {
        ItemStatus::Disponible | ItemStatus::Ordenar => "nothidden",
        _ => "hidden",
    };

    view! {
        <div class="product-item">
            <div class="product-image">
                <a href=product.image_url target="_blank">
                    <img src=product.image_url alt=product.name />
                </a>
            </div>
            <div>
                <p>{product.name}</p>
                <div class="descr">{product.description}</div>
                <p>{format!("Price: {}€ ", product.price)}</p>
            </div>
            <div>
                <div class="product-status">
                    {match product.status {
                        ItemStatus::Vendido => view! { <div class="sold">"Vendido"</div> },
                        ItemStatus::Disponible => view! { <div class="available">"Disponible"</div> },
                        ItemStatus::Reservado => view! { <div class="reserved">"Reservado"</div> },
                        ItemStatus::Ordenar => view! { <div class="order">"Por encargo"</div> }
                    }}
                </div>
                <div class=is_button_hidden>
                     { whatsapp_button }
                </div>
            </div>
        </div>
    }
}