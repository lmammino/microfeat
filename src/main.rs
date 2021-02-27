use actix::prelude::*;
use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct MyMsg {
    text: String,
}

impl Message for MyMsg {
    type Result = Result<(), ws::ProtocolError>;
}

#[derive(Debug)]
struct AppState {
    flags: HashMap<String, String>,
    clients: Vec<Addr<MyWs>>,
}

#[derive(Debug)]
struct MyWs {
    state: web::Data<Mutex<AppState>>,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<MyMsg> for MyWs {
    type Result = Result<(), actix_web::client::WsProtocolError>;

    fn handle(
        &mut self,
        msg: MyMsg,
        ctx: &mut Self::Context,
    ) -> Result<(), actix_web::client::WsProtocolError> {
        ctx.text(msg.text);
        Ok(())
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn started(&mut self, ctx: &mut Self::Context) {
        let mut state = self.state.lock().unwrap();
        let addr = ctx.address();
        state.clients.push(addr);

        println!("{:?}", state.clients);
        let data = format!("{:?}", state.flags);
        ctx.text(data)
    }
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn websocket(
    state: web::Data<Mutex<AppState>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs { state }, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn enable_flag(state: web::Data<Mutex<AppState>>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").expect("Missing flag name");
    let mut state = state.lock().unwrap();
    state.flags.insert(String::from(name), String::from("true"));
    println!("{:?}", state);
    let message = format!("OK {} ON!", &name);
    for client in state.clients.iter() {
        // TODO: do we need to make this parallel or actix does that for us here?
        client.do_send(MyMsg {
            text: message.clone(),
        });
    }
    message
}

async fn disable_flag(state: web::Data<Mutex<AppState>>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").expect("Missing flag name");
    let mut state = state.lock().unwrap();
    state
        .flags
        .insert(String::from(name), String::from("false"));
    println!("{:?}", state);
    let message = format!("OK {} OFF!", &name);
    for client in state.clients.iter() {
        client.do_send(MyMsg {
            text: message.clone(),
        });
    }
    message
}

async fn delete_flag(state: web::Data<Mutex<AppState>>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").expect("Missing flag name");
    let mut state = state.lock().unwrap();
    state.flags.remove(&String::from(name));
    println!("{:?}", state);
    let message = format!("OK {} DELETED!", &name);
    for client in state.clients.iter() {
        client.do_send(MyMsg {
            text: message.clone(),
        });
    }
    message
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        flags: HashMap::new(),
        clients: vec![],
    };
    let state = web::Data::new(Mutex::new(state));
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/websocket", web::get().to(websocket))
            .route("/on/{name}", web::get().to(enable_flag))
            .route("/off/{name}", web::get().to(disable_flag))
            .route("/del/{name}", web::get().to(delete_flag))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
