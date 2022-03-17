use std::collections::HashMap;

// /// The mediatype for WASM layers.
// pub const WASM_LAYER_MEDIA_TYPE: &str = "application/vnd.wasm.content.layer.v1+wasm";
// /// The mediatype for a WASM image config.
// pub const WASM_CONFIG_MEDIA_TYPE: &str = "application/vnd.wasm.config.v1+json";
/// The mediatype for an OCI manifest.
pub const IMAGE_MANIFEST_MEDIA_TYPE: &str = "application/vnd.docker.distribution.manifest.v2+json";
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
    pub config: Descriptor,
    pub layers: Vec<Descriptor>,
    pub annotations: Option<HashMap<String, String>>,
}

impl Manifest {
    pub fn digest(&self) -> String {
        crate::util::sha256_digest(&serde_json::to_vec(self).unwrap())
    }
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
    pub media_type: String,
    pub digest: String,
    pub size: Option<i64>,
    pub urls: Option<Vec<String>>,
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
