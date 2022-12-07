// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CHANGE_DATA_EVENT_FEED: ::grpcio::Method<super::cdcpb::ChangeDataRequest, super::cdcpb::ChangeDataEvent> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/cdcpb.ChangeData/EventFeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ChangeDataClient {
    client: ::grpcio::Client,
}

impl ChangeDataClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ChangeDataClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn event_feed_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::cdcpb::ChangeDataRequest>, ::grpcio::ClientDuplexReceiver<super::cdcpb::ChangeDataEvent>)> {
        self.client.duplex_streaming(&METHOD_CHANGE_DATA_EVENT_FEED, opt)
    }

    pub fn event_feed(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::cdcpb::ChangeDataRequest>, ::grpcio::ClientDuplexReceiver<super::cdcpb::ChangeDataEvent>)> {
        self.event_feed_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ChangeData {
    fn event_feed(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::cdcpb::ChangeDataRequest>, sink: ::grpcio::DuplexSink<super::cdcpb::ChangeDataEvent>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_change_data<S: ChangeData + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_CHANGE_DATA_EVENT_FEED, move |ctx, req, resp| {
        instance.event_feed(ctx, req, resp)
    });
    builder.build()
}
