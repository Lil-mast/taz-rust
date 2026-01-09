use anyhow::Result;
use libp2p::{
    identity, noise, tcp, yamux, Multiaddr, PeerId, Transport, swarm::Swarm
};
use libp2p::futures::StreamExt;
use log::info;
use tokio::select;

/// Start libp2p mesh. MCP could extend for peer context exchange.
pub async fn start_mesh() -> Result<()> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    info!("Local peer ID: {:?}", peer_id);

    let transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&id_keys).map_err(|e| anyhow::anyhow!("Noise config failed: {}", e))?)
        .multiplex(yamux::Config::default())
        .boxed();

    // Create a simple swarm with a dummy behaviour
    let mut swarm = Swarm::new(transport, libp2p::swarm::dummy::Behaviour, peer_id, libp2p::swarm::Config::with_tokio_executor());

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

    loop {
        select! {
            event = swarm.select_next_some() => {
                info!("Mesh event: {:?}", event);
            }
        }
    }
}