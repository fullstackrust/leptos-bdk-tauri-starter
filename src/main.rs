#![allow(non_snake_case, clippy::redundant_closure)]
use bip39::{Language, Mnemonic};
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (words, set_words) = create_signal(12_u32);
    let (mnemonic, set_mnemonic) = create_signal("".to_owned());

    view! {
        <div>
            <p>
                <button on:click=move |_| {
                    set_words(12);
                }>"12 words"</button>
            </p>
            <p>
                <button on:click=move |_| {
                    set_words(24);
                }>"24 words"</button>
            </p>
            <p>"Number of words: " {move || words()}</p>
        </div>
        <div>
            <p>
                <button on:click=move |_| {
                    let mut entropy = [0u8; 32];
                    getrandom::getrandom(&mut entropy).expect("entropy generation");
                    let m = Mnemonic::from_entropy_in(Language::English, &entropy)
                        .expect("mnemonic generation");
                    set_mnemonic(m.to_string());
                }>"Generate mnemonic"</button>
            </p>
            <p>"Mnemonic: " {move || mnemonic()}</p>
        </div>
    }
}
