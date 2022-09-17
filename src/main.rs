use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod content;
mod generator;
mod markdown;
mod pages;

use inflection_rs::inflection::Inflection;
use pages::{
    author::Author, author_list::AuthorList, home::Home, page_not_found::PageNotFound, post::Post,
    post_list::PostList,
};
use yew::html::Scope;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u64 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    navbar_active: bool,
    user_language: Option<String>,
    inflection: Inflection,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // let window: web_sys::Window = gloo_utils::window();
        // let navigator = window.navigator();

        // if let Some(window) = window_opt {
        //     let lan = navigator.userLanguage || navigator.language;
        // }

        Self {
            navbar_active: false,
            user_language: None,
            inflection: Inflection::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let result: Html = html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    // <div class="content has-text-centered">
                    //     { "Powered by " }
                    //     <a href="https://yew.rs">{ "Yew" }</a>
                    //     { " using " }
                    //     <a href="https://bulma.io">{ "Bulma" }</a>
                    //     { " and images from " }
                    //     <a href="https://unsplash.com">{ "Unsplash" }</a>
                    // </div>
                </footer>
            </BrowserRouter>
        };

        result
    }
}
impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar main-color-background" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                        <Link<Route> to={Route::Home}>
                            <h1 class="navbar-item is-size-3">
                                // <img
                                //     alt=""
                                //     src="/images/icon/android-chrome-192x192.png"
                                //     style="padding-left: 1rem; padding-right: 1rem;"
                                //     width="64px"
                                //     height="100%"
                                // />
                                { "Floating Floaties" }
                            </h1>
                        </Link<Route>>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Posts}>
                            { "Blog" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "Projects" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Authors}>
                                    { "Dialog Flow System" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Post { id } => {
            let result = html! { <Post seed={id} /> };
            result
        }
        Route::Posts => {
            let result = html! { <PostList /> };
            result
        }
        Route::Author { id } => {
            let result = html! { <Author seed={id} /> };
            result
        }
        Route::Authors => {
            let result = html! { <AuthorList /> };
            result
        }
        Route::Home => {
            let result = html! { <Home /> };
            result
        }
        Route::NotFound => {
            let result = html! { <PageNotFound /> };
            result
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
