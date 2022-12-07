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

const METHOD_TRACE_RECORD_PUB_SUB_SUBSCRIBE: ::grpcio::Method<super::tracepb::TraceRecordRequest, super::tracepb::TraceRecord> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/tracepb.TraceRecordPubSub/Subscribe",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TraceRecordPubSubClient {
    client: ::grpcio::Client,
}

impl TraceRecordPubSubClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TraceRecordPubSubClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_opt(&self, req: &super::tracepb::TraceRecordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::tracepb::TraceRecord>> {
        self.client.server_streaming(&METHOD_TRACE_RECORD_PUB_SUB_SUBSCRIBE, req, opt)
    }

    pub fn subscribe(&self, req: &super::tracepb::TraceRecordRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::tracepb::TraceRecord>> {
        self.subscribe_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait TraceRecordPubSub {
    fn subscribe(&mut self, ctx: ::grpcio::RpcContext, _req: super::tracepb::TraceRecordRequest, sink: ::grpcio::ServerStreamingSink<super::tracepb::TraceRecord>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_trace_record_pub_sub<S: TraceRecordPubSub + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_TRACE_RECORD_PUB_SUB_SUBSCRIBE, move |ctx, req, resp| {
        instance.subscribe(ctx, req, resp)
    });
    builder.build()
}
