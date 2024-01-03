use crate::{ws, Agent, Agents, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: usize,
    topic: String,
}

#[derive(Deserialize)]
pub struct TopicActionRequest {
    topic: String,
    agent_id: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: serde_json::Value,
}

pub async fn publish_handler(body: Event, agents: Agents) -> Result<impl Reply> {
    agents
        .read()
        .await
        .iter()
        .filter(|(_, agent)| match body.user_id {
            Some(v) => agent.user_id == v,
            None => true,
        })
        .filter(|(_, agent)| agent.topics.contains(&body.topic))
        .for_each(|(_, agent)| {
            if let Some(sender) = &agent.sender {
                let _ = sender.send(Ok(Message::text(body.message.to_string())));
            }
        });

    Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, agents: Agents) -> Result<impl Reply> {
    let user_id = body.user_id;
    let topic = body.topic; // Capture the entry topic
    let uuid = Uuid::new_v4().as_simple().to_string();

    register_agent(uuid.clone(), user_id, topic, agents).await; // Pass the entry topic
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

async fn register_agent(id: String, user_id: usize, topic: String, agents: Agents) {
    agents.write().await.insert(
        id,
        Agent {
            user_id,
            topics: vec![topic],
            sender: None,
        },
    );
}

pub async fn unregister_handler(id: String, agents: Agents) -> Result<impl Reply> {
    agents.write().await.remove(&id);
    Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, id: String, agents: Agents) -> Result<impl Reply> {
    let agent = agents.read().await.get(&id).cloned();
    match agent {
        Some(c) => Ok(ws.on_upgrade(move |socket| ws::agent_connection(socket, id, agents, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}

pub async fn add_topic(body: TopicActionRequest, agents: Agents) -> Result<impl Reply> {
    let mut agents_write = agents.write().await;
    if let Some(agent) = agents_write.get_mut(&body.agent_id) {
        agent.topics.push(body.topic);
    }
    Ok(warp::reply::with_status(
        "Added topic successfully",
        StatusCode::OK,
    ))
}

pub async fn remove_topic(body: TopicActionRequest, agents: Agents) -> Result<impl Reply> {
    let mut agents_write = agents.write().await;
    if let Some(agent) = agents_write.get_mut(&body.agent_id) {
        agent.topics.retain(|t| t != &body.topic);
    }
    Ok(warp::reply::with_status(
        "Removed topic successfully",
        StatusCode::OK,
    ))
}
