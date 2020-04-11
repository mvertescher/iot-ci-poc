#![recursion_limit = "512"]

use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Cbor, Json};
use yew::services::console::ConsoleService;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type AsBinary = bool;

pub enum WsAction {
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
}

pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<WsResponse, Error>),
    Ignore,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

/// This type is used to parse data from `./static/data.json` file and
/// have to correspond the data layout from that file.
#[derive(Deserialize, Debug)]
pub struct DataFromFile {
    value: u32,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

pub struct Model {
    ws_service: WebSocketService,
    link: ComponentLink<Model>,
    data: Option<u32>,
    ws: Option<WebSocketTask>,
}

impl Model {
    fn ws_connect(&mut self) {
        let callback = self.link.callback(|Cbor(data)| {
            let mut console = ConsoleService::new();
            console.log(&format!("cbor: {:?}", data));

            Msg::WsReady(data)
        });
        let notification = self.link.callback(|status| match status {
            WebSocketStatus::Opened => Msg::Ignore,
            WebSocketStatus::Closed | WebSocketStatus::Error => WsAction::Lost.into(),
        });
        let task = self
            .ws_service
            .connect("ws://localhost:8080/ws/", callback, notification)
            .unwrap();

        self.ws = Some(task);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            ws_service: WebSocketService::new(),
            link,
            data: None,
            ws: None,
        };
        model.ws_connect();

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    self.ws_connect();
                }
                WsAction::SendData(binary) => {
                    let request = WsRequest { value: 321 };

                    if binary {
                        self.ws.as_mut().unwrap().send_binary(Json(&request));
                    } else {
                        self.ws.as_mut().unwrap().send(Json(&request));
                    }
                }
                WsAction::Disconnect => {
                    self.ws.take();
                }
                WsAction::Lost => {
                    self.ws = None;
                }
            },
            Msg::WsReady(response) => {
                self.data = response.map(|data| data.value).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        // let mut console = ConsoleService::new();
        // console.log("view refresh...");

        html! {
            <>
            // Header
            <div>
                <div class="header">
                    <div class="flex bg-white border-b border-grey-200 fixed top-0 inset-x-0 z-100 h-16 items-center">
                        <div class="w-full max-w-screen=x1 relative mx-auto px-6">
                            <div class="flex items-center -mx-6">
                                <div class="lg:w-1/4 xl:w-1/5 pl-6 pr-6 lg:pr-8">
                                    { "Dashboard" }
                                </div>
                                <div class="flex flex-grow lg:w-3/4 xl:w-4/5">
                                    { "" }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Padding
            <div class="py-10"/>

            // Body
            <div class="container mx-auto">
                <div class="flex bg-grey-200 py-8">
                    <button class="flex-1 text-gray-700 text-center bg-gray-400 px-4 py-2 m-2"
                            disabled=self.ws.is_some()
                            onclick=self.link.callback(|_| WsAction::Connect)>
                        { "Connect To WebSocket" }
                    </button>
                    <button class="flex-1 text-gray-700 text-center bg-gray-400 px-4 py-2 m-2"
                            disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData(false))>
                        { "Send To WebSocket" }
                    </button>
                    <button class="flex-1 text-gray-700 text-center bg-gray-400 px-4 py-2 m-2"
                           disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::SendData(true))>
                        { "Send To WebSocket [binary]" }
                    </button>
                    <button class="flex-1 text-gray-700 text-center bg-gray-400 px-4 py-2 m-2"
                            disabled=self.ws.is_none()
                            onclick=self.link.callback(|_| WsAction::Disconnect)>
                        { "Close WebSocket connection" }
                    </button>
                </div>
            </div>
            </>
        }
    }
}
