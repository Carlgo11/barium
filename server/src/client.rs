use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        RwLock,
        mpsc
    }
};
use padlock;
use barium_shared::{AfkStatus, ToClient, UserHash};
use crate::error::BariumResult;
use rmp_serde;
use rsa;

pub type Clients = Arc<RwLock<HashMap<UserHash, Client>>>;

#[derive(Debug)]
pub struct Client {
    stream: Mutex<mpsc::Sender<Vec<u8>>>,
    key: rsa::RSAPublicKey,
    idle: RwLock<AfkStatus>
}

impl Client {

    pub fn new(stream: &mpsc::Sender<Vec<u8>>, key: rsa::RSAPublicKey, idle: AfkStatus) -> Self {

        Self {
            stream: Mutex::new(stream.clone()),
            key: key,
            idle: RwLock::new(idle)
        }

    }

    pub fn set_idle(&self, idle: AfkStatus) {

        padlock::rw_write_lock(&self.idle, |lock| {
            *lock = idle;
        })

    }

    pub fn get_idle(&self) -> AfkStatus {

        padlock::rw_read_lock(&self.idle, |lock| {
            *lock
        })

    }

    pub fn send_data(&self, to_client: ToClient) -> BariumResult<()> {

        let data = rmp_serde::to_vec(&to_client)?;

        padlock::mutex_lock(&self.stream, |lock| {
            lock.send(data)
        })?;

        Ok(())

    }

    pub fn get_public_key(&self) -> rsa::RSAPublicKey {

        self.key.clone()

    }

}
