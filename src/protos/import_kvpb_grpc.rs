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

const METHOD_IMPORT_KV_SWITCH_MODE: ::grpcio::Method<super::import_kvpb::SwitchModeRequest, super::import_kvpb::SwitchModeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/SwitchMode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_OPEN_ENGINE: ::grpcio::Method<super::import_kvpb::OpenEngineRequest, super::import_kvpb::OpenEngineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/OpenEngine",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_WRITE_ENGINE: ::grpcio::Method<super::import_kvpb::WriteEngineRequest, super::import_kvpb::WriteEngineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/import_kvpb.ImportKV/WriteEngine",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_WRITE_ENGINE_V3: ::grpcio::Method<super::import_kvpb::WriteEngineV3Request, super::import_kvpb::WriteEngineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/WriteEngineV3",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_CLOSE_ENGINE: ::grpcio::Method<super::import_kvpb::CloseEngineRequest, super::import_kvpb::CloseEngineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/CloseEngine",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_IMPORT_ENGINE: ::grpcio::Method<super::import_kvpb::ImportEngineRequest, super::import_kvpb::ImportEngineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/ImportEngine",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_CLEANUP_ENGINE: ::grpcio::Method<super::import_kvpb::CleanupEngineRequest, super::import_kvpb::CleanupEngineResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/CleanupEngine",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_COMPACT_CLUSTER: ::grpcio::Method<super::import_kvpb::CompactClusterRequest, super::import_kvpb::CompactClusterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/CompactCluster",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_GET_VERSION: ::grpcio::Method<super::import_kvpb::GetVersionRequest, super::import_kvpb::GetVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/GetVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_GET_METRICS: ::grpcio::Method<super::import_kvpb::GetMetricsRequest, super::import_kvpb::GetMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_kvpb.ImportKV/GetMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ImportKvClient {
    client: ::grpcio::Client,
}

