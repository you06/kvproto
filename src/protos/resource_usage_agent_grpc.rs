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

const METHOD_RESOURCE_USAGE_AGENT_REPORT: ::grpcio::Method<super::resource_usage_agent::ResourceUsageRecord, super::resource_usage_agent::EmptyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/resource_usage_agent.ResourceUsageAgent/Report",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ResourceUsageAgentClient {
    client: ::grpcio::Client,
}

impl ResourceUsageAgentClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ResourceUsageAgentClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn report_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::resource_usage_agent::ResourceUsageRecord>, ::grpcio::ClientCStreamReceiver<super::resource_usage_agent::EmptyResponse>)> {
        self.client.client_streaming(&METHOD_RESOURCE_USAGE_AGENT_REPORT, opt)
    }

    pub fn report(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::resource_usage_agent::ResourceUsageRecord>, ::grpcio::ClientCStreamReceiver<super::resource_usage_agent::EmptyResponse>)> {
        self.report_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ResourceUsageAgent {
    fn report(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::resource_usage_agent::ResourceUsageRecord>, sink: ::grpcio::ClientStreamingSink<super::resource_usage_agent::EmptyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_resource_usage_agent<S: ResourceUsageAgent + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_client_streaming_handler(&METHOD_RESOURCE_USAGE_AGENT_REPORT, move |ctx, req, resp| {
        instance.report(ctx, req, resp)
    });
    builder.build()
}

const METHOD_RESOURCE_METERING_PUB_SUB_SUBSCRIBE: ::grpcio::Method<super::resource_usage_agent::ResourceMeteringRequest, super::resource_usage_agent::ResourceUsageRecord> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/resource_usage_agent.ResourceMeteringPubSub/Subscribe",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ResourceMeteringPubSubClient {
    client: ::grpcio::Client,
}

impl ResourceMeteringPubSubClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ResourceMeteringPubSubClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_opt(&self, req: &super::resource_usage_agent::ResourceMeteringRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::resource_usage_agent::ResourceUsageRecord>> {
        self.client.server_streaming(&METHOD_RESOURCE_METERING_PUB_SUB_SUBSCRIBE, req, opt)
    }

    pub fn subscribe(&self, req: &super::resource_usage_agent::ResourceMeteringRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::resource_usage_agent::ResourceUsageRecord>> {
        self.subscribe_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ResourceMeteringPubSub {
    fn subscribe(&mut self, ctx: ::grpcio::RpcContext, _req: super::resource_usage_agent::ResourceMeteringRequest, sink: ::grpcio::ServerStreamingSink<super::resource_usage_agent::ResourceUsageRecord>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_resource_metering_pub_sub<S: ResourceMeteringPubSub + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_RESOURCE_METERING_PUB_SUB_SUBSCRIBE, move |ctx, req, resp| {
        instance.subscribe(ctx, req, resp)
    });
    builder.build()
}
