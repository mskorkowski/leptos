use leptos::prelude::*;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::path;
use leptos_router::StaticSegment;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body>
                <RouterExample/>
            </body>
        </html>
    }
}

#[component]
pub fn RouterExample() -> impl IntoView {
    view! {
        <Router>
            <main>
                // !!!!!!!!!!!!!!!!!!!!!!!!
                // 
                // To understand how the things are matched reorder the routes to see the changes
                //
                // !!!!!!!!!!!!!!!!!!!!!!!!
                <Routes fallback=|| "Not found. fallback">
                    <Route path=(StaticSegment(""),StaticSegment("/")) view=|| view!{ "Empty slash"} />
                    <Route path=(StaticSegment("/"),StaticSegment("/")) view=|| view!{ "Slash slash"} />
                    <Route path=(StaticSegment("/"),StaticSegment("")) view=|| view!{ "Slash empty"} />
                    <Route path=(StaticSegment(""),StaticSegment("")) view=|| view!{ "Empty"} />
                    // <Route path=path!("/users") view=|| view!{ "Users" } />
                    // <Route path=path!("/users/id") view=|| view!{ "user profile" } />
                    // <Route path=path!("/*any") view=|| view! { <h1>"Not Found any"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(RouterExample);
}
