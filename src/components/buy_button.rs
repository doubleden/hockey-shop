use leptos::*;
use crate::models::item::{Item};

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
    view! {
        <a href=whatsapp_url target="_blank" class="whatsapp-button">
            <button>"Comprar"</button>
        </a>
    }
}