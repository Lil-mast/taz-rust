use anyhow::{Context, Result};
use libp2p::{
    identity, mdns::{Mdns, MdnsConfig}, noise, swarm::{Swarm, SwarmBuilder}, tcp, yamux, Multiaddr, PeerId, Transport
};
use log::info;
use tokio::select;

/// Start libp2p mesh. MCP could extend for peer context exchange.
pub async fn start_mesh() -> Result<()> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    info!("Local peer ID: {:?}", peer_id);

    let transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&id_keys).context("Noise config failed")?)
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = Mdns::new(MdnsConfig::default()).await?;

    let mut swarm = SwarmBuilder::with_existing_identity(id_keys)
        .with_tokio()
        .with_other_transport(transport)?
        .with_behaviour(|_| behaviour)?
        .build()?;

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

    loop {
        select! {
            event = swarm.select_next_some() => {
                info!("Mesh event: {:?}", event);
            }
        }
    }
}