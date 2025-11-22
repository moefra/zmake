use crate::cas::Cas;
use crate::proto::net::SocketAddress;
use crate::socket_address;
use dashmap::DashMap;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Code, Status};

#[derive(Debug)]
pub struct DefaultCasServer {
    cas: Arc<dyn Cas + 'static>,
    map: DashMap<String, TransportDetails>,
    addr: crate::proto::net::SocketAddress,
}

impl DefaultCasServer {
    pub fn new(cas: Arc<dyn Cas + 'static>, server_addr: std::net::SocketAddr) -> Self {
        Self {
            cas,
            map: DashMap::new(),
            addr: server_addr.into(),
        }
    }
}

use crate::proto::cas::{
    GetTransportDetailsRequest, NegotiateBlobsRequest, NegotiateBlobsResponse, TransportDetails,
    content_addressable_storage_server::ContentAddressableStorage,
};
use tonic::codegen::tokio_stream::Stream;
use tonic::{Request, Response, Streaming};

#[tonic::async_trait]
impl ContentAddressableStorage for DefaultCasServer {
    type NegotiateBlobsStream =
        Pin<Box<dyn Stream<Item = Result<NegotiateBlobsResponse, Status>> + Send>>;

    async fn negotiate_blobs(
        &self,
        request: Request<Streaming<NegotiateBlobsRequest>>,
    ) -> Result<Response<Self::NegotiateBlobsStream>, Status> {
        let mut missing_blob_digests: Vec<crate::proto::digest::Digest> = Vec::new();

        let next_request = request.into_inner().message().await?;

        if let Some(request) = next_request {
            for blob_digest in request.blob_digests {
                let cloned = blob_digest.clone();
                if !self
                    .cas
                    .check(
                        &cloned
                            .try_into()
                            .map_err(|err: crate::digest::DigestError| {
                                Status::invalid_argument(err.to_string())
                            })?,
                    )
                    .await
                {
                    missing_blob_digests.push(blob_digest);
                }
            }
        }

        return Ok(Response::new(Box::pin(tokio_stream::iter(
            missing_blob_digests.into_iter().map(|digest| {
                Ok(NegotiateBlobsResponse {
                    missing_blob_digests: vec![digest.into()],
                })
            }),
        ))));
    }

    async fn get_transport_details(
        &self,
        request: Request<GetTransportDetailsRequest>,
    ) -> Result<Response<TransportDetails>, Status> {
        let server_addr = Some(self.addr.clone());

        let inner = request.into_inner();

        let mut accept_protocol: Option<crate::proto::net::Protocol> = None;

        for supported_protocol in inner.supported_protocols {
            if supported_protocol
                == <crate::proto::net::Protocol as Into<i32>>::into(
                    crate::proto::net::Protocol::Quic,
                )
            {
                accept_protocol = Some(crate::proto::net::Protocol::Quic);
                break;
            }
        }

        return Ok(Response::new(TransportDetails {
            server_addr,
            auth_token: "todo!()".to_string(),
            recommended_concurrency: 1,
        }));
    }
}
