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

const METHOD_GC_LIST_KEY_SPACES: ::grpcio::Method<super::gcpb::ListKeySpacesRequest, super::gcpb::ListKeySpacesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gcpb.GC/ListKeySpaces",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GC_GET_MIN_SERVICE_SAFE_POINT: ::grpcio::Method<super::gcpb::GetMinServiceSafePointRequest, super::gcpb::GetMinServiceSafePointResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gcpb.GC/GetMinServiceSafePoint",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GC_UPDATE_GC_SAFE_POINT: ::grpcio::Method<super::gcpb::UpdateGcSafePointRequest, super::gcpb::UpdateGcSafePointResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gcpb.GC/UpdateGCSafePoint",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GC_UPDATE_SERVICE_SAFE_POINT: ::grpcio::Method<super::gcpb::UpdateServiceSafePointRequest, super::gcpb::UpdateServiceSafePointResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/gcpb.GC/UpdateServiceSafePoint",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GcClient {
    client: ::grpcio::Client,
}

impl GcClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GcClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_key_spaces_opt(&self, req: &super::gcpb::ListKeySpacesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gcpb::ListKeySpacesResponse> {
        self.client.unary_call(&METHOD_GC_LIST_KEY_SPACES, req, opt)
    }

    pub fn list_key_spaces(&self, req: &super::gcpb::ListKeySpacesRequest) -> ::grpcio::Result<super::gcpb::ListKeySpacesResponse> {
        self.list_key_spaces_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_key_spaces_async_opt(&self, req: &super::gcpb::ListKeySpacesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::ListKeySpacesResponse>> {
        self.client.unary_call_async(&METHOD_GC_LIST_KEY_SPACES, req, opt)
    }

    pub fn list_key_spaces_async(&self, req: &super::gcpb::ListKeySpacesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::ListKeySpacesResponse>> {
        self.list_key_spaces_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_min_service_safe_point_opt(&self, req: &super::gcpb::GetMinServiceSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gcpb::GetMinServiceSafePointResponse> {
        self.client.unary_call(&METHOD_GC_GET_MIN_SERVICE_SAFE_POINT, req, opt)
    }

    pub fn get_min_service_safe_point(&self, req: &super::gcpb::GetMinServiceSafePointRequest) -> ::grpcio::Result<super::gcpb::GetMinServiceSafePointResponse> {
        self.get_min_service_safe_point_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_min_service_safe_point_async_opt(&self, req: &super::gcpb::GetMinServiceSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::GetMinServiceSafePointResponse>> {
        self.client.unary_call_async(&METHOD_GC_GET_MIN_SERVICE_SAFE_POINT, req, opt)
    }

    pub fn get_min_service_safe_point_async(&self, req: &super::gcpb::GetMinServiceSafePointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::GetMinServiceSafePointResponse>> {
        self.get_min_service_safe_point_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_gc_safe_point_opt(&self, req: &super::gcpb::UpdateGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gcpb::UpdateGcSafePointResponse> {
        self.client.unary_call(&METHOD_GC_UPDATE_GC_SAFE_POINT, req, opt)
    }

    pub fn update_gc_safe_point(&self, req: &super::gcpb::UpdateGcSafePointRequest) -> ::grpcio::Result<super::gcpb::UpdateGcSafePointResponse> {
        self.update_gc_safe_point_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_gc_safe_point_async_opt(&self, req: &super::gcpb::UpdateGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::UpdateGcSafePointResponse>> {
        self.client.unary_call_async(&METHOD_GC_UPDATE_GC_SAFE_POINT, req, opt)
    }

    pub fn update_gc_safe_point_async(&self, req: &super::gcpb::UpdateGcSafePointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::UpdateGcSafePointResponse>> {
        self.update_gc_safe_point_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_safe_point_opt(&self, req: &super::gcpb::UpdateServiceSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::gcpb::UpdateServiceSafePointResponse> {
        self.client.unary_call(&METHOD_GC_UPDATE_SERVICE_SAFE_POINT, req, opt)
    }

    pub fn update_service_safe_point(&self, req: &super::gcpb::UpdateServiceSafePointRequest) -> ::grpcio::Result<super::gcpb::UpdateServiceSafePointResponse> {
        self.update_service_safe_point_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_safe_point_async_opt(&self, req: &super::gcpb::UpdateServiceSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::UpdateServiceSafePointResponse>> {
        self.client.unary_call_async(&METHOD_GC_UPDATE_SERVICE_SAFE_POINT, req, opt)
    }

    pub fn update_service_safe_point_async(&self, req: &super::gcpb::UpdateServiceSafePointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::gcpb::UpdateServiceSafePointResponse>> {
        self.update_service_safe_point_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Gc {
    fn list_key_spaces(&mut self, ctx: ::grpcio::RpcContext, _req: super::gcpb::ListKeySpacesRequest, sink: ::grpcio::UnarySink<super::gcpb::ListKeySpacesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_min_service_safe_point(&mut self, ctx: ::grpcio::RpcContext, _req: super::gcpb::GetMinServiceSafePointRequest, sink: ::grpcio::UnarySink<super::gcpb::GetMinServiceSafePointResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_gc_safe_point(&mut self, ctx: ::grpcio::RpcContext, _req: super::gcpb::UpdateGcSafePointRequest, sink: ::grpcio::UnarySink<super::gcpb::UpdateGcSafePointResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_service_safe_point(&mut self, ctx: ::grpcio::RpcContext, _req: super::gcpb::UpdateServiceSafePointRequest, sink: ::grpcio::UnarySink<super::gcpb::UpdateServiceSafePointResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_gc<S: Gc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GC_LIST_KEY_SPACES, move |ctx, req, resp| {
        instance.list_key_spaces(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GC_GET_MIN_SERVICE_SAFE_POINT, move |ctx, req, resp| {
        instance.get_min_service_safe_point(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GC_UPDATE_GC_SAFE_POINT, move |ctx, req, resp| {
        instance.update_gc_safe_point(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_GC_UPDATE_SERVICE_SAFE_POINT, move |ctx, req, resp| {
        instance.update_service_safe_point(ctx, req, resp)
    });
    builder.build()
}
