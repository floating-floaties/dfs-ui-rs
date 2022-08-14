use yew::prelude::*;
use yew_router::prelude::*;
use yew_oauth2::prelude::*;
use yew_oauth2::oauth2::*;

use yew::Callback;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/news")]
    News,
    #[at("/contact")]
    Contact,
    #[at("/settings/:s")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
enum SettingsRoute {
    #[at("/settings/profile")]
    Profile,
    #[at("/settings/friends")]
    Friends,
    #[at("/settings/theme")]
    Theme,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

fn switch_main(route: &MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<h1>{"Home"}</h1>},
        MainRoute::News => html! {<h1>{"News"}</h1>},
        MainRoute::Contact => html! {<h1>{"Contact"}</h1>},
        MainRoute::Settings => html! {
            <Switch<SettingsRoute> render={Switch::render(switch_settings)} />
        },
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

fn switch_settings(route: &SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<h1>{"Profile"}</h1>},
        SettingsRoute::Friends => html! {<h1>{"Friends"}</h1>},
        SettingsRoute::Theme => html! {<h1>{"Theme"}</h1>},
        SettingsRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let config = Config {
        client_id: "cognito-client-id".into(),
        auth_url: "cognito-url".into(),
        token_url: "toek-url/token".into()
    };

    let login = Callback::from(|_: MouseEvent| {
        OAuth2Dispatcher::<Client>::new().start_login();
    });
    
    let logout = Callback::from(|_: MouseEvent| {
        OAuth2Dispatcher::<Client>::new().logout();
    });

    html! {
        <OAuth2 {config}>
            <Authenticated>
                <p>
                    <button onclick={logout.clone()}>{"Logout"}</button>
                </p>
                <BrowserRouter>
                    <Switch<MainRoute> render={Switch::render(switch_main)} />
                </BrowserRouter>
            </Authenticated>
            <NotAuthenticated>
                <p>
                    {"You need to log in"}
                    <button onclick={login.clone()}>{"Login"}</button>
                </p>
            </NotAuthenticated>
        </OAuth2>
    }
}

fn main() {
    yew::start_app::<App>();
}
