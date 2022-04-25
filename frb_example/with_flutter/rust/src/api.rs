// use anyhow::Result;
use clap::{App, AppSettings, Arg};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use tokio::time::Duration;
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::{MediaEngine, MIME_TYPE_OPUS, MIME_TYPE_VP8};
use webrtc::api::APIBuilder;
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication;
use webrtc::rtp_transceiver::rtp_codec::{
    RTCRtpCodecCapability, RTCRtpCodecParameters, RTPCodecType,
};
use webrtc::rtp_transceiver::rtp_receiver::RTCRtpReceiver;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::track::track_local::{TrackLocal, TrackLocalWriter};
use webrtc::track::track_remote::TrackRemote;


use anyhow::{anyhow, Result};

use flutter_rust_bridge::ZeroCopyBuffer;

use std::{thread, time};

//
// NOTE: Please look at https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/rust/src/api.rs
// to see more types that this code generator can generate.
//








/// encode encodes the input in base64
/// It can optionally zip the input before encoding
pub fn encode(b: &str) -> String {
    //if COMPRESS {
    //    b = zip(b)
    //}

    base64::encode(b)
}

/// decode decodes the input from base64
/// It can optionally unzip the input after decoding
pub fn decode(s: &str) -> Result<String> {
    let b = base64::decode(s)?;

    //if COMPRESS {
    //    b = unzip(b)
    //}

    let s = String::from_utf8(b)?;
    Ok(s)
}






pub fn draw_mandelbrot(
    image_size: Size,
    zoom_point: Point,
    scale: f64,
    num_threads: i32,
) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    // Just an example that generates "complicated" images ;)
    let image = crate::off_topic_code::mandelbrot(image_size, zoom_point, scale, num_threads)?;
    Ok(ZeroCopyBuffer(image))
}

pub fn passing_complex_structs(root: TreeNode) -> Result<String> {
    Ok(format!(
        "Hi this string is from Rust. I received a complex struct: {:?}",
        root
    ))
}

