# BossMan

A language agnostic job runner for kubernetes. Run compute heavy tasks as kubernetes jobs and get autoscaling for free.

Bossman contains two parts, the `bossman_server` (written in rust) and the client (`worker`), which can be written in any language. Communication between bossman_server and clients are done via gRPC.

As a result, you can easily generate most of the code needed to implement a worker in your favourite language using [a protobuf cli](https://github.com/protocolbuffers/protobuf/) (`brew install protobuf` on MacOS) and the protobuf files found in the [protos folder](/bossman_server/protos).

### Provided clients

- [elixir client](/workers/elixir/)
- go client [_(coming soon, contributions welcome ...)_](#18)
- python client [_(coming soon, contributions welcome ...)_](#19)
- php client [_(coming soon, contributions welcome ...)_](#20)
- rust client [_(coming soon, contributions welcome ...)_](#21)
- ruby client [_(coming soon, contributions welcome ...)_](#22)
- javascript client [_(coming soon, contributions welcome ...)_](#25)

## What?

`bossman_server` is a extremely lightweight server that lives in your kubernetes cluster.

The `worker` is a library written in your favourite language which makes it easy to communicate with the server via gRPC.

You can use Bossman to easily spin up kubernetes `v1/batch` jobs and to check the status of those jobs.

### Example (using elixir worker)

The simplest way to create a job is with the `perform` function. For this function you need to give the job a group name and a the docker image name. You can optionally also provide some kubernetes options.

- **job group name** – any user provided string that identifies similar jobs, this can be used with the `get_list` function get a list of all the jobs with the same group name
- **docker image name** – this refers to the docker image the job will run
- **options (optional)** – kubernetes options, see a list of available options below

```elixir
Bossman.Job.perform("png-conversion", "myregister/png-to-jpg-converter")
```

Usually you will need to set some options to get anything useful for example:

```elixir
Bossman.Job.perform(
  "png-conversion",
  "docker.pkg.github.com/kuberails/bossman/png-to-jpg-converter:master",
  %{
    image_pull_secrets: "github",
    namespace: "default2",
    env: %{name: "IMAGE_NAME", value: "cute_kitty.png"}
  }
)
```

### Functions

- **`perform(group_name, docker_image, // options)`** – perform a job. Creates the job in kubernetes. Returns the job with an id which is a generated uuid.
- **`get(id)`** – get the job with the id (the generated uuid returned from the perform function)
- **`get_status(id)`** – get the status of the job
- **`get_list(group_name)`** – returns a list of jobs with the given group name
- **`get_all()`** – returns all jobs managed by BossMan

## License

Apache 2.0 licensed. See [LICENSE](LICENSE.md) for details.
