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

const METHOD_IMPORT_SST_SWITCH_MODE: ::grpcio::Method<super::import_sstpb::SwitchModeRequest, super::import_sstpb::SwitchModeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/SwitchMode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_UPLOAD: ::grpcio::Method<super::import_sstpb::UploadRequest, super::import_sstpb::UploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/import_sstpb.ImportSST/Upload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_INGEST: ::grpcio::Method<super::import_sstpb::IngestRequest, super::import_sstpb::IngestResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/Ingest",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_COMPACT: ::grpcio::Method<super::import_sstpb::CompactRequest, super::import_sstpb::CompactResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/Compact",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT: ::grpcio::Method<super::import_sstpb::SetDownloadSpeedLimitRequest, super::import_sstpb::SetDownloadSpeedLimitResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/SetDownloadSpeedLimit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_DOWNLOAD: ::grpcio::Method<super::import_sstpb::DownloadRequest, super::import_sstpb::DownloadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/Download",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_WRITE: ::grpcio::Method<super::import_sstpb::WriteRequest, super::import_sstpb::WriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/import_sstpb.ImportSST/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_RAW_WRITE: ::grpcio::Method<super::import_sstpb::RawWriteRequest, super::import_sstpb::RawWriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/import_sstpb.ImportSST/RawWrite",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_MULTI_INGEST: ::grpcio::Method<super::import_sstpb::MultiIngestRequest, super::import_sstpb::IngestResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/MultiIngest",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_DUPLICATE_DETECT: ::grpcio::Method<super::import_sstpb::DuplicateDetectRequest, super::import_sstpb::DuplicateDetectResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/import_sstpb.ImportSST/DuplicateDetect",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_APPLY: ::grpcio::Method<super::import_sstpb::ApplyRequest, super::import_sstpb::ApplyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/Apply",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_CLEAR_FILES: ::grpcio::Method<super::import_sstpb::ClearRequest, super::import_sstpb::ClearResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/import_sstpb.ImportSST/ClearFiles",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ImportSstClient {
    client: ::grpcio::Client,
}

