use dioxus::prelude::*;
#[component]
pub fn RootLayout() -> Element {
    rsx! {
        crate::components::navbar::Navbar {}
        crate::components::sidebar::Sidebar {}
        Outlet::<crate::Route> { }
        // footer { "footer" }
    }
}
