#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc};
use ethers::types::{Address, RecoveryMessage, Signature, SignatureError};
use rocket::{Config, State};
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::cors::CORS;

mod cors;

#[derive(Default)]
struct App {
    balances: Arc<RwLock<HashMap<Address, f64>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignedRequest {
    signature: String,
    raw_message: String,
}

impl SignedRequest {
    async fn parse(self) -> Result<(Address, TransferRequest), SignatureError> {
        let signature: Signature = self.signature.parse()?;
        let message = RecoveryMessage::from(self.raw_message.as_str());
        let address = signature.recover(message.clone())?;
        signature.verify(message.clone(), address)?;
        let request: Result<TransferRequest, serde_json::Error> = serde_json::from_str(&self.raw_message);
        let Ok(request) = request else {
            return Err(SignatureError::RecoveryError);
        };
        Ok((address, request))
    }
}



#[derive(Serialize, Deserialize)]
struct TransferRequest {
    recipient: String,
    amount: f64,
}

#[derive(Serialize, Deserialize)]
struct BalanceResponse {
    balance: f64
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    message: String
}

impl ErrorMessage {

    fn new(message: &str) -> Self {
        ErrorMessage { message: message.into() }
    }
    fn into_bad_request(self) -> ErrorResponse {
        ErrorResponse::BadRequest(BadRequest(self.into()))
    }

    fn into_not_found(self) -> ErrorResponse {
        ErrorResponse::NotFound(NotFound(self.into()))
    }
}

#[derive(Debug, Responder)]
enum ErrorResponse {
    BadRequest(BadRequest<Json<ErrorMessage>>),
    NotFound(NotFound<Json<ErrorMessage>>),
}

#[get("/balance/<address>")]
async fn get_balance(address: &str, state: &State<App>) -> Result<Json<BalanceResponse>, ErrorResponse> {
    let Ok(address) = Address::from_str(address) else {
        return Err(ErrorMessage::new("Address is invalid.").into_bad_request());
    };
    let balance = get_or_init_balance(&address, state).await;
    Ok(Json(BalanceResponse { balance }))
}

async fn get_or_init_balance(account: &Address, state: &State<App>) -> f64 {
    let balance = {
        let reader = state.balances.read().await;
        reader.get(account).copied()
    };
    match balance {
        Some(balance) => balance,
        None => {
            let mut writer = state.balances.write().await;
            writer.insert(*account, 100.0); // Welcome credit for demo purposes
            100.0
        }
    }
}

#[post("/send", data = "<req>")]
async fn send(req: Json<SignedRequest>, state: &State<App>) -> Result<Json<BalanceResponse>, ErrorResponse> {
    let Ok((sender_address, transfer_request)) = req.into_inner().parse().await else {
        return Err(ErrorMessage::new("Invalid Signature").into_bad_request());
    };
    let TransferRequest { recipient, amount } = transfer_request;
    let Ok(recipient_address) = Address::from_str(&recipient) else {
        return Err(ErrorMessage::new("Address is invalid.").into_bad_request());
    };
    let sender_balance = get_or_init_balance(&sender_address, state).await;
    let recipient_balance = get_or_init_balance(&recipient_address, state).await;
    if sender_balance < amount {
        return Err(ErrorMessage::new("Not enough funds").into_bad_request());
    }
    let mut writer = state.balances.write().await;
    let new_sender_balance = sender_balance - amount;
    writer.insert(sender_address, new_sender_balance);
    writer.insert(recipient_address, recipient_balance + amount);
    Ok(Json(BalanceResponse { balance: new_sender_balance }))
}

fn init_app() -> App {
    let mut balances: HashMap<Address, f64> = HashMap::new();
    App { balances: Arc::new(RwLock::new(balances)) }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .configure(Config::figment().merge(("port", 3042)))
        .manage(init_app())
        .mount("/", routes![get_balance, send])
}
