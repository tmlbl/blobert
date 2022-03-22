blobert
=======

Blobert is an OCI registry designed for speed and efficiency in bare-metal
environments.

This repository is a work in progress.

# Goals

- In-memory caching of layers for fast concurrent pulls
- Cluster model with metadata servers and blob servers, with redundancy of blob
layers and pulls distributed among multiple blob servers

# TODOS

- Custom error type to match OCI spec for errors
- Make BlobStream buffer size configurable and reuse buffer
- Add blob stat function and "repair" manifests from Docker missing size
- Add TLS
- Implement blob cache
