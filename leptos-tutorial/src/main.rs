use leptos::*;
use leptos_router::*;

mod pages;

use crate::pages::{About::About, Contact::Contact, Home::Home};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/about" view=About />
                <Route path="/contact" view=Contact />
            </Routes>
        </Router>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
