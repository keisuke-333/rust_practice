use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "counter"
        </button>
        <p class:red=move || count() % 2 == 1>{count}</p>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
