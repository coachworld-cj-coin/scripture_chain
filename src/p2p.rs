use libp2p::{
    gossipsub,
    identity,
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use futures::StreamExt;

pub async fn start_p2p() {

    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(local_key.public());

    println!("Local node id: {:?}", peer_id);

    let config = gossipsub::Config::default();

    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key),
        config,
    ).unwrap();

    let topic = gossipsub::IdentTopic::new("scripture-blocks");

    gossipsub.subscribe(&topic).unwrap();

    let mut swarm = Swarm::new(
        libp2p::development_transport(local_key).await.unwrap(),
        gossipsub,
        peer_id,
    );

    loop {

        match swarm.select_next_some().await {

            SwarmEvent::Behaviour(event) => {
                println!("Event: {:?}", event);
            }

            _ => {}
        }
    }
}
