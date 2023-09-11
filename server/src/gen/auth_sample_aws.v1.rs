// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignUpRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub birthday: i64,
    #[prost(string, tag="4")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub password_confirm: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignUpResponse {
}
/// Encoded file descriptor set for the `auth_sample_aws.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa0, 0x07, 0x0a, 0x28, 0x61, 0x75, 0x74, 0x68, 0x2d, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x2d, 0x61, 0x77, 0x73, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x73, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x61,
    0x75, 0x74, 0x68, 0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76,
    0x31, 0x22, 0x9c, 0x01, 0x0a, 0x0d, 0x53, 0x69, 0x67, 0x6e, 0x55, 0x70, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x12, 0x1a, 0x0a,
    0x08, 0x62, 0x69, 0x72, 0x74, 0x68, 0x64, 0x61, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x08, 0x62, 0x69, 0x72, 0x74, 0x68, 0x64, 0x61, 0x79, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x61, 0x73,
    0x73, 0x77, 0x6f, 0x72, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x70, 0x61, 0x73,
    0x73, 0x77, 0x6f, 0x72, 0x64, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0f, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x72, 0x6d,
    0x22, 0x10, 0x0a, 0x0e, 0x53, 0x69, 0x67, 0x6e, 0x55, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x32, 0x67, 0x0a, 0x14, 0x41, 0x75, 0x74, 0x68, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65,
    0x41, 0x77, 0x73, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x4f, 0x0a, 0x06, 0x53, 0x69,
    0x67, 0x6e, 0x55, 0x70, 0x12, 0x21, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x73, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x55, 0x70,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x22, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x73,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x61, 0x77, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x69, 0x67,
    0x6e, 0x55, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x8d, 0x01, 0x0a, 0x16,
    0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f,
    0x61, 0x77, 0x73, 0x2e, 0x76, 0x31, 0x42, 0x12, 0x41, 0x75, 0x74, 0x68, 0x53, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x41, 0x77, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x41,
    0x58, 0x58, 0xaa, 0x02, 0x10, 0x41, 0x75, 0x74, 0x68, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41,
    0x77, 0x73, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x10, 0x41, 0x75, 0x74, 0x68, 0x53, 0x61, 0x6d, 0x70,
    0x6c, 0x65, 0x41, 0x77, 0x73, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1c, 0x41, 0x75, 0x74, 0x68, 0x53,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x11, 0x41, 0x75, 0x74, 0x68, 0x53, 0x61,
    0x6d, 0x70, 0x6c, 0x65, 0x41, 0x77, 0x73, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xad, 0x03, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x10, 0x19, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00,
    0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x04,
    0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x08, 0x3b, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x0c, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x13, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x2b, 0x39, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x08, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x0f, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0a, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0a, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x0f,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x17, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0b, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x0e, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x0b, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x0c, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0c, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x0f, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0d, 0x0f, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x0d, 0x22, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x00, 0x19, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("auth_sample_aws.v1.serde.rs");
include!("auth_sample_aws.v1.tonic.rs");
// @@protoc_insertion_point(module)