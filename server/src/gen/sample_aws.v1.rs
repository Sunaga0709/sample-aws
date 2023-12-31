// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleMethodRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleMethodResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadImageRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub image_data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadImageResponse {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `sample_aws.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc7, 0x07, 0x0a, 0x1e, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2d, 0x61, 0x77, 0x73, 0x2f,
    0x76, 0x31, 0x2f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e,
    0x76, 0x31, 0x22, 0x16, 0x0a, 0x14, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x4d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x17, 0x0a, 0x15, 0x45, 0x78,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0x47, 0x0a, 0x12, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x49, 0x6d, 0x61,
    0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x44, 0x61, 0x74, 0x61, 0x22, 0x2f, 0x0a, 0x13,
    0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x32, 0xc4, 0x01,
    0x0a, 0x10, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x53, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x12, 0x5a, 0x0a, 0x0d, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x4d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x12, 0x23, 0x2e, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73,
    0x2e, 0x76, 0x31, 0x2e, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x4d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x24, 0x2e, 0x73, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x4d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x54,
    0x0a, 0x0b, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x12, 0x21, 0x2e,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x55, 0x70,
    0x6c, 0x6f, 0x61, 0x64, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x22, 0x2e, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76, 0x31,
    0x2e, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x49, 0x6d, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x42, 0x74, 0x0a, 0x11, 0x63, 0x6f, 0x6d, 0x2e, 0x73, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76, 0x31, 0x42, 0x0e, 0x53, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x41, 0x77, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x53, 0x58,
    0x58, 0xaa, 0x02, 0x0c, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x2e, 0x56, 0x31,
    0xca, 0x02, 0x0c, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x5c, 0x56, 0x31, 0xe2,
    0x02, 0x18, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x5c, 0x56, 0x31, 0x5c, 0x47,
    0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x0d, 0x53, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xa5, 0x03, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12,
    0x04, 0x04, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x08, 0x50, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x0c, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x1a, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x39, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x06, 0x08, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x06, 0x0c, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x06, 0x18,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x35, 0x48, 0x0a,
    0x09, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x03, 0x09, 0x00, 0x1f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0b, 0x00,
    0x20, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x0d, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x0d, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0e,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x08, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x0f, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0f, 0x0f, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0f, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x12, 0x00, 0x14, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x12, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x13, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x13, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x13, 0x0f, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13,
    0x19, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("sample_aws.v1.serde.rs");
include!("sample_aws.v1.tonic.rs");
// @@protoc_insertion_point(module)