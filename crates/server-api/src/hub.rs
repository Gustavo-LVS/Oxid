//! Distribuição de eventos para as sessões de gateway conectadas.
//!
//! Um nó só: canal broadcast em memória. Para vários nós, trocar por Redis
//! pub/sub mantendo esta interface.

use oxid_proto::gateway::ServerEvent;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum HubEvent {
    /// Evento para todos os membros conectados de um servidor.
    ToServer { server_id: Uuid, event: ServerEvent },
    /// Usuário entrou num servidor: a sessão dele passa a receber eventos de lá.
    MemberJoined { server_id: Uuid, user_id: Uuid },
}

#[derive(Clone)]
pub struct Hub(broadcast::Sender<HubEvent>);

impl Default for Hub {
    fn default() -> Self {
        Self(broadcast::channel(1024).0)
    }
}

impl Hub {
    pub fn publish(&self, ev: HubEvent) {
        // Sem ninguém conectado o envio falha, e tudo bem.
        let _ = self.0.send(ev);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<HubEvent> {
        self.0.subscribe()
    }
}
