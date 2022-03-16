use std::collections::HashMap;

// /// The mediatype for WASM layers.
// pub const WASM_LAYER_MEDIA_TYPE: &str = "application/vnd.wasm.content.layer.v1+wasm";
// /// The mediatype for a WASM image config.
// pub const WASM_CONFIG_MEDIA_TYPE: &str = "application/vnd.wasm.config.v1+json";
// /// The mediatype for an OCI manifest.
// pub const IMAGE_MANIFEST_MEDIA_TYPE: &str = "application/vnd.docker.distribution.manifest.v2+json";
/// The mediatype for an image config (manifest).
pub const IMAGE_CONFIG_MEDIA_TYPE: &str = "application/vnd.oci.image.config.v1+json";
// /// The mediatype that Docker uses for image configs.
// pub const IMAGE_DOCKER_CONFIG_MEDIA_TYPE: &str = "application/vnd.docker.container.image.v1+json";
// /// The mediatype for a layer.
// pub const IMAGE_LAYER_MEDIA_TYPE: &str = "application/vnd.oci.image.layer.v1.tar";
// /// The mediatype for a layer that is gzipped.
// pub const IMAGE_LAYER_GZIP_MEDIA_TYPE: &str = "application/vnd.oci.image.layer.v1.tar+gzip";
// /// The mediatype that Docker uses for a layer that is gzipped.
// pub const IMAGE_DOCKER_LAYER_GZIP_MEDIA_TYPE: &str =
//     "application/vnd.docker.image.rootfs.diff.tar.gzip";
// /// The mediatype for a layer that is nondistributable.
// pub const IMAGE_LAYER_NONDISTRIBUTABLE_MEDIA_TYPE: &str =
//     "application/vnd.oci.image.layer.nondistributable.v1.tar";
// /// The mediatype for a layer that is nondistributable and gzipped.
// pub const IMAGE_LAYER_NONDISTRIBUTABLE_GZIP_MEDIA_TYPE: &str =
//     "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip";

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub schema_version: u8,

    pub media_type: Option<String>,
    /// The image configuration.
    ///
    /// This object is required.
    pub config: Descriptor,

    /// The OCI image layers
    ///
    /// The specification is unclear whether this is required. We have left it
    /// required, assuming an empty vector can be used if necessary.
    pub layers: Vec<Descriptor>,

    /// The annotations for this manifest
    ///
    /// The specification says "If there are no annotations then this property
    /// MUST either be absent or be an empty map."
    /// TO accomodate either, this is optional.
    pub annotations: Option<HashMap<String, String>>,
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest {
            schema_version: 2,
            media_type: None,
            config: Descriptor::default(),
            layers: vec![],
            annotations: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptor {
    /// The media type of this descriptor.
    ///
    /// Layers, config, and manifests may all have descriptors. Each
    /// is differentiated by its mediaType.
    ///
    /// This REQUIRED property contains the media type of the referenced
    /// content. Values MUST comply with RFC 6838, including the naming
    /// requirements in its section 4.2.
    pub media_type: String,
    /// The SHA 256 or 512 digest of the object this describes.
    ///
    /// This REQUIRED property is the digest of the targeted content, conforming
    /// to the requirements outlined in Digests. Retrieved content SHOULD be
    /// verified against this digest when consumed via untrusted sources.
    pub digest: String,
    /// The size, in bytes, of the object this describes.
    ///
    /// This is supposed to be required, but Docker client has a habit of not
    /// setting it
    pub size: Option<i64>,
    /// This OPTIONAL property specifies a list of URIs from which this
    /// object MAY be downloaded. Each entry MUST conform to RFC 3986.
    /// Entries SHOULD use the http and https schemes, as defined in RFC 7230.
    pub urls: Option<Vec<String>>,

    /// This OPTIONAL property contains arbitrary metadata for this descriptor.
    /// This OPTIONAL property MUST use the annotation rules.
    /// https://github.com/opencontainers/image-spec/blob/master/annotations.md#rules
    pub annotations: Option<HashMap<String, String>>,
}

impl Default for Descriptor {
    fn default() -> Self {
        Descriptor {
            media_type: IMAGE_CONFIG_MEDIA_TYPE.to_owned(),
            digest: "".to_owned(),
            size: Some(0),
            urls: None,
            annotations: None,
        }
    }
}
