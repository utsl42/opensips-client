use jsonrpsee::http_client::HttpClientBuilder;
use opensips_client::*;
use tracing::info;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::net::SocketAddr;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let url = "http://127.0.0.1:28888/mi";

	let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 10000))).await?;
	let notifier = UdpNotificationReceiver { socket };
	let (tx, mut rx) = mpsc::channel(1);
	tokio::spawn(notifier.run(move |n| {
		let tx = tx.clone();
		async move {
			tx.send(n).await;
			()
		}
	}));

	let client = HttpClientBuilder::default()
		.build(url)?;
	client.event_subscribe("E_UA_SESSION".to_string(), "udp:127.0.0.1:10000".to_string()).await?;

	loop {
		if let Some(thing) = rx.recv().await {
			info!("received: {:?}", thing);
		}
	}

	// Ok(())
}