impl ImportSstClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportSstClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn switch_mode_opt(&self, req: &super::import_sstpb::SwitchModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::SwitchModeResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_SWITCH_MODE, req, opt)
    }

    pub fn switch_mode(&self, req: &super::import_sstpb::SwitchModeRequest) -> ::grpcio::Result<super::import_sstpb::SwitchModeResponse> {
        self.switch_mode_opt(req, ::grpcio::CallOption::default())
    }

    pub fn switch_mode_async_opt(&self, req: &super::import_sstpb::SwitchModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::SwitchModeResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_SWITCH_MODE, req, opt)
    }

    pub fn switch_mode_async(&self, req: &super::import_sstpb::SwitchModeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::SwitchModeResponse>> {
        self.switch_mode_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upload_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_sstpb::UploadRequest>, ::grpcio::ClientCStreamReceiver<super::import_sstpb::UploadResponse>)> {
        self.client.client_streaming(&METHOD_IMPORT_SST_UPLOAD, opt)
    }

    pub fn upload(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_sstpb::UploadRequest>, ::grpcio::ClientCStreamReceiver<super::import_sstpb::UploadResponse>)> {
        self.upload_opt(::grpcio::CallOption::default())
    }

    pub fn ingest_opt(&self, req: &super::import_sstpb::IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::IngestResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_INGEST, req, opt)
    }

    pub fn ingest(&self, req: &super::import_sstpb::IngestRequest) -> ::grpcio::Result<super::import_sstpb::IngestResponse> {
        self.ingest_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ingest_async_opt(&self, req: &super::import_sstpb::IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::IngestResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_INGEST, req, opt)
    }

    pub fn ingest_async(&self, req: &super::import_sstpb::IngestRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::IngestResponse>> {
        self.ingest_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_opt(&self, req: &super::import_sstpb::CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::CompactResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_COMPACT, req, opt)
    }

    pub fn compact(&self, req: &super::import_sstpb::CompactRequest) -> ::grpcio::Result<super::import_sstpb::CompactResponse> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_async_opt(&self, req: &super::import_sstpb::CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::CompactResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_COMPACT, req, opt)
    }

    pub fn compact_async(&self, req: &super::import_sstpb::CompactRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::CompactResponse>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_download_speed_limit_opt(&self, req: &super::import_sstpb::SetDownloadSpeedLimitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::SetDownloadSpeedLimitResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT, req, opt)
    }

    pub fn set_download_speed_limit(&self, req: &super::import_sstpb::SetDownloadSpeedLimitRequest) -> ::grpcio::Result<super::import_sstpb::SetDownloadSpeedLimitResponse> {
        self.set_download_speed_limit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_download_speed_limit_async_opt(&self, req: &super::import_sstpb::SetDownloadSpeedLimitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::SetDownloadSpeedLimitResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT, req, opt)
    }

    pub fn set_download_speed_limit_async(&self, req: &super::import_sstpb::SetDownloadSpeedLimitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::SetDownloadSpeedLimitResponse>> {
        self.set_download_speed_limit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn download_opt(&self, req: &super::import_sstpb::DownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::DownloadResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_DOWNLOAD, req, opt)
    }

    pub fn download(&self, req: &super::import_sstpb::DownloadRequest) -> ::grpcio::Result<super::import_sstpb::DownloadResponse> {
        self.download_opt(req, ::grpcio::CallOption::default())
    }

    pub fn download_async_opt(&self, req: &super::import_sstpb::DownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::DownloadResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_DOWNLOAD, req, opt)
    }

    pub fn download_async(&self, req: &super::import_sstpb::DownloadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::DownloadResponse>> {
        self.download_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_sstpb::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::import_sstpb::WriteResponse>)> {
        self.client.client_streaming(&METHOD_IMPORT_SST_WRITE, opt)
    }

    pub fn write(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_sstpb::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::import_sstpb::WriteResponse>)> {
        self.write_opt(::grpcio::CallOption::default())
    }

    pub fn raw_write_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_sstpb::RawWriteRequest>, ::grpcio::ClientCStreamReceiver<super::import_sstpb::RawWriteResponse>)> {
        self.client.client_streaming(&METHOD_IMPORT_SST_RAW_WRITE, opt)
    }

    pub fn raw_write(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::import_sstpb::RawWriteRequest>, ::grpcio::ClientCStreamReceiver<super::import_sstpb::RawWriteResponse>)> {
        self.raw_write_opt(::grpcio::CallOption::default())
    }

    pub fn multi_ingest_opt(&self, req: &super::import_sstpb::MultiIngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::IngestResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_MULTI_INGEST, req, opt)
    }

    pub fn multi_ingest(&self, req: &super::import_sstpb::MultiIngestRequest) -> ::grpcio::Result<super::import_sstpb::IngestResponse> {
        self.multi_ingest_opt(req, ::grpcio::CallOption::default())
    }

    pub fn multi_ingest_async_opt(&self, req: &super::import_sstpb::MultiIngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::IngestResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_MULTI_INGEST, req, opt)
    }

    pub fn multi_ingest_async(&self, req: &super::import_sstpb::MultiIngestRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::IngestResponse>> {
        self.multi_ingest_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn duplicate_detect_opt(&self, req: &super::import_sstpb::DuplicateDetectRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::import_sstpb::DuplicateDetectResponse>> {
        self.client.server_streaming(&METHOD_IMPORT_SST_DUPLICATE_DETECT, req, opt)
    }

    pub fn duplicate_detect(&self, req: &super::import_sstpb::DuplicateDetectRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::import_sstpb::DuplicateDetectResponse>> {
        self.duplicate_detect_opt(req, ::grpcio::CallOption::default())
    }

    pub fn apply_opt(&self, req: &super::import_sstpb::ApplyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::ApplyResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_APPLY, req, opt)
    }

    pub fn apply(&self, req: &super::import_sstpb::ApplyRequest) -> ::grpcio::Result<super::import_sstpb::ApplyResponse> {
        self.apply_opt(req, ::grpcio::CallOption::default())
    }

    pub fn apply_async_opt(&self, req: &super::import_sstpb::ApplyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::ApplyResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_APPLY, req, opt)
    }

    pub fn apply_async(&self, req: &super::import_sstpb::ApplyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::ApplyResponse>> {
        self.apply_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn clear_files_opt(&self, req: &super::import_sstpb::ClearRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import_sstpb::ClearResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_CLEAR_FILES, req, opt)
    }

    pub fn clear_files(&self, req: &super::import_sstpb::ClearRequest) -> ::grpcio::Result<super::import_sstpb::ClearResponse> {
        self.clear_files_opt(req, ::grpcio::CallOption::default())
    }

    pub fn clear_files_async_opt(&self, req: &super::import_sstpb::ClearRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::ClearResponse>> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_CLEAR_FILES, req, opt)
    }

    pub fn clear_files_async(&self, req: &super::import_sstpb::ClearRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::import_sstpb::ClearResponse>> {
        self.clear_files_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ImportSst {
    fn switch_mode(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::SwitchModeRequest, sink: ::grpcio::UnarySink<super::import_sstpb::SwitchModeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn upload(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::import_sstpb::UploadRequest>, sink: ::grpcio::ClientStreamingSink<super::import_sstpb::UploadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn ingest(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::IngestRequest, sink: ::grpcio::UnarySink<super::import_sstpb::IngestResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn compact(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::CompactRequest, sink: ::grpcio::UnarySink<super::import_sstpb::CompactResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_download_speed_limit(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::SetDownloadSpeedLimitRequest, sink: ::grpcio::UnarySink<super::import_sstpb::SetDownloadSpeedLimitResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn download(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::DownloadRequest, sink: ::grpcio::UnarySink<super::import_sstpb::DownloadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn write(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::import_sstpb::WriteRequest>, sink: ::grpcio::ClientStreamingSink<super::import_sstpb::WriteResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_write(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::import_sstpb::RawWriteRequest>, sink: ::grpcio::ClientStreamingSink<super::import_sstpb::RawWriteResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn multi_ingest(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::MultiIngestRequest, sink: ::grpcio::UnarySink<super::import_sstpb::IngestResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn duplicate_detect(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::DuplicateDetectRequest, sink: ::grpcio::ServerStreamingSink<super::import_sstpb::DuplicateDetectResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn apply(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::ApplyRequest, sink: ::grpcio::UnarySink<super::import_sstpb::ApplyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn clear_files(&mut self, ctx: ::grpcio::RpcContext, _req: super::import_sstpb::ClearRequest, sink: ::grpcio::UnarySink<super::import_sstpb::ClearResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_import_sst<S: ImportSst + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_SWITCH_MODE, move |ctx, req, resp| {
        instance.switch_mode(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_SST_UPLOAD, move |ctx, req, resp| {
        instance.upload(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_INGEST, move |ctx, req, resp| {
        instance.ingest(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT, move |ctx, req, resp| {
        instance.set_download_speed_limit(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_DOWNLOAD, move |ctx, req, resp| {
        instance.download(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_SST_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_SST_RAW_WRITE, move |ctx, req, resp| {
        instance.raw_write(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_MULTI_INGEST, move |ctx, req, resp| {
        instance.multi_ingest(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_IMPORT_SST_DUPLICATE_DETECT, move |ctx, req, resp| {
        instance.duplicate_detect(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_APPLY, move |ctx, req, resp| {
        instance.apply(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_CLEAR_FILES, move |ctx, req, resp| {
        instance.clear_files(ctx, req, resp)
    });
    builder.build()
}
