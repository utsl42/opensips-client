use jsonrpsee::http_client::HttpClientBuilder;
use opensips_client::*;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let url = "http://127.0.0.1:38888/mi";

	let client = HttpClientBuilder::default()
		.build(url)?;
	let records: RegListResponse = client.reg_list().await?;
	info!("records: {:?}", records);
	let first = records.records[0].clone();
	let v = client.reg_list_record(first.aor, first.binding, first.registrar).await?;
	info!("v: {:?}", v);

	Ok(())
}
