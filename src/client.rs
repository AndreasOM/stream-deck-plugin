use futures_util::SinkExt;
use crate::ClientError;
use crate::ClientRequest;
use crate::client_request::SetImagePayload;
use crate::client_request::SetStatePayload;
use crate::client_request::SetTitlePayload;
use crate::ClientResponse;
use futures_util::StreamExt;

use color_eyre::Result;
use std::env;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;
use url::Url;


#[derive(Debug)]
enum Command {
    Request(ClientRequest),
}

#[derive(Debug)]
pub struct Client {
    rx: mpsc::Receiver<ClientResponse>,
    tx: mpsc::Sender<Command>,
}

impl Client {
    pub async fn new_from_args() -> Result<Self> {
        let mut args = env::args();

        args.next().expect("Binary should have name"); // skip binary name

        let mut port = None;
        let mut plugin_uuid = None;
        let mut register_event = None;
        let mut info = None;

        while let Some(a) = args.next() {
            match a.as_str() {
                "-port" => {
                    let p = args.next().ok_or_else(|| ClientError::MissingValue {
                        param: String::from("port"),
                    })?;
                    let p = p.parse::<u16>().map_err(|_e| ClientError::InvalidValue {
                        param: String::from("port"),
                        value: String::from(p),
                    })?;
                    port = Some(p);

                    tracing::info!("Port {port:?}\n");
                }
                "-pluginUUID" => {
                    let v = args.next().ok_or_else(|| ClientError::MissingValue {
                        param: String::from("pluginUUID"),
                    })?;
                    plugin_uuid = Some(v);
                    tracing::info!("plugin_uuid {plugin_uuid:?}");
                }
                "-registerEvent" => {
                    let v = args.next().ok_or_else(|| ClientError::MissingValue {
                        param: String::from("registerEvent"),
                    })?;
                    register_event = Some(v);

                    tracing::info!("register_event {register_event:?}");
                }
                "-info" => {
                    let v = args.next().ok_or_else(|| ClientError::MissingValue {
                        param: String::from("info"),
                    })?;
                    info = Some(v);
                    tracing::info!("info {info:?}");
                }
                o => {
                    tracing::info!("Unhandled argument {o}");
                }
            }
        }
        if port.is_none() {
            return Err(ClientError::MissingParameter {
                param: String::from("port"),
            }
            .into());
        }
        let port = port.unwrap();
        if plugin_uuid.is_none() {
            return Err(ClientError::MissingParameter {
                param: String::from("pluginUUID"),
            }
            .into());
        }
        let plugin_uuid = plugin_uuid.unwrap();
        if register_event.is_none() {
            return Err(ClientError::MissingParameter {
                param: String::from("registerEvent"),
            }
            .into());
        }
        let register_event = register_event.unwrap();
        if register_event != "registerPlugin" {
            return Err(ClientError::Severe {
                msg: format!("Unexpected registerEvent '{register_event}'"),
            }
            .into());            
        }
        if info.is_none() {
            return Err(ClientError::MissingParameter {
                param: String::from("info"),
            }
            .into());
        }
        let _info = info.unwrap();

        let url = format!("ws://localhost:{port}/");
        let url = Url::parse(&url)?;

        /*
        let (mut socket, response) =
            connect(url).expect("Can't connect");
        */
        let (socket, response) = connect_async(url).await.expect("Can connect");

        tracing::info!("Connected to the server");

        tracing::info!("Response HTTP code: {}", response.status());

        tracing::info!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            tracing::info!("* {}", header);
        }

        let (tx, rx) = mpsc::channel::<ClientResponse>(16);

        let (mut write, read) = socket.split();
        let register = ClientRequest::RegisterPlugin {
            uuid: plugin_uuid.clone(),
        };

        let j = serde_json::to_string(&register)?;

        write.send(j.into()).await?;


        tokio::spawn(async move {
            read.for_each(|m| async {
                match m {
                    Ok(msg) => {
                        match msg {
                            Message::Text(ref txt) => {
                                match serde_json::from_str::<ClientResponse>(&txt) {
                                    Ok(r) => {
                                        // Note: we could do custom handling here ...
                                        /*
                                        match r {
                                            PluginResponse::WillAppear { .. } => {
                                                tracing::info!("WillAppear {r:?}");
                                                let _ = tx.send( r ).await;
                                            }
                                            o => {
                                                tracing::warn!("Unhandled {o:?}");
                                            }
                                        }
                                        */
                                        // ... or just blindly route
                                        let _ = tx.send(r).await;
                                    }
                                    Err(e) => {
                                        tracing::info!("ERROR! Failed parsing: {msg} -> {e:?}");
                                    }
                                }
                            }
                            m => {
                                tracing::info!("WARN! Unhandled: {m}");
                            }
                        }
                    }
                    Err(e) => {
                        tracing::info!("ERROR! Error reading message: {e:?}");
                    }
                }
            })
            .await;
        });

        let (tx, mut cmd_rx) = mpsc::channel::<Command>(16);
        tokio::spawn(async move {
            loop {
                let timeout = tokio::time::sleep(tokio::time::Duration::from_millis(2000));
                tokio::select! {
                    _ = timeout => {
                        // tracing::info!("Heartbeat");

                        // for future extensions
                    }
                    Some( cmd ) = cmd_rx.recv() => {
                        match cmd {
                            Command::Request( request ) => {
                                    match serde_json::to_string(&request) {
                                        Ok( j ) => {
                                            //tracing::info!("Sending {j:?}");
                                            let _ = write.send(j.into()).await;
                                        },
                                        Err(_) => todo!()
                                    }
                            }
                        }
                    }
                }
            }
        });

        Ok(Self { rx, tx })
    }

    pub async fn recv(&mut self) -> Option<ClientResponse> {
        self.rx.recv().await
    }

    pub async fn send(&self, request: ClientRequest) -> Result<()> {
        let _ = self.tx.send(Command::Request(request)).await?;
        Ok(())
    }

    pub async fn send_set_title(
        &self,
        context: String,
        title: String,
        target: String,
        state: usize,
    ) -> Result<()> {
        self.send(ClientRequest::SetTitle {
            context,
            payload: SetTitlePayload {
                title,
                target,
                state,
            },
        })
        .await
    }
    pub async fn send_set_state(&self, context: String, state: usize) -> Result<()> {
        self.send(ClientRequest::SetState {
            context,
            payload: SetStatePayload { state },
        })
        .await
    }
    pub async fn send_set_image(&self, context: String, image: String, state: usize) -> Result<()> {
        self.send(ClientRequest::SetImage {
            context,
            payload: SetImagePayload { image, state },
        })
        .await
    }
}