impl ImportKvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportKvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn switch_mode_opt(&self, req: &super::import_kvpb::SwitchModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::SwitchModeResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_SWITCH_MODE, req, opt)
    }

    pub fn switch_mode(&self, req: &super::import_kvpb::SwitchModeRequest) -> ::grpcio::Result<super::import_kvpb::SwitchModeResponse> {
        self.switch_mode_opt(req, ::grpcio::CallOption::default())
    }

    pub fn switch_mode_async_opt(&self, req: &super::import_kvpb::SwitchModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::SwitchModeResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_SWITCH_MODE, req, opt)
    }

    pub fn switch_mode_async(&self, req: &super::import_kvpb::SwitchModeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::SwitchModeResponse>> {
        self.switch_mode_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_engine_opt(&self, req: &super::import_kvpb::OpenEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::OpenEngineResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_OPEN_ENGINE, req, opt)
    }

    pub fn open_engine(&self, req: &super::import_kvpb::OpenEngineRequest) -> ::grpcio::Result<super::import_kvpb::OpenEngineResponse> {
        self.open_engine_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_engine_async_opt(&self, req: &super::import_kvpb::OpenEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::OpenEngineResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_OPEN_ENGINE, req, opt)
    }

    pub fn open_engine_async(&self, req: &super::import_kvpb::OpenEngineRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::OpenEngineResponse>> {
        self.open_engine_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_engine_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_kvpb::WriteEngineRequest>, ::grpcio::ClientCStreamReceiver<super::import_kvpb::WriteEngineResponse>)> {
        self.client.client_streaming(&METHOD_IMPORT_KV_WRITE_ENGINE, opt)
    }

    pub fn write_engine(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_kvpb::WriteEngineRequest>, ::grpcio::ClientCStreamReceiver<super::import_kvpb::WriteEngineResponse>)> {
        self.write_engine_opt(::grpcio::CallOption::default())
    }

    pub fn write_engine_v3_opt(&self, req: &super::import_kvpb::WriteEngineV3Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::WriteEngineResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_WRITE_ENGINE_V3, req, opt)
    }

    pub fn write_engine_v3(&self, req: &super::import_kvpb::WriteEngineV3Request) -> ::grpcio::Result<super::import_kvpb::WriteEngineResponse> {
        self.write_engine_v3_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_engine_v3_async_opt(&self, req: &super::import_kvpb::WriteEngineV3Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::WriteEngineResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_WRITE_ENGINE_V3, req, opt)
    }

    pub fn write_engine_v3_async(&self, req: &super::import_kvpb::WriteEngineV3Request) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::WriteEngineResponse>> {
        self.write_engine_v3_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_engine_opt(&self, req: &super::import_kvpb::CloseEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::CloseEngineResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_CLOSE_ENGINE, req, opt)
    }

    pub fn close_engine(&self, req: &super::import_kvpb::CloseEngineRequest) -> ::grpcio::Result<super::import_kvpb::CloseEngineResponse> {
        self.close_engine_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_engine_async_opt(&self, req: &super::import_kvpb::CloseEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::CloseEngineResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_CLOSE_ENGINE, req, opt)
    }

    pub fn close_engine_async(&self, req: &super::import_kvpb::CloseEngineRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::CloseEngineResponse>> {
        self.close_engine_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_engine_opt(&self, req: &super::import_kvpb::ImportEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::ImportEngineResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_IMPORT_ENGINE, req, opt)
    }

    pub fn import_engine(&self, req: &super::import_kvpb::ImportEngineRequest) -> ::grpcio::Result<super::import_kvpb::ImportEngineResponse> {
        self.import_engine_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_engine_async_opt(&self, req: &super::import_kvpb::ImportEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::ImportEngineResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_IMPORT_ENGINE, req, opt)
    }

    pub fn import_engine_async(&self, req: &super::import_kvpb::ImportEngineRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::ImportEngineResponse>> {
        self.import_engine_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cleanup_engine_opt(&self, req: &super::import_kvpb::CleanupEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::CleanupEngineResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_CLEANUP_ENGINE, req, opt)
    }

    pub fn cleanup_engine(&self, req: &super::import_kvpb::CleanupEngineRequest) -> ::grpcio::Result<super::import_kvpb::CleanupEngineResponse> {
        self.cleanup_engine_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cleanup_engine_async_opt(&self, req: &super::import_kvpb::CleanupEngineRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::CleanupEngineResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_CLEANUP_ENGINE, req, opt)
    }

    pub fn cleanup_engine_async(&self, req: &super::import_kvpb::CleanupEngineRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::CleanupEngineResponse>> {
        self.cleanup_engine_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_cluster_opt(&self, req: &super::import_kvpb::CompactClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::CompactClusterResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_COMPACT_CLUSTER, req, opt)
    }

    pub fn compact_cluster(&self, req: &super::import_kvpb::CompactClusterRequest) -> ::grpcio::Result<super::import_kvpb::CompactClusterResponse> {
        self.compact_cluster_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_cluster_async_opt(&self, req: &super::import_kvpb::CompactClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::CompactClusterResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_COMPACT_CLUSTER, req, opt)
    }

    pub fn compact_cluster_async(&self, req: &super::import_kvpb::CompactClusterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::CompactClusterResponse>> {
        self.compact_cluster_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_version_opt(&self, req: &super::import_kvpb::GetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::GetVersionResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_GET_VERSION, req, opt)
    }

    pub fn get_version(&self, req: &super::import_kvpb::GetVersionRequest) -> ::grpcio::Result<super::import_kvpb::GetVersionResponse> {
        self.get_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_version_async_opt(&self, req: &super::import_kvpb::GetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::GetVersionResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_GET_VERSION, req, opt)
    }

    pub fn get_version_async(&self, req: &super::import_kvpb::GetVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::GetVersionResponse>> {
        self.get_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metrics_opt(&self, req: &super::import_kvpb::GetMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_kvpb::GetMetricsResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_GET_METRICS, req, opt)
    }

    pub fn get_metrics(&self, req: &super::import_kvpb::GetMetricsRequest) -> ::grpcio::Result<super::import_kvpb::GetMetricsResponse> {
        self.get_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metrics_async_opt(&self, req: &super::import_kvpb::GetMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::GetMetricsResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_GET_METRICS, req, opt)
    }

    pub fn get_metrics_async(&self, req: &super::import_kvpb::GetMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_kvpb::GetMetricsResponse>> {
        self.get_metrics_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ImportKv {
    fn switch_mode(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::SwitchModeRequest, sink: ::grpcio::UnarySink<super::import_kvpb::SwitchModeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn open_engine(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::OpenEngineRequest, sink: ::grpcio::UnarySink<super::import_kvpb::OpenEngineResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn write_engine(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::import_kvpb::WriteEngineRequest>, sink: ::grpcio::ClientStreamingSink<super::import_kvpb::WriteEngineResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn write_engine_v3(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::WriteEngineV3Request, sink: ::grpcio::UnarySink<super::import_kvpb::WriteEngineResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn close_engine(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::CloseEngineRequest, sink: ::grpcio::UnarySink<super::import_kvpb::CloseEngineResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn import_engine(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::ImportEngineRequest, sink: ::grpcio::UnarySink<super::import_kvpb::ImportEngineResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cleanup_engine(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::CleanupEngineRequest, sink: ::grpcio::UnarySink<super::import_kvpb::CleanupEngineResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn compact_cluster(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::CompactClusterRequest, sink: ::grpcio::UnarySink<super::import_kvpb::CompactClusterResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::GetVersionRequest, sink: ::grpcio::UnarySink<super::import_kvpb::GetVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_kvpb::GetMetricsRequest, sink: ::grpcio::UnarySink<super::import_kvpb::GetMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_import_kv<S: ImportKv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_SWITCH_MODE, move |ctx, req, resp| {
        instance.switch_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_OPEN_ENGINE, move |ctx, req, resp| {
        instance.open_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_KV_WRITE_ENGINE, move |ctx, req, resp| {
        instance.write_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_WRITE_ENGINE_V3, move |ctx, req, resp| {
        instance.write_engine_v3(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_CLOSE_ENGINE, move |ctx, req, resp| {
        instance.close_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_IMPORT_ENGINE, move |ctx, req, resp| {
        instance.import_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_CLEANUP_ENGINE, move |ctx, req, resp| {
        instance.cleanup_engine(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_COMPACT_CLUSTER, move |ctx, req, resp| {
        instance.compact_cluster(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_GET_VERSION, move |ctx, req, resp| {
        instance.get_version(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_GET_METRICS, move |ctx, req, resp| {
        instance.get_metrics(ctx, req, resp)
    });
    builder.build()
}