#[tokio::main]
async fn foo() -> Result<String> {
    // Everything below is the WebRTC-rs API! Thanks for using it ❤️.

    // Create a MediaEngine object to configure the supported codec
    let mut m = MediaEngine::default();

    m.register_codec(
        RTCRtpCodecParameters {
            capability: RTCRtpCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_owned(),
                ..Default::default()
            },
            payload_type: 120,
            ..Default::default()
        },
        RTPCodecType::Audio,
    )?;
    
    // Create a InterceptorRegistry. This is the user configurable RTP/RTCP Pipeline.
    // This provides NACKs, RTCP Reports and other features. If you use `webrtc.NewPeerConnection`
    // this is enabled by default. If you are manually managing You MUST create a InterceptorRegistry
    // for each PeerConnection.
    let mut registry = Registry::new();

    // Use the default set of Interceptors
    registry = register_default_interceptors(registry, &mut m)?;

    // Create the API object with the MediaEngine
    let api = APIBuilder::new()
        .with_media_engine(m)
        .with_interceptor_registry(registry)
        .build();

    // Prepare the configuration
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    // Create a new RTCPeerConnection
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);
    let mut output_tracks = HashMap::new();
    let mut media = vec![];
    media.push("audio");
    for s in media {
        let output_track = Arc::new(TrackLocalStaticRTP::new(
            RTCRtpCodecCapability {
                mime_type: if s == "video" {
                    MIME_TYPE_VP8.to_owned()
                } else {
                    MIME_TYPE_OPUS.to_owned()
                },
                ..Default::default()
            },
            format!("track-{}", s),
            "webrtc-rs".to_owned(),
        ));

        // Add this newly created track to the PeerConnection
        let rtp_sender = peer_connection
            .add_track(Arc::clone(&output_track) as Arc<dyn TrackLocal + Send + Sync>)
            .await?;

        // Read incoming RTCP packets
        // Before these packets are returned they are processed by interceptors. For things
        // like NACK this needs to be called.
        let m = s.to_owned();
        tokio::spawn(async move {
            let mut rtcp_buf = vec![0u8; 1500];
            while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
            println!("{} rtp_sender.read loop exit", m);
            Result::<()>::Ok(())
        });
        

        let ten_millis = time::Duration::from_millis(5000);
        let now = time::Instant::now();
        
        thread::sleep(ten_millis);        

        output_tracks.insert(s.to_owned(), output_track);
    }









    // Wait for the offer to be pasted
    let line = "eyJ0eXBlIjoib2ZmZXIiLCJzZHAiOiJ2PTBcclxubz0tIDcxNzc2MDEzNzY5OTI0OTA2MjAgMiBJTiBJUDQgMTI3LjAuMC4xXHJcbnM9LVxyXG50PTAgMFxyXG5hPWdyb3VwOkJVTkRMRSAwIDFcclxuYT1leHRtYXAtYWxsb3ctbWl4ZWRcclxuYT1tc2lkLXNlbWFudGljOiBXTVMgTlFaVnFKTTVSaHdyeTY3Z2hJOFBpM3pKdmV6akRHRjNaWmw3XHJcbm09YXVkaW8gNTI1NzYgVURQL1RMUy9SVFAvU0FWUEYgMTExIDYzIDEwMyAxMDQgOSAwIDggMTA2IDEwNSAxMyAxMTAgMTEyIDExMyAxMjZcclxuYz1JTiBJUDQgNTguMTUyLjE1My4xMDZcclxuYT1ydGNwOjkgSU4gSVA0IDAuMC4wLjBcclxuYT1jYW5kaWRhdGU6MTU3Mjk0MzQ2NSAxIHVkcCAyMTIyMjYwMjIzIDE5Mi4xNjguMTg4LjMgNTI1NzYgdHlwIGhvc3QgZ2VuZXJhdGlvbiAwIG5ldHdvcmstaWQgMSBuZXR3b3JrLWNvc3QgMTBcclxuYT1jYW5kaWRhdGU6MzY5OTk3ODQ2MSAxIHVkcCAxNjg2MDUyNjA3IDU4LjE1Mi4xNTMuMTA2IDUyNTc2IHR5cCBzcmZseCByYWRkciAxOTIuMTY4LjE4OC4zIHJwb3J0IDUyNTc2IGdlbmVyYXRpb24gMCBuZXR3b3JrLWlkIDEgbmV0d29yay1jb3N0IDEwXHJcbmE9Y2FuZGlkYXRlOjMyMzE0NDM0NSAxIHRjcCAxNTE4MjgwNDQ3IDE5Mi4xNjguMTg4LjMgOSB0eXAgaG9zdCB0Y3B0eXBlIGFjdGl2ZSBnZW5lcmF0aW9uIDAgbmV0d29yay1pZCAxIG5ldHdvcmstY29zdCAxMFxyXG5hPWljZS11ZnJhZzpLUUMxXHJcbmE9aWNlLXB3ZDphTmZSS0ttNDlIdXpCOXRBMzBrL29QYllcclxuYT1pY2Utb3B0aW9uczp0cmlja2xlXHJcbmE9ZmluZ2VycHJpbnQ6c2hhLTI1NiA2QTpFQTo4MzpBNjo2MTo0MzpBNjpFOToyQTpFMDo0MTo5Qzo4NTo1MzowNjpFRTo1ODo2Nzo2QToxQjpENDowOTpFRjo1NDpGNzo0NjpEQzo5Nzo4NzpEQTo5MzpEQlxyXG5hPXNldHVwOmFjdHBhc3NcclxuYT1taWQ6MFxyXG5hPWV4dG1hcDoxIHVybjppZXRmOnBhcmFtczpydHAtaGRyZXh0OnNzcmMtYXVkaW8tbGV2ZWxcclxuYT1leHRtYXA6MiBodHRwOi8vd3d3LndlYnJ0Yy5vcmcvZXhwZXJpbWVudHMvcnRwLWhkcmV4dC9hYnMtc2VuZC10aW1lXHJcbmE9ZXh0bWFwOjMgaHR0cDovL3d3dy5pZXRmLm9yZy9pZC9kcmFmdC1ob2xtZXItcm1jYXQtdHJhbnNwb3J0LXdpZGUtY2MtZXh0ZW5zaW9ucy0wMVxyXG5hPWV4dG1hcDo0IHVybjppZXRmOnBhcmFtczpydHAtaGRyZXh0OnNkZXM6bWlkXHJcbmE9c2VuZHJlY3ZcclxuYT1tc2lkOk5RWlZxSk01Umh3cnk2N2doSThQaTN6SnZlempER0YzWlpsNyBlODc1NTM0Ny0yZGM2LTQwNTQtOWY1Yy03NTQ4Y2NlMTY2Y2ZcclxuYT1ydGNwLW11eFxyXG5hPXJ0cG1hcDoxMTEgb3B1cy80ODAwMC8yXHJcbmE9cnRjcC1mYjoxMTEgdHJhbnNwb3J0LWNjXHJcbmE9Zm10cDoxMTEgbWlucHRpbWU9MTA7dXNlaW5iYW5kZmVjPTFcclxuYT1ydHBtYXA6NjMgcmVkLzQ4MDAwLzJcclxuYT1mbXRwOjYzIDExMS8xMTFcclxuYT1ydHBtYXA6MTAzIElTQUMvMTYwMDBcclxuYT1ydHBtYXA6MTA0IElTQUMvMzIwMDBcclxuYT1ydHBtYXA6OSBHNzIyLzgwMDBcclxuYT1ydHBtYXA6MCBQQ01VLzgwMDBcclxuYT1ydHBtYXA6OCBQQ01BLzgwMDBcclxuYT1ydHBtYXA6MTA2IENOLzMyMDAwXHJcbmE9cnRwbWFwOjEwNSBDTi8xNjAwMFxyXG5hPXJ0cG1hcDoxMyBDTi84MDAwXHJcbmE9cnRwbWFwOjExMCB0ZWxlcGhvbmUtZXZlbnQvNDgwMDBcclxuYT1ydHBtYXA6MTEyIHRlbGVwaG9uZS1ldmVudC8zMjAwMFxyXG5hPXJ0cG1hcDoxMTMgdGVsZXBob25lLWV2ZW50LzE2MDAwXHJcbmE9cnRwbWFwOjEyNiB0ZWxlcGhvbmUtZXZlbnQvODAwMFxyXG5hPXNzcmM6MzYxOTExNjg0NiBjbmFtZTpkenRMVG90aHMxYUtkSnVUXHJcbmE9c3NyYzozNjE5MTE2ODQ2IG1zaWQ6TlFaVnFKTTVSaHdyeTY3Z2hJOFBpM3pKdmV6akRHRjNaWmw3IGU4NzU1MzQ3LTJkYzYtNDA1NC05ZjVjLTc1NDhjY2UxNjZjZlxyXG5hPXNzcmM6MzYxOTExNjg0NiBtc2xhYmVsOk5RWlZxSk01Umh3cnk2N2doSThQaTN6SnZlempER0YzWlpsN1xyXG5hPXNzcmM6MzYxOTExNjg0NiBsYWJlbDplODc1NTM0Ny0yZGM2LTQwNTQtOWY1Yy03NTQ4Y2NlMTY2Y2ZcclxubT12aWRlbyA2MjAwOSBVRFAvVExTL1JUUC9TQVZQRiA5NiA5NyA5OCA5OSAxMDAgMTAxIDEwMiAxMjEgMTI3IDEyMCAxMjUgMTA3IDEwOCAxMDkgMTI0IDExOSAxMjMgMTE3IDM1IDM2IDExNCAxMTUgMTE2IDYyIDExOFxyXG5jPUlOIElQNCA1OC4xNTIuMTUzLjEwNlxyXG5hPXJ0Y3A6OSBJTiBJUDQgMC4wLjAuMFxyXG5hPWNhbmRpZGF0ZToxNTcyOTQzNDY1IDEgdWRwIDIxMjIyNjAyMjMgMTkyLjE2OC4xODguMyA2MjAwOSB0eXAgaG9zdCBnZW5lcmF0aW9uIDAgbmV0d29yay1pZCAxIG5ldHdvcmstY29zdCAxMFxyXG5hPWNhbmRpZGF0ZTozNjk5OTc4NDYxIDEgdWRwIDE2ODYwNTI2MDcgNTguMTUyLjE1My4xMDYgNjIwMDkgdHlwIHNyZmx4IHJhZGRyIDE5Mi4xNjguMTg4LjMgcnBvcnQgNjIwMDkgZ2VuZXJhdGlvbiAwIG5ldHdvcmstaWQgMSBuZXR3b3JrLWNvc3QgMTBcclxuYT1jYW5kaWRhdGU6MzIzMTQ0MzQ1IDEgdGNwIDE1MTgyODA0NDcgMTkyLjE2OC4xODguMyA5IHR5cCBob3N0IHRjcHR5cGUgYWN0aXZlIGdlbmVyYXRpb24gMCBuZXR3b3JrLWlkIDEgbmV0d29yay1jb3N0IDEwXHJcbmE9aWNlLXVmcmFnOktRQzFcclxuYT1pY2UtcHdkOmFOZlJLS200OUh1ekI5dEEzMGsvb1BiWVxyXG5hPWljZS1vcHRpb25zOnRyaWNrbGVcclxuYT1maW5nZXJwcmludDpzaGEtMjU2IDZBOkVBOjgzOkE2OjYxOjQzOkE2OkU5OjJBOkUwOjQxOjlDOjg1OjUzOjA2OkVFOjU4OjY3OjZBOjFCOkQ0OjA5OkVGOjU0OkY3OjQ2OkRDOjk3Ojg3OkRBOjkzOkRCXHJcbmE9c2V0dXA6YWN0cGFzc1xyXG5hPW1pZDoxXHJcbmE9ZXh0bWFwOjE0IHVybjppZXRmOnBhcmFtczpydHAtaGRyZXh0OnRvZmZzZXRcclxuYT1leHRtYXA6MiBodHRwOi8vd3d3LndlYnJ0Yy5vcmcvZXhwZXJpbWVudHMvcnRwLWhkcmV4dC9hYnMtc2VuZC10aW1lXHJcbmE9ZXh0bWFwOjEzIHVybjozZ3BwOnZpZGVvLW9yaWVudGF0aW9uXHJcbmE9ZXh0bWFwOjMgaHR0cDovL3d3dy5pZXRmLm9yZy9pZC9kcmFmdC1ob2xtZXItcm1jYXQtdHJhbnNwb3J0LXdpZGUtY2MtZXh0ZW5zaW9ucy0wMVxyXG5hPWV4dG1hcDo1IGh0dHA6Ly93d3cud2VicnRjLm9yZy9leHBlcmltZW50cy9ydHAtaGRyZXh0L3BsYXlvdXQtZGVsYXlcclxuYT1leHRtYXA6NiBodHRwOi8vd3d3LndlYnJ0Yy5vcmcvZXhwZXJpbWVudHMvcnRwLWhkcmV4dC92aWRlby1jb250ZW50LXR5cGVcclxuYT1leHRtYXA6NyBodHRwOi8vd3d3LndlYnJ0Yy5vcmcvZXhwZXJpbWVudHMvcnRwLWhkcmV4dC92aWRlby10aW1pbmdcclxuYT1leHRtYXA6OCBodHRwOi8vd3d3LndlYnJ0Yy5vcmcvZXhwZXJpbWVudHMvcnRwLWhkcmV4dC9jb2xvci1zcGFjZVxyXG5hPWV4dG1hcDo0IHVybjppZXRmOnBhcmFtczpydHAtaGRyZXh0OnNkZXM6bWlkXHJcbmE9ZXh0bWFwOjEwIHVybjppZXRmOnBhcmFtczpydHAtaGRyZXh0OnNkZXM6cnRwLXN0cmVhbS1pZFxyXG5hPWV4dG1hcDoxMSB1cm46aWV0ZjpwYXJhbXM6cnRwLWhkcmV4dDpzZGVzOnJlcGFpcmVkLXJ0cC1zdHJlYW0taWRcclxuYT1zZW5kcmVjdlxyXG5hPW1zaWQ6TlFaVnFKTTVSaHdyeTY3Z2hJOFBpM3pKdmV6akRHRjNaWmw3IDA3OTdhMGZkLWEwNDItNDBmNy05ZTFiLTBkYmU0MzkwMzllOVxyXG5hPXJ0Y3AtbXV4XHJcbmE9cnRjcC1yc2l6ZVxyXG5hPXJ0cG1hcDo5NiBWUDgvOTAwMDBcclxuYT1ydGNwLWZiOjk2IGdvb2ctcmVtYlxyXG5hPXJ0Y3AtZmI6OTYgdHJhbnNwb3J0LWNjXHJcbmE9cnRjcC1mYjo5NiBjY20gZmlyXHJcbmE9cnRjcC1mYjo5NiBuYWNrXHJcbmE9cnRjcC1mYjo5NiBuYWNrIHBsaVxyXG5hPXJ0cG1hcDo5NyBydHgvOTAwMDBcclxuYT1mbXRwOjk3IGFwdD05NlxyXG5hPXJ0cG1hcDo5OCBWUDkvOTAwMDBcclxuYT1ydGNwLWZiOjk4IGdvb2ctcmVtYlxyXG5hPXJ0Y3AtZmI6OTggdHJhbnNwb3J0LWNjXHJcbmE9cnRjcC1mYjo5OCBjY20gZmlyXHJcbmE9cnRjcC1mYjo5OCBuYWNrXHJcbmE9cnRjcC1mYjo5OCBuYWNrIHBsaVxyXG5hPWZtdHA6OTggcHJvZmlsZS1pZD0wXHJcbmE9cnRwbWFwOjk5IHJ0eC85MDAwMFxyXG5hPWZtdHA6OTkgYXB0PTk4XHJcbmE9cnRwbWFwOjEwMCBWUDkvOTAwMDBcclxuYT1ydGNwLWZiOjEwMCBnb29nLXJlbWJcclxuYT1ydGNwLWZiOjEwMCB0cmFuc3BvcnQtY2NcclxuYT1ydGNwLWZiOjEwMCBjY20gZmlyXHJcbmE9cnRjcC1mYjoxMDAgbmFja1xyXG5hPXJ0Y3AtZmI6MTAwIG5hY2sgcGxpXHJcbmE9Zm10cDoxMDAgcHJvZmlsZS1pZD0yXHJcbmE9cnRwbWFwOjEwMSBydHgvOTAwMDBcclxuYT1mbXRwOjEwMSBhcHQ9MTAwXHJcbmE9cnRwbWFwOjEwMiBIMjY0LzkwMDAwXHJcbmE9cnRjcC1mYjoxMDIgZ29vZy1yZW1iXHJcbmE9cnRjcC1mYjoxMDIgdHJhbnNwb3J0LWNjXHJcbmE9cnRjcC1mYjoxMDIgY2NtIGZpclxyXG5hPXJ0Y3AtZmI6MTAyIG5hY2tcclxuYT1ydGNwLWZiOjEwMiBuYWNrIHBsaVxyXG5hPWZtdHA6MTAyIGxldmVsLWFzeW1tZXRyeS1hbGxvd2VkPTE7cGFja2V0aXphdGlvbi1tb2RlPTE7cHJvZmlsZS1sZXZlbC1pZD00MjAwMWZcclxuYT1ydHBtYXA6MTIxIHJ0eC85MDAwMFxyXG5hPWZtdHA6MTIxIGFwdD0xMDJcclxuYT1ydHBtYXA6MTI3IEgyNjQvOTAwMDBcclxuYT1ydGNwLWZiOjEyNyBnb29nLXJlbWJcclxuYT1ydGNwLWZiOjEyNyB0cmFuc3BvcnQtY2NcclxuYT1ydGNwLWZiOjEyNyBjY20gZmlyXHJcbmE9cnRjcC1mYjoxMjcgbmFja1xyXG5hPXJ0Y3AtZmI6MTI3IG5hY2sgcGxpXHJcbmE9Zm10cDoxMjcgbGV2ZWwtYXN5bW1ldHJ5LWFsbG93ZWQ9MTtwYWNrZXRpemF0aW9uLW1vZGU9MDtwcm9maWxlLWxldmVsLWlkPTQyMDAxZlxyXG5hPXJ0cG1hcDoxMjAgcnR4LzkwMDAwXHJcbmE9Zm10cDoxMjAgYXB0PTEyN1xyXG5hPXJ0cG1hcDoxMjUgSDI2NC85MDAwMFxyXG5hPXJ0Y3AtZmI6MTI1IGdvb2ctcmVtYlxyXG5hPXJ0Y3AtZmI6MTI1IHRyYW5zcG9ydC1jY1xyXG5hPXJ0Y3AtZmI6MTI1IGNjbSBmaXJcclxuYT1ydGNwLWZiOjEyNSBuYWNrXHJcbmE9cnRjcC1mYjoxMjUgbmFjayBwbGlcclxuYT1mbXRwOjEyNSBsZXZlbC1hc3ltbWV0cnktYWxsb3dlZD0xO3BhY2tldGl6YXRpb24tbW9kZT0xO3Byb2ZpbGUtbGV2ZWwtaWQ9NDJlMDFmXHJcbmE9cnRwbWFwOjEwNyBydHgvOTAwMDBcclxuYT1mbXRwOjEwNyBhcHQ9MTI1XHJcbmE9cnRwbWFwOjEwOCBIMjY0LzkwMDAwXHJcbmE9cnRjcC1mYjoxMDggZ29vZy1yZW1iXHJcbmE9cnRjcC1mYjoxMDggdHJhbnNwb3J0LWNjXHJcbmE9cnRjcC1mYjoxMDggY2NtIGZpclxyXG5hPXJ0Y3AtZmI6MTA4IG5hY2tcclxuYT1ydGNwLWZiOjEwOCBuYWNrIHBsaVxyXG5hPWZtdHA6MTA4IGxldmVsLWFzeW1tZXRyeS1hbGxvd2VkPTE7cGFja2V0aXphdGlvbi1tb2RlPTA7cHJvZmlsZS1sZXZlbC1pZD00MmUwMWZcclxuYT1ydHBtYXA6MTA5IHJ0eC85MDAwMFxyXG5hPWZtdHA6MTA5IGFwdD0xMDhcclxuYT1ydHBtYXA6MTI0IEgyNjQvOTAwMDBcclxuYT1ydGNwLWZiOjEyNCBnb29nLXJlbWJcclxuYT1ydGNwLWZiOjEyNCB0cmFuc3BvcnQtY2NcclxuYT1ydGNwLWZiOjEyNCBjY20gZmlyXHJcbmE9cnRjcC1mYjoxMjQgbmFja1xyXG5hPXJ0Y3AtZmI6MTI0IG5hY2sgcGxpXHJcbmE9Zm10cDoxMjQgbGV2ZWwtYXN5bW1ldHJ5LWFsbG93ZWQ9MTtwYWNrZXRpemF0aW9uLW1vZGU9MTtwcm9maWxlLWxldmVsLWlkPTRkMDAxZlxyXG5hPXJ0cG1hcDoxMTkgcnR4LzkwMDAwXHJcbmE9Zm10cDoxMTkgYXB0PTEyNFxyXG5hPXJ0cG1hcDoxMjMgSDI2NC85MDAwMFxyXG5hPXJ0Y3AtZmI6MTIzIGdvb2ctcmVtYlxyXG5hPXJ0Y3AtZmI6MTIzIHRyYW5zcG9ydC1jY1xyXG5hPXJ0Y3AtZmI6MTIzIGNjbSBmaXJcclxuYT1ydGNwLWZiOjEyMyBuYWNrXHJcbmE9cnRjcC1mYjoxMjMgbmFjayBwbGlcclxuYT1mbXRwOjEyMyBsZXZlbC1hc3ltbWV0cnktYWxsb3dlZD0xO3BhY2tldGl6YXRpb24tbW9kZT0wO3Byb2ZpbGUtbGV2ZWwtaWQ9NGQwMDFmXHJcbmE9cnRwbWFwOjExNyBydHgvOTAwMDBcclxuYT1mbXRwOjExNyBhcHQ9MTIzXHJcbmE9cnRwbWFwOjM1IEFWMS85MDAwMFxyXG5hPXJ0Y3AtZmI6MzUgZ29vZy1yZW1iXHJcbmE9cnRjcC1mYjozNSB0cmFuc3BvcnQtY2NcclxuYT1ydGNwLWZiOjM1IGNjbSBmaXJcclxuYT1ydGNwLWZiOjM1IG5hY2tcclxuYT1ydGNwLWZiOjM1IG5hY2sgcGxpXHJcbmE9cnRwbWFwOjM2IHJ0eC85MDAwMFxyXG5hPWZtdHA6MzYgYXB0PTM1XHJcbmE9cnRwbWFwOjExNCBIMjY0LzkwMDAwXHJcbmE9cnRjcC1mYjoxMTQgZ29vZy1yZW1iXHJcbmE9cnRjcC1mYjoxMTQgdHJhbnNwb3J0LWNjXHJcbmE9cnRjcC1mYjoxMTQgY2NtIGZpclxyXG5hPXJ0Y3AtZmI6MTE0IG5hY2tcclxuYT1ydGNwLWZiOjExNCBuYWNrIHBsaVxyXG5hPWZtdHA6MTE0IGxldmVsLWFzeW1tZXRyeS1hbGxvd2VkPTE7cGFja2V0aXphdGlvbi1tb2RlPTE7cHJvZmlsZS1sZXZlbC1pZD02NDAwMzJcclxuYT1ydHBtYXA6MTE1IHJ0eC85MDAwMFxyXG5hPWZtdHA6MTE1IGFwdD0xMTRcclxuYT1ydHBtYXA6MTE2IHJlZC85MDAwMFxyXG5hPXJ0cG1hcDo2MiBydHgvOTAwMDBcclxuYT1mbXRwOjYyIGFwdD0xMTZcclxuYT1ydHBtYXA6MTE4IHVscGZlYy85MDAwMFxyXG5hPXNzcmMtZ3JvdXA6RklEIDQwNzY3ODc1MTAgNDEyNTc2MDQyOFxyXG5hPXNzcmM6NDA3Njc4NzUxMCBjbmFtZTpkenRMVG90aHMxYUtkSnVUXHJcbmE9c3NyYzo0MDc2Nzg3NTEwIG1zaWQ6TlFaVnFKTTVSaHdyeTY3Z2hJOFBpM3pKdmV6akRHRjNaWmw3IDA3OTdhMGZkLWEwNDItNDBmNy05ZTFiLTBkYmU0MzkwMzllOVxyXG5hPXNzcmM6NDA3Njc4NzUxMCBtc2xhYmVsOk5RWlZxSk01Umh3cnk2N2doSThQaTN6SnZlempER0YzWlpsN1xyXG5hPXNzcmM6NDA3Njc4NzUxMCBsYWJlbDowNzk3YTBmZC1hMDQyLTQwZjctOWUxYi0wZGJlNDM5MDM5ZTlcclxuYT1zc3JjOjQxMjU3NjA0MjggY25hbWU6ZHp0TFRvdGhzMWFLZEp1VFxyXG5hPXNzcmM6NDEyNTc2MDQyOCBtc2lkOk5RWlZxSk01Umh3cnk2N2doSThQaTN6SnZlempER0YzWlpsNyAwNzk3YTBmZC1hMDQyLTQwZjctOWUxYi0wZGJlNDM5MDM5ZTlcclxuYT1zc3JjOjQxMjU3NjA0MjggbXNsYWJlbDpOUVpWcUpNNVJod3J5NjdnaEk4UGkzekp2ZXpqREdGM1pabDdcclxuYT1zc3JjOjQxMjU3NjA0MjggbGFiZWw6MDc5N2EwZmQtYTA0Mi00MGY3LTllMWItMGRiZTQzOTAzOWU5XHJcbiJ9";
    let desc_data = decode(line)?;
    let offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;

    // Set the remote SessionDescription
    peer_connection.set_remote_description(offer).await?;

    // Set a handler for when a new remote track starts, this handler copies inbound RTP packets,
    // replaces the SSRC and sends them back
    let pc = Arc::downgrade(&peer_connection);
    peer_connection
        .on_track(Box::new(
            move |track: Option<Arc<TrackRemote>>, _receiver: Option<Arc<RTCRtpReceiver>>| {
                if let Some(track) = track {
                    // Send a PLI on an interval so that the publisher is pushing a keyframe every rtcpPLIInterval
                    // This is a temporary fix until we implement incoming RTCP events, then we would push a PLI only when a viewer requests it
                    let media_ssrc = track.ssrc();

                    if track.kind() == RTPCodecType::Video {
                        let pc2 = pc.clone();
                        tokio::spawn(async move {
                            let mut result = Result::<usize>::Ok(0);
                            while result.is_ok() {
                                let timeout = tokio::time::sleep(Duration::from_secs(3));
                                tokio::pin!(timeout);

                                tokio::select! {
                                    _ = timeout.as_mut() =>{
                                        if let Some(pc) = pc2.upgrade(){
                                            result = pc.write_rtcp(&[Box::new(PictureLossIndication{
                                                    sender_ssrc: 0,
                                                    media_ssrc,
                                            })]).await.map_err(Into::into);
                                        }else{
                                            break;
                                        }
                                    }
                                };
                            }
                        });
                    }

                    let kind = if track.kind() == RTPCodecType::Audio {
                        "audio"
                    } else {
                        "video"
                    };
                    let output_track = if let Some(output_track) = output_tracks.get(kind) {
                        Arc::clone(output_track)
                    } else {
                        println!("output_track not found for type = {}", kind);
                        return Box::pin(async {});
                    };

                    let output_track2 = Arc::clone(&output_track);
                    tokio::spawn(async move {
                        println!(
                            "Track has started, of type {}: {}",
                            track.payload_type(),
                            track.codec().await.capability.mime_type
                        );
                        // Read RTP packets being sent to webrtc-rs
                        while let Ok((rtp, _)) = track.read_rtp().await {
                            if let Err(err) = output_track2.write_rtp(&rtp).await {
                                println!("output track write_rtp got error: {}", err);
                                break;
                            }
                        }

                        println!(
                            "on_track finished, of type {}: {}",
                            track.payload_type(),
                            track.codec().await.capability.mime_type
                        );
                    });
                }
                Box::pin(async {})
            },
        ))
        .await;

    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel::<()>(1);

    // Set the handler for Peer connection state
    // This will notify you when the peer has connected/disconnected
    peer_connection
        .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
            println!("Peer Connection State has changed: {}", s);

            if s == RTCPeerConnectionState::Failed {
                // Wait until PeerConnection has had no network activity for 30 seconds or another failure. It may be reconnected using an ICE Restart.
                // Use webrtc.PeerConnectionStateDisconnected if you are interested in detecting faster timeout.
                // Note that the PeerConnection may come back from PeerConnectionStateDisconnected.
                println!("Peer Connection has gone to failed exiting");
                let _ = done_tx.try_send(());
            }

            Box::pin(async {})
        }))
        .await;

    // Create an answer
    let answer = peer_connection.create_answer(None).await?;

    // Create channel that is blocked until ICE Gathering is complete
    let mut gather_complete = peer_connection.gathering_complete_promise().await;

    // Sets the LocalDescription, and starts our UDP listeners
    peer_connection.set_local_description(answer).await?;

    // Block until ICE Gathering is complete, disabling trickle ICE
    // we do this because we only can exchange one signaling message
    // in a production application you should exchange ICE Candidates via OnICECandidate
    let _ = gather_complete.recv().await;

    let mut b64output = String::new();
    b64output = String::from("byebye");

    // Output the answer in base64 so we can paste it in browser
    if let Some(local_desc) = peer_connection.local_description().await {
        let json_str = serde_json::to_string(&local_desc)?;
        let b64 = encode(&json_str);
        b64output = encode(&json_str);
        println!("{}", b64);
    } else {
        println!("generate local_description failed!");
    }

    println!("Press ctrl-c to stop");
    //let timeout = tokio::time::sleep(Duration::from_secs(20));
    //tokio::pin!(timeout);

    tokio::select! {
        //_ = timeout.as_mut() => {
        //    println!("received timeout signal!");
        //}
        _ = done_rx.recv() => {
            println!("received done signal!");
        }
        _ = tokio::signal::ctrl_c() => {
            println!("");
        }
    };

    peer_connection.close().await?;


    // Ok((format!("{}", output_tracks.len())))
    Ok((format!("{}", b64output)))
}
pub fn hello() -> Result<String> {
    foo()
    // Ok(format!("Hello"))
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

// following are used only for memory tests. Readers of this example do not need to consider it.

pub fn off_topic_memory_test_input_array(input: Vec<u8>) -> Result<i32> {
    Ok(input.len() as i32)
}

pub fn off_topic_memory_test_output_zero_copy_buffer(len: i32) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    Ok(ZeroCopyBuffer(vec![0u8; len as usize]))
}

