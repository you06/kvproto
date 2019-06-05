// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

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

const METHOD_BACKUP_BACKUP: ::grpcio::Method<super::backup::BackupRequest, super::backup::BackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.Backup/backup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BACKUP_BACKUP_REGION: ::grpcio::Method<super::backup::BackupRegionRequest, super::backup::BackupRegionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.Backup/backup_region",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct BackupClient {
    client: ::grpcio::Client,
}

impl BackupClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BackupClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn backup_opt(&self, req: &super::backup::BackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::backup::BackupResponse> {
        self.client.unary_call(&METHOD_BACKUP_BACKUP, req, opt)
    }

    pub fn backup(&self, req: &super::backup::BackupRequest) -> ::grpcio::Result<super::backup::BackupResponse> {
        self.backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn backup_async_opt(&self, req: &super::backup::BackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::BackupResponse>> {
        self.client.unary_call_async(&METHOD_BACKUP_BACKUP, req, opt)
    }

    pub fn backup_async(&self, req: &super::backup::BackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::BackupResponse>> {
        self.backup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn backup_region_opt(&self, req: &super::backup::BackupRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::backup::BackupRegionResponse> {
        self.client.unary_call(&METHOD_BACKUP_BACKUP_REGION, req, opt)
    }

    pub fn backup_region(&self, req: &super::backup::BackupRegionRequest) -> ::grpcio::Result<super::backup::BackupRegionResponse> {
        self.backup_region_opt(req, ::grpcio::CallOption::default())
    }

    pub fn backup_region_async_opt(&self, req: &super::backup::BackupRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::BackupRegionResponse>> {
        self.client.unary_call_async(&METHOD_BACKUP_BACKUP_REGION, req, opt)
    }

    pub fn backup_region_async(&self, req: &super::backup::BackupRegionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::BackupRegionResponse>> {
        self.backup_region_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Backup {
    fn backup(&mut self, ctx: ::grpcio::RpcContext, req: super::backup::BackupRequest, sink: ::grpcio::UnarySink<super::backup::BackupResponse>);
    fn backup_region(&mut self, ctx: ::grpcio::RpcContext, req: super::backup::BackupRegionRequest, sink: ::grpcio::UnarySink<super::backup::BackupRegionResponse>);
}

pub fn create_backup<S: Backup + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BACKUP_BACKUP, move |ctx, req, resp| {
        instance.backup(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BACKUP_BACKUP_REGION, move |ctx, req, resp| {
        instance.backup_region(ctx, req, resp)
    });
    builder.build()
}
