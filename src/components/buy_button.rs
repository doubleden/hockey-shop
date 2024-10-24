use leptos::*;
use crate::models::item::{Item};
use crate::models::item::ItemStatus;

#[component]
pub fn WhatsappButton(item: Item) -> impl IntoView {
    const PHONE_NUMBER: &str = "79160518106";
    let disponible_message = format!(
        "Hola, quiero comprar {} {} {} {}", item.name, item.description, item.price, item.image_url
    );

    let ordenar_message = format!(
        "Hola, quiero comprar {} {} {} {}", item.name, item.description, item.price, item.image_url
    );

    let (button_text, whatsapp_url) = match item.status {
        ItemStatus::Logrono => {
            let whatsapp_url = format!(
                "https://wa.me/{}?text={}",
                PHONE_NUMBER,
                urlencoding::encode(&disponible_message)
            );
            ("Comprar", whatsapp_url)
        },
        ItemStatus::Disponible => {
            let whatsapp_url = format!(
                "https://wa.me/{}?text={}",
                PHONE_NUMBER,
                urlencoding::encode(&ordenar_message)
            );
            ("Comprar", whatsapp_url)
        },
        _ => {
            ("No disponible", String::new())
        }
    };

    view! {
        <a href=whatsapp_url target="_blank" class="whatsapp-button">
            <button>{button_text}</button>
        </a>
    }
}