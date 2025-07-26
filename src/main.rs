use app::App;

fn main() {
    leptos::mount::mount_to_body(App)
}

mod owned_tree;

mod app;

mod components {
    pub mod parse_tree_component;
}
