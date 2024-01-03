use crate::{Agent, Agents};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub async fn agent_connection(ws: WebSocket, id: String, agents: Agents, mut agent: Agent) {
    let (agent_ws_sender, mut agent_ws_rcv) = ws.split();
    let (agent_sender, agent_rcv) = mpsc::unbounded_channel();

    let agent_rcv = UnboundedReceiverStream::new(agent_rcv);
    tokio::task::spawn(agent_rcv.forward(agent_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    agent.sender = Some(agent_sender);
    agents.write().await.insert(id.clone(), agent);

    println!("{} connected", id);

    while let Some(result) = agent_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        agent_msg(&id, msg, &agents).await;
    }

    agents.write().await.remove(&id);
    println!("{} disconnected", id);
}

async fn agent_msg(id: &str, msg: Message, _agents: &Agents) {
    println!("received message from {}: {:?}", id, msg);
}
