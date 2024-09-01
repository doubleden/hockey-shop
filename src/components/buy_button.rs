use leptos::*;
use crate::models::item::{Item};
use stylers::style;

#[component]
pub fn WhatsappButton<'a>(product:&'a Item) -> impl IntoView {
    const PHONE_NUMBER: &str = "79160518106";
    let message = format!(
        "Hola, quiero comprar {} {} {}", product.name, product.description, product.price
    );
    let whatsapp_url = format!(
        "https://wa.me/{}?text={}",
        PHONE_NUMBER,
        urlencoding::encode(&message)
    );

    let styler = style! {"WhatsappButton",
        whatsapp-button button {
            background-color: gray;
            color: white;
            padding: 10px 20px;
        }
      
        whatsapp-button button:hover {
            background-color: black;
        }
    };

    view! { class = styler,
        <a href=whatsapp_url target="_blank" class="whatsapp-button">
            <button>"Comprar"</button>
        </a>
    }
}