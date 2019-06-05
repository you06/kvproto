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

const METHOD_BACKUP_START_FULL_BACKUP: ::grpcio::Method<super::backup::StartFullBackupRequest, super::backup::StartFullBackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.Backup/start_full_backup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BACKUP_BACKUP_REGION: ::grpcio::Method<super::backup::BackupRegionRequest, super::backup::BackupRegionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.Backup/backup_region",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BACKUP_FINISH_FULL_BACKUP: ::grpcio::Method<super::backup::FinishFullBackupRequest, super::backup::FinishFullBackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.Backup/finish_full_backup",
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

    pub fn start_full_backup_opt(&self, req: &super::backup::StartFullBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::backup::StartFullBackupResponse> {
        self.client.unary_call(&METHOD_BACKUP_START_FULL_BACKUP, req, opt)
    }

    pub fn start_full_backup(&self, req: &super::backup::StartFullBackupRequest) -> ::grpcio::Result<super::backup::StartFullBackupResponse> {
        self.start_full_backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_full_backup_async_opt(&self, req: &super::backup::StartFullBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::StartFullBackupResponse>> {
        self.client.unary_call_async(&METHOD_BACKUP_START_FULL_BACKUP, req, opt)
    }

    pub fn start_full_backup_async(&self, req: &super::backup::StartFullBackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::StartFullBackupResponse>> {
        self.start_full_backup_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn finish_full_backup_opt(&self, req: &super::backup::FinishFullBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::backup::FinishFullBackupResponse> {
        self.client.unary_call(&METHOD_BACKUP_FINISH_FULL_BACKUP, req, opt)
    }

    pub fn finish_full_backup(&self, req: &super::backup::FinishFullBackupRequest) -> ::grpcio::Result<super::backup::FinishFullBackupResponse> {
        self.finish_full_backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn finish_full_backup_async_opt(&self, req: &super::backup::FinishFullBackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::FinishFullBackupResponse>> {
        self.client.unary_call_async(&METHOD_BACKUP_FINISH_FULL_BACKUP, req, opt)
    }

    pub fn finish_full_backup_async(&self, req: &super::backup::FinishFullBackupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::backup::FinishFullBackupResponse>> {
        self.finish_full_backup_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Backup {
    fn start_full_backup(&mut self, ctx: ::grpcio::RpcContext, req: super::backup::StartFullBackupRequest, sink: ::grpcio::UnarySink<super::backup::StartFullBackupResponse>);
    fn backup_region(&mut self, ctx: ::grpcio::RpcContext, req: super::backup::BackupRegionRequest, sink: ::grpcio::UnarySink<super::backup::BackupRegionResponse>);
    fn finish_full_backup(&mut self, ctx: ::grpcio::RpcContext, req: super::backup::FinishFullBackupRequest, sink: ::grpcio::UnarySink<super::backup::FinishFullBackupResponse>);
}

pub fn create_backup<S: Backup + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BACKUP_START_FULL_BACKUP, move |ctx, req, resp| {
        instance.start_full_backup(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BACKUP_BACKUP_REGION, move |ctx, req, resp| {
        instance.backup_region(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BACKUP_FINISH_FULL_BACKUP, move |ctx, req, resp| {
        instance.finish_full_backup(ctx, req, resp)
    });
    builder.build()
}
