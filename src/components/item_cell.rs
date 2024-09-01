use leptos::*;
use crate::models::item::{Item, ItemStatus};
use crate::components::buy_button::WhatsappButton;
use stylers::style;

#[component]
pub fn ProductItem<'a>(product: &'a Item) -> impl IntoView {
    let whatsapp_button = move || {
        if product.status == ItemStatus::Disponible {
            Some(view! { <WhatsappButton product=product /> })
        } else {
            None
        }
    };

    let styler = style! {"ProductItem",
        div.product-item {
            border: 1px solid #ddd;
            padding: 10px;
            width: 200px;
            text-align: center;
            border-radius: 15px;
            min-height: 350px;
            display: grid;
            grid-template-rows: 1fr 0.5fr auto;
            height: 100%;
        }

        product-status {
            margin-top: 5px;
            margin-bottom: 20px;
        }

        product-image {
            width: 100%;
            height: 200px;
        }

        product-image img {
            width: 100%;
            height: 100%;
            object-fit: cover;
            border-radius: 15px;
        }

        descr {
            font-size: 60%;
        }
          
        sold {
            background: red;
            border-radius: 20px;
        }
          
        available {
            background: green;
            border-radius: 20px;
        }
          
        reserved {
            background: orange;
            border-radius: 20px;
        }
    };

    view! { class = styler,
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
                    }}
                </div>
                {whatsapp_button()}
            </div>
        </div>
    }
}