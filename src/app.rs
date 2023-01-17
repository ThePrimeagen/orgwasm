use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pagination::{PaginationBar, PaginationBarProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let pagination = create_rw_signal(cx, 0);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! {
                    cx,
                    <main class="bg-slate-800 h-screen w-screen my-0 mx-auto max-w-3xl text-center">
                        <PaginationBar count={14} signal={pagination}/>
                        {move || {
                            pagination.get()
                        }}
                    </main>
            }/>
            </Routes>
        </Router>
    }
}
