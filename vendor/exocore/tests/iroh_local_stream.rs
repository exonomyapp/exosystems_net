use anyhow::{Context, Result};
use iroh::{Endpoint, RelayMode, endpoint::presets};
use tokio::time::{Duration, timeout};

const ALPN: &[u8] = b"exocore/it/iroh-local/1";

#[tokio::test]
async fn iroh_two_endpoints_dial_and_stream_bytes_local_only() -> Result<()> {
    let ep1 = Endpoint::builder(presets::Minimal)
        .alpns(vec![ALPN.to_vec()])
        .relay_mode(RelayMode::Disabled)
        .bind()
        .await
        .context("bind ep1")?;

    let ep2 = Endpoint::builder(presets::Minimal)
        .alpns(vec![ALPN.to_vec()])
        .relay_mode(RelayMode::Disabled)
        .bind()
        .await
        .context("bind ep2")?;

    let ep2_addr = ep2.addr();
    anyhow::ensure!(
        ep2_addr.relay_urls().next().is_none(),
        "ep2 is configured with relay urls"
    );
    anyhow::ensure!(ep2_addr.ip_addrs().next().is_some(), "ep2 has no ip addrs");

    let server = tokio::spawn(async move {
        let incoming = timeout(Duration::from_secs(10), ep2.accept())
            .await
            .context("timeout waiting for incoming connection")?
            .context("endpoint closed before accepting")?;

        let mut accepting = incoming.accept().context("accept incoming")?;
        let alpn = timeout(Duration::from_secs(10), accepting.alpn())
            .await
            .context("timeout reading alpn")?
            .context("read alpn")?;
        anyhow::ensure!(alpn.as_slice() == ALPN, "unexpected alpn");

        let conn = timeout(Duration::from_secs(10), accepting)
            .await
            .context("timeout finishing accept")?
            .context("finish accept")?;

        let (mut send, mut recv) = timeout(Duration::from_secs(10), conn.accept_bi())
            .await
            .context("timeout waiting for stream")?
            .context("accept bi stream")?;

        let request = timeout(Duration::from_secs(10), recv.read_to_end(64 * 1024))
            .await
            .context("timeout reading request")?
            .context("read request")?;
        anyhow::ensure!(request.as_slice() == b"ping", "unexpected request bytes");

        send.write_all(b"pong").await.context("write response")?;
        send.finish().context("finish response")?;
        let _ = timeout(Duration::from_secs(10), conn.closed()).await;

        ep2.close().await;
        Ok::<(), anyhow::Error>(())
    });

    let conn = timeout(Duration::from_secs(10), ep1.connect(ep2_addr, ALPN))
        .await
        .context("timeout connecting")?
        .context("connect")?;

    let (mut send, mut recv) = timeout(Duration::from_secs(10), conn.open_bi())
        .await
        .context("timeout opening stream")?
        .context("open bi stream")?;

    send.write_all(b"ping").await.context("write request")?;
    send.finish().context("finish request")?;

    let response = timeout(Duration::from_secs(10), recv.read_to_end(64 * 1024))
        .await
        .context("timeout reading response")?
        .context("read response")?;
    anyhow::ensure!(response.as_slice() == b"pong", "unexpected response bytes");
    conn.close(0u32.into(), b"done");

    server.await.context("join server task")??;

    ep1.close().await;
    Ok(())
}
