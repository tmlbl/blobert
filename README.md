blobert
=======

Blobert is a minimal OCI registry written in Rust.

This is currently just an experiment to learn Rust and evaluate whether it can
offer any advantage over the official registry.

Push and pull are currently supported, with no TLS or authentication.

```bash
# Pull an image from hub
tim@doulos ~> docker pull nats
Using default tag: latest
latest: Pulling from library/nats
5e5797f39fa0: Pull complete
28c3b732adf6: Pull complete
Digest: sha256:9686d73262f91e44c10b838eecbab53e705d2f4be5718efe92dfdf9f86f4e786
Status: Downloaded newer image for nats:latest
docker.io/library/nats:latest

# Tag with localhost or 127.0.0.1. Blobert runs on port 7000
tim@doulos ~ > docker tag nats 127.0.0.1:7000/nats

# Push the image to blobert
tim@doulos ~> docker push 127.0.0.1:7000/nats
Using default tag: latest
The push refers to repository [127.0.0.1:7000/nats]
f22e8439b53a: Pushed
7e49c6f1bf90: Pushed
latest: digest: sha256:211e543a39d6378c483852a76b78a114bb26bdbe40a7aeda3daae61c62cbcf59 size: 715
```
