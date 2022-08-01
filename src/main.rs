use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

mod markdown;

const FLOATIES_API: &str = "https://floaties-api.dudi.win/";

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

async fn fetch_floaties(url: &'static str, condition: String) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

enum Msg {
    SetStringFetchState(FetchState<String>),
    GetString,
    GetError,
}
struct App {
    markdown: FetchState<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            markdown: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetStringFetchState(fetch_state) => {
                self.markdown = fetch_state;
                true
            }
            Msg::GetString => {
                ctx.link().send_future(async {
                    match fetch_floaties(FLOATIES_API, "2 == 2".to_string()).await {
                        Ok(md) => Msg::SetStringFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetStringFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetStringFetchState(FetchState::Fetching));
                false
            }
            Msg::GetError => {
                ctx.link().send_future(async {
                    match fetch_floaties(FLOATIES_API, "2 == 2".to_string()).await {
                        Ok(md) => Msg::SetStringFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetStringFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetStringFetchState(FetchState::Fetching));

                
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.markdown {
            FetchState::NotFetching => html! {
                <>
                    <button onclick={ctx.link().callback(|_| Msg::GetString)}>
                        { "Get String" }
                    </button>
                    // <button onclick={ctx.link().callback(|_| Msg::GetError)}>
                    //     { "Get using incorrect URL" }
                    // </button>
                </>
            },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(data) => html! { data },
            FetchState::Failed(err) => html! { err },
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
