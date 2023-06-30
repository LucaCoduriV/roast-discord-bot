use std::collections::LinkedList;
use std::sync::{Arc};
use tokio::sync::Mutex;
use reqwest::Client;
use crate::dto::{BotResponse, Message};
use crate::dto::Root;

pub struct RoastBotAi {
    req_client: Client,
    history: Arc<Mutex<LinkedList<Message>>>,
}

impl RoastBotAi {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self {
            req_client: client,
            history: Arc::new(Mutex::new(LinkedList::new())),
        }
    }

    pub async fn send_message(& self, message: &str) -> Option<String> {
        let mut history = self.history.lock().await;
        let vec_history = history.iter().map(|x| x.clone()).collect::<Vec<Message>>();
        let user_message = Message {
            role: "user".to_string(),
            content: message.to_string(),
        };

        let root = Root {
            user_message: user_message.clone(),
            history: vec_history,
            style: "default".to_string(),
        };

        history.push_back(user_message);

        if history.len() > 16 {
            history.pop_front();
        }

        let res = self.req_client.post("https://www.roastedby.ai/api/generate")
            .json(&root)
            .send()
            .await;

        match res {
            Ok(v) => Some(v.json::<BotResponse>().await.expect("Couldn't parse message !").content),
            Err(_) => None,
        }
    }
}

// #[cfg(test)]
// mod test {
//
//     #[tokio::test]
//     async fn send_message_test() {
//         let mut ai = super::RoastBotAi::new();
//         let res = ai.send_message("Hello").await;
//         println!("{:?}", res);
//
//     }
//
//
// }
