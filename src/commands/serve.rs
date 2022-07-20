//More or less copied from mdbook cmd/serve.rs

use std::path::PathBuf;
use std::net::{SocketAddr, ToSocketAddrs};


use clap::{Command, ArgMatches};

use tokio::sync::broadcast;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use warp::ws::Message;
use warp::Filter;

use log;

use super::build::build_bookshelf_cmd;


// Create clap subcommand arguments
pub fn make_subcommand_serve<'help>() -> Command<'help> {
    Command::new("serve")
        .about("Serves the bookshelf at http://localhost:3000")
}

// Serve command implementation
pub fn execute_serve(_args: &ArgMatches) -> Result<(), anyhow::Error> {

    build_bookshelf_cmd();

    spawn_server("./build".to_string(), "127.0.0.1", "3000");

    
    //loop{} //loop until Ctrl+C is ran.

    Ok(())
}



/// The HTTP endpoint for the websocket used to trigger reloads when a file changes.
const LIVE_RELOAD_ENDPOINT: &str = "__livereload";

//Spawn a basic static server
pub fn spawn_server(build_dir: String, hostname: &str, port: &str) {

    let address = format!("{}:{}", hostname, port);

    let sockaddr: SocketAddr = address
        .to_socket_addrs()
        .expect(format!("{} is an invalid socket address.", address).as_str())
        .next()
        .ok_or_else(|| anyhow::anyhow!("no address found for {}", address))
        .expect("no address found");
        
    /*let input_404 = book
        .config
        .get("output.html.input-404")
        .map(toml::Value::as_str)
        .and_then(std::convert::identity) // flatten
        .map(ToString::to_string);
    let file_404 = get_404_output_file(&input_404);*/

    // A channel used to broadcast to any websockets to reload when a file changes.
    let (tx, _rx) = tokio::sync::broadcast::channel::<Message>(100);

    let reload_tx = tx.clone();
    let thread_handle = std::thread::spawn(move || {
        serve(PathBuf::from(build_dir), sockaddr, reload_tx, "error404.html");
    });

    let serving_url = format!("http://{}", address);
    log::info!("Serving on: {}", serving_url);
    
    //wait forever until program is closed with Ctrl+C
    if let Result::Err(err) = thread_handle.join() {
        log::error!("Error occured with server thread: {:?}", err);
    }
}


#[tokio::main]
async fn serve(
    build_dir: PathBuf,
    address: SocketAddr,
    reload_tx: broadcast::Sender<Message>,
    file_404: &str,
) {
    // A warp Filter which captures `reload_tx` and provides an `rx` copy to
    // receive reload messages.
    let sender = warp::any().map(move || reload_tx.subscribe());

    // A warp Filter to handle the livereload endpoint. This upgrades to a
    // websocket, and then waits for any filesystem change notifications, and
    // relays them over the websocket.
    let livereload = warp::path(LIVE_RELOAD_ENDPOINT)
        .and(warp::ws())
        .and(sender)
        .map(|ws: warp::ws::Ws, mut rx: broadcast::Receiver<Message>| {
            ws.on_upgrade(move |ws| async move {
                let (mut user_ws_tx, _user_ws_rx) = ws.split();
                log::trace!("websocket got connection");
                if let Ok(m) = rx.recv().await {
                    log::trace!("notify of reload");
                    let _ = user_ws_tx.send(m).await;
                }
            })
        });
    // A warp Filter that serves from the filesystem.
    let book_route = warp::fs::dir(build_dir.clone());
    // The fallback route for 404 errors
    let fallback_route = warp::fs::file(build_dir.join(file_404))
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::NOT_FOUND));
    let routes = livereload.or(book_route).or(fallback_route);

    std::panic::set_hook(Box::new(move |panic_info| {
        // exit if serve panics
        log::error!("Unable to serve: {}", panic_info);
        std::process::exit(1);
    }));

    warp::serve(routes).run(address).await;
}
