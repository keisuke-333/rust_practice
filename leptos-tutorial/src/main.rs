use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Counter"
        </button>
        <p>{move || count.get()}</p>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
