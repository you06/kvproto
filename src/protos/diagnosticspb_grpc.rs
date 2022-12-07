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

const METHOD_DIAGNOSTICS_SEARCH_LOG: ::grpcio::Method<super::diagnosticspb::SearchLogRequest, super::diagnosticspb::SearchLogResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/diagnosticspb.Diagnostics/search_log",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DIAGNOSTICS_SERVER_INFO: ::grpcio::Method<super::diagnosticspb::ServerInfoRequest, super::diagnosticspb::ServerInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/diagnosticspb.Diagnostics/server_info",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DiagnosticsClient {
    client: ::grpcio::Client,
}

impl DiagnosticsClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DiagnosticsClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn search_log_opt(&self, req: &super::diagnosticspb::SearchLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::diagnosticspb::SearchLogResponse>> {
        self.client.server_streaming(&METHOD_DIAGNOSTICS_SEARCH_LOG, req, opt)
    }

    pub fn search_log(&self, req: &super::diagnosticspb::SearchLogRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::diagnosticspb::SearchLogResponse>> {
        self.search_log_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_info_opt(&self, req: &super::diagnosticspb::ServerInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::diagnosticspb::ServerInfoResponse> {
        self.client.unary_call(&METHOD_DIAGNOSTICS_SERVER_INFO, req, opt)
    }

    pub fn server_info(&self, req: &super::diagnosticspb::ServerInfoRequest) -> ::grpcio::Result<super::diagnosticspb::ServerInfoResponse> {
        self.server_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_info_async_opt(&self, req: &super::diagnosticspb::ServerInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::diagnosticspb::ServerInfoResponse>> {
        self.client.unary_call_async(&METHOD_DIAGNOSTICS_SERVER_INFO, req, opt)
    }

    pub fn server_info_async(&self, req: &super::diagnosticspb::ServerInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::diagnosticspb::ServerInfoResponse>> {
        self.server_info_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Diagnostics {
    fn search_log(&mut self, ctx: ::grpcio::RpcContext, _req: super::diagnosticspb::SearchLogRequest, sink: ::grpcio::ServerStreamingSink<super::diagnosticspb::SearchLogResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn server_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::diagnosticspb::ServerInfoRequest, sink: ::grpcio::UnarySink<super::diagnosticspb::ServerInfoResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_diagnostics<S: Diagnostics + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_DIAGNOSTICS_SEARCH_LOG, move |ctx, req, resp| {
        instance.search_log(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DIAGNOSTICS_SERVER_INFO, move |ctx, req, resp| {
        instance.server_info(ctx, req, resp)
    });
    builder.build()
}