pub fn off_topic_memory_test_output_vec_u8(len: i32) -> Result<Vec<u8>> {
    Ok(vec![0u8; len as usize])
}

pub fn off_topic_memory_test_input_vec_of_object(input: Vec<Size>) -> Result<i32> {
    Ok(input.len() as i32)
}

pub fn off_topic_memory_test_output_vec_of_object(len: i32) -> Result<Vec<Size>> {
    let item = Size {
        width: 42,
        height: 42,
    };
    Ok(vec![item; len as usize])
}

pub fn off_topic_memory_test_input_complex_struct(input: TreeNode) -> Result<i32> {
    Ok(input.children.len() as i32)
}

pub fn off_topic_memory_test_output_complex_struct(len: i32) -> Result<TreeNode> {
    let child = TreeNode {
        name: "child".to_string(),
        children: Vec::new(),
    };
    Ok(TreeNode {
        name: "root".to_string(),
        children: vec![child; len as usize],
    })
}

pub fn off_topic_deliberately_return_error() -> Result<i32> {
    std::env::set_var("RUST_BACKTRACE", "1"); // optional, just to see more info...
    Err(anyhow!("deliberately return Error!"))
}

pub fn off_topic_deliberately_panic() -> Result<i32> {
    std::env::set_var("RUST_BACKTRACE", "1"); // optional, just to see more info...
    panic!("deliberately panic!")
}
