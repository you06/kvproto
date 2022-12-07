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

const METHOD_RECOVER_DATA_READ_REGION_META: ::grpcio::Method<super::recoverdatapb::ReadRegionMetaRequest, super::recoverdatapb::RegionMeta> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/recover_data.RecoverData/ReadRegionMeta",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RECOVER_DATA_RECOVER_REGION: ::grpcio::Method<super::recoverdatapb::RecoverRegionRequest, super::recoverdatapb::RecoverRegionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/recover_data.RecoverData/RecoverRegion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RECOVER_DATA_WAIT_APPLY: ::grpcio::Method<super::recoverdatapb::WaitApplyRequest, super::recoverdatapb::WaitApplyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/recover_data.RecoverData/WaitApply",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RECOVER_DATA_RESOLVE_KV_DATA: ::grpcio::Method<super::recoverdatapb::ResolveKvDataRequest, super::recoverdatapb::ResolveKvDataResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/recover_data.RecoverData/ResolveKvData",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RecoverDataClient {
    client: ::grpcio::Client,
}

impl RecoverDataClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RecoverDataClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn read_region_meta_opt(&self, req: &super::recoverdatapb::ReadRegionMetaRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::recoverdatapb::RegionMeta>> {
        self.client.server_streaming(&METHOD_RECOVER_DATA_READ_REGION_META, req, opt)
    }

    pub fn read_region_meta(&self, req: &super::recoverdatapb::ReadRegionMetaRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::recoverdatapb::RegionMeta>> {
        self.read_region_meta_opt(req, ::grpcio::CallOption::default())
    }

    pub fn recover_region_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::recoverdatapb::RecoverRegionRequest>, ::grpcio::ClientCStreamReceiver<super::recoverdatapb::RecoverRegionResponse>)> {
        self.client.client_streaming(&METHOD_RECOVER_DATA_RECOVER_REGION, opt)
    }

    pub fn recover_region(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::recoverdatapb::RecoverRegionRequest>, ::grpcio::ClientCStreamReceiver<super::recoverdatapb::RecoverRegionResponse>)> {
        self.recover_region_opt(::grpcio::CallOption::default())
    }

    pub fn wait_apply_opt(&self, req: &super::recoverdatapb::WaitApplyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::recoverdatapb::WaitApplyResponse> {
        self.client.unary_call(&METHOD_RECOVER_DATA_WAIT_APPLY, req, opt)
    }

    pub fn wait_apply(&self, req: &super::recoverdatapb::WaitApplyRequest) -> ::grpcio::Result<super::recoverdatapb::WaitApplyResponse> {
        self.wait_apply_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_apply_async_opt(&self, req: &super::recoverdatapb::WaitApplyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::recoverdatapb::WaitApplyResponse>> {
        self.client.unary_call_async(&METHOD_RECOVER_DATA_WAIT_APPLY, req, opt)
    }

    pub fn wait_apply_async(&self, req: &super::recoverdatapb::WaitApplyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::recoverdatapb::WaitApplyResponse>> {
        self.wait_apply_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn resolve_kv_data_opt(&self, req: &super::recoverdatapb::ResolveKvDataRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::recoverdatapb::ResolveKvDataResponse>> {
        self.client.server_streaming(&METHOD_RECOVER_DATA_RESOLVE_KV_DATA, req, opt)
    }

    pub fn resolve_kv_data(&self, req: &super::recoverdatapb::ResolveKvDataRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::recoverdatapb::ResolveKvDataResponse>> {
        self.resolve_kv_data_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RecoverData {
    fn read_region_meta(&mut self, ctx: ::grpcio::RpcContext, _req: super::recoverdatapb::ReadRegionMetaRequest, sink: ::grpcio::ServerStreamingSink<super::recoverdatapb::RegionMeta>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn recover_region(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::recoverdatapb::RecoverRegionRequest>, sink: ::grpcio::ClientStreamingSink<super::recoverdatapb::RecoverRegionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn wait_apply(&mut self, ctx: ::grpcio::RpcContext, _req: super::recoverdatapb::WaitApplyRequest, sink: ::grpcio::UnarySink<super::recoverdatapb::WaitApplyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn resolve_kv_data(&mut self, ctx: ::grpcio::RpcContext, _req: super::recoverdatapb::ResolveKvDataRequest, sink: ::grpcio::ServerStreamingSink<super::recoverdatapb::ResolveKvDataResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_recover_data<S: RecoverData + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_RECOVER_DATA_READ_REGION_META, move |ctx, req, resp| {
        instance.read_region_meta(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_RECOVER_DATA_RECOVER_REGION, move |ctx, req, resp| {
        instance.recover_region(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RECOVER_DATA_WAIT_APPLY, move |ctx, req, resp| {
        instance.wait_apply(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_RECOVER_DATA_RESOLVE_KV_DATA, move |ctx, req, resp| {
        instance.resolve_kv_data(ctx, req, resp)
    });
    builder.build()
}
