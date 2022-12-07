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

const METHOD_LOG_BACKUP_GET_LAST_FLUSH_TS_OF_REGION: ::grpcio::Method<super::logbackuppb::GetLastFlushTsOfRegionRequest, super::logbackuppb::GetLastFlushTsOfRegionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/logbackup.LogBackup/GetLastFlushTSOfRegion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LogBackupClient {
    client: ::grpcio::Client,
}

impl LogBackupClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LogBackupClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_last_flush_ts_of_region_opt(&self, req: &super::logbackuppb::GetLastFlushTsOfRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logbackuppb::GetLastFlushTsOfRegionResponse> {
        self.client.unary_call(&METHOD_LOG_BACKUP_GET_LAST_FLUSH_TS_OF_REGION, req, opt)
    }

    pub fn get_last_flush_ts_of_region(&self, req: &super::logbackuppb::GetLastFlushTsOfRegionRequest) -> ::grpcio::Result<super::logbackuppb::GetLastFlushTsOfRegionResponse> {
        self.get_last_flush_ts_of_region_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_last_flush_ts_of_region_async_opt(&self, req: &super::logbackuppb::GetLastFlushTsOfRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logbackuppb::GetLastFlushTsOfRegionResponse>> {
        self.client.unary_call_async(&METHOD_LOG_BACKUP_GET_LAST_FLUSH_TS_OF_REGION, req, opt)
    }

    pub fn get_last_flush_ts_of_region_async(&self, req: &super::logbackuppb::GetLastFlushTsOfRegionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logbackuppb::GetLastFlushTsOfRegionResponse>> {
        self.get_last_flush_ts_of_region_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait LogBackup {
    fn get_last_flush_ts_of_region(&mut self, ctx: ::grpcio::RpcContext, _req: super::logbackuppb::GetLastFlushTsOfRegionRequest, sink: ::grpcio::UnarySink<super::logbackuppb::GetLastFlushTsOfRegionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_log_backup<S: LogBackup + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_LOG_BACKUP_GET_LAST_FLUSH_TS_OF_REGION, move |ctx, req, resp| {
        instance.get_last_flush_ts_of_region(ctx, req, resp)
    });
    builder.build()
}
