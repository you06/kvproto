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

const METHOD_BACKUP_BACKUP: ::grpcio::Method<super::brpb::BackupRequest, super::brpb::BackupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/backup.Backup/backup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BACKUP_CHECK_PENDING_ADMIN_OP: ::grpcio::Method<super::brpb::CheckAdminRequest, super::brpb::CheckAdminResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/backup.Backup/CheckPendingAdminOp",
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

    pub fn backup_opt(&self, req: &super::brpb::BackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::brpb::BackupResponse>> {
        self.client.server_streaming(&METHOD_BACKUP_BACKUP, req, opt)
    }

    pub fn backup(&self, req: &super::brpb::BackupRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::brpb::BackupResponse>> {
        self.backup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_pending_admin_op_opt(&self, req: &super::brpb::CheckAdminRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::brpb::CheckAdminResponse>> {
        self.client.server_streaming(&METHOD_BACKUP_CHECK_PENDING_ADMIN_OP, req, opt)
    }

    pub fn check_pending_admin_op(&self, req: &super::brpb::CheckAdminRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::brpb::CheckAdminResponse>> {
        self.check_pending_admin_op_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Backup {
    fn backup(&mut self, ctx: ::grpcio::RpcContext, _req: super::brpb::BackupRequest, sink: ::grpcio::ServerStreamingSink<super::brpb::BackupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn check_pending_admin_op(&mut self, ctx: ::grpcio::RpcContext, _req: super::brpb::CheckAdminRequest, sink: ::grpcio::ServerStreamingSink<super::brpb::CheckAdminResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_backup<S: Backup + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_BACKUP_BACKUP, move |ctx, req, resp| {
        instance.backup(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_BACKUP_CHECK_PENDING_ADMIN_OP, move |ctx, req, resp| {
        instance.check_pending_admin_op(ctx, req, resp)
    });
    builder.build()
}

const METHOD_EXTERNAL_STORAGE_RESTORE: ::grpcio::Method<super::brpb::ExternalStorageRestoreRequest, super::brpb::ExternalStorageRestoreResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.ExternalStorage/restore",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_EXTERNAL_STORAGE_SAVE: ::grpcio::Method<super::brpb::ExternalStorageSaveRequest, super::brpb::ExternalStorageSaveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/backup.ExternalStorage/save",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ExternalStorageClient {
    client: ::grpcio::Client,
}

impl ExternalStorageClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ExternalStorageClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn restore_opt(&self, req: &super::brpb::ExternalStorageRestoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::brpb::ExternalStorageRestoreResponse> {
        self.client.unary_call(&METHOD_EXTERNAL_STORAGE_RESTORE, req, opt)
    }

    pub fn restore(&self, req: &super::brpb::ExternalStorageRestoreRequest) -> ::grpcio::Result<super::brpb::ExternalStorageRestoreResponse> {
        self.restore_opt(req, ::grpcio::CallOption::default())
    }

    pub fn restore_async_opt(&self, req: &super::brpb::ExternalStorageRestoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::brpb::ExternalStorageRestoreResponse>> {
        self.client.unary_call_async(&METHOD_EXTERNAL_STORAGE_RESTORE, req, opt)
    }

    pub fn restore_async(&self, req: &super::brpb::ExternalStorageRestoreRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::brpb::ExternalStorageRestoreResponse>> {
        self.restore_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn save_opt(&self, req: &super::brpb::ExternalStorageSaveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::brpb::ExternalStorageSaveResponse> {
        self.client.unary_call(&METHOD_EXTERNAL_STORAGE_SAVE, req, opt)
    }

    pub fn save(&self, req: &super::brpb::ExternalStorageSaveRequest) -> ::grpcio::Result<super::brpb::ExternalStorageSaveResponse> {
        self.save_opt(req, ::grpcio::CallOption::default())
    }

    pub fn save_async_opt(&self, req: &super::brpb::ExternalStorageSaveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::brpb::ExternalStorageSaveResponse>> {
        self.client.unary_call_async(&METHOD_EXTERNAL_STORAGE_SAVE, req, opt)
    }

    pub fn save_async(&self, req: &super::brpb::ExternalStorageSaveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::brpb::ExternalStorageSaveResponse>> {
        self.save_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ExternalStorage {
    fn restore(&mut self, ctx: ::grpcio::RpcContext, _req: super::brpb::ExternalStorageRestoreRequest, sink: ::grpcio::UnarySink<super::brpb::ExternalStorageRestoreResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn save(&mut self, ctx: ::grpcio::RpcContext, _req: super::brpb::ExternalStorageSaveRequest, sink: ::grpcio::UnarySink<super::brpb::ExternalStorageSaveResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_external_storage<S: ExternalStorage + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_EXTERNAL_STORAGE_RESTORE, move |ctx, req, resp| {
        instance.restore(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_EXTERNAL_STORAGE_SAVE, move |ctx, req, resp| {
        instance.save(ctx, req, resp)
    });
    builder.build()
}
