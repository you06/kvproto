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

const METHOD_AUTO_ID_ALLOC_ALLOC_AUTO_ID: ::grpcio::Method<super::autoid::AutoIdRequest, super::autoid::AutoIdResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/autoid.AutoIDAlloc/AllocAutoID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AUTO_ID_ALLOC_REBASE: ::grpcio::Method<super::autoid::RebaseRequest, super::autoid::RebaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/autoid.AutoIDAlloc/Rebase",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AutoIdAllocClient {
    client: ::grpcio::Client,
}

impl AutoIdAllocClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AutoIdAllocClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn alloc_auto_id_opt(&self, req: &super::autoid::AutoIdRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::autoid::AutoIdResponse> {
        self.client.unary_call(&METHOD_AUTO_ID_ALLOC_ALLOC_AUTO_ID, req, opt)
    }

    pub fn alloc_auto_id(&self, req: &super::autoid::AutoIdRequest) -> ::grpcio::Result<super::autoid::AutoIdResponse> {
        self.alloc_auto_id_opt(req, ::grpcio::CallOption::default())
    }

    pub fn alloc_auto_id_async_opt(&self, req: &super::autoid::AutoIdRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::autoid::AutoIdResponse>> {
        self.client.unary_call_async(&METHOD_AUTO_ID_ALLOC_ALLOC_AUTO_ID, req, opt)
    }

    pub fn alloc_auto_id_async(&self, req: &super::autoid::AutoIdRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::autoid::AutoIdResponse>> {
        self.alloc_auto_id_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rebase_opt(&self, req: &super::autoid::RebaseRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::autoid::RebaseResponse> {
        self.client.unary_call(&METHOD_AUTO_ID_ALLOC_REBASE, req, opt)
    }

    pub fn rebase(&self, req: &super::autoid::RebaseRequest) -> ::grpcio::Result<super::autoid::RebaseResponse> {
        self.rebase_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rebase_async_opt(&self, req: &super::autoid::RebaseRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::autoid::RebaseResponse>> {
        self.client.unary_call_async(&METHOD_AUTO_ID_ALLOC_REBASE, req, opt)
    }

    pub fn rebase_async(&self, req: &super::autoid::RebaseRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::autoid::RebaseResponse>> {
        self.rebase_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AutoIdAlloc {
    fn alloc_auto_id(&mut self, ctx: ::grpcio::RpcContext, _req: super::autoid::AutoIdRequest, sink: ::grpcio::UnarySink<super::autoid::AutoIdResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn rebase(&mut self, ctx: ::grpcio::RpcContext, _req: super::autoid::RebaseRequest, sink: ::grpcio::UnarySink<super::autoid::RebaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_auto_id_alloc<S: AutoIdAlloc + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_AUTO_ID_ALLOC_ALLOC_AUTO_ID, move |ctx, req, resp| {
        instance.alloc_auto_id(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_AUTO_ID_ALLOC_REBASE, move |ctx, req, resp| {
        instance.rebase(ctx, req, resp)
    });
    builder.build()
}
