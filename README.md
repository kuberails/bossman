# BossMan

A language agnostic job runner for kubernetes. Run compute heavy tasks as kubernetes jobs and get autoscaling for free.

Bossman contains two parts, the bossman_server (written in rust) and the client (worker), which can be written in any language. Communication between bossman_server and clients are done via gRPC.

As a result, you can easily generate most of the code needed to implement a worker in your favourite language using [a protobuf cli](https://github.com/protocolbuffers/protobuf/) (`brew install protobuf` on MacOS) and the protobuf files found in the [protos folder](/bossman_server/protos).

### Provided clients

- [elixir client](/workers/elixir)
- go client [_(coming soon, contributions welcome ...)_](#18)
- python client [_(coming soon, contributions welcome ...)_](#19)
- php client [_(coming soon, contributions welcome ...)_](#20)
- rust client [_(coming soon, contributions welcome ...)_](#21)
- ruby client [_(coming soon, contributions welcome ...)_](#22)

## License

Apache 2.0 licensed. See [LICENSE](LICENSE.md) for details.
