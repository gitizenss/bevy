//! A simple client that allows issuing queries to a remote Bevy server via the BRP.

use anyhow::Result as AnyhowResult;
use argh::FromArgs;
use ureq;
use serde_json;

/// Command-line arguments struct
#[derive(FromArgs)]
struct Args {
    /// the host IP address to connect to
    #[argh(option, default = "\"127.0.0.1\".to_string()")]
    host: String,
    /// the port to connect to
    #[argh(option, default = "15702")]
    port: u16,
    /// full type names of components to query or spawn
    #[argh(positional, greedy)]
    components: Vec<String>,
    /// flag to spawn a new cube
    #[argh(switch)]
    spawn: bool,
}

fn main() -> AnyhowResult<()> {
    // Parse command-line arguments
    let args: Args = argh::from_env();
    let url = format!("http://{}:{}/", args.host, args.port);

    if args.spawn {
        // Create a JSON request to spawn a new cube
        let spawn_params = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "bevy/spawn",
            "params": {
                "components": {
                    "server::Cube": { "0": 1.0 },
                    "bevy_transform::components::transform::Transform": {
                        "translation": { "x": 0.0, "y": 1.5, "z": 0.0 },
                        "rotation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 },
                        "scale": { "x": 1.0, "y": 1.0, "z": 1.0 }
                    }
                }
            }
        });

        let res = ureq::post(&url).send_json(spawn_params)?.into_json::<serde_json::Value>()?;
        println!("{:#}", res);
    } else {
        // Query the components on the server
        let res = ureq::post(&url)
            .send_json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "bevy/query",
                "params": {
                    "data": {
                        "components": args.components
                    }
                }
            }))?
            .into_json::<serde_json::Value>()?;

        println!("{:#}", res);
    }

    Ok(())
}
