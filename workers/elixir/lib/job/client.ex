defmodule Bossman.Job.Client do
  alias Bossman.Protobuf.V1alpha1.Options
  use GRPC.Server, service: Bossman.Protobuf.V1alpha1.JobService.Service

  @spec perform(%{value: String.t()}, %{value: String.t()}, Options.t()) :: {:ok, any}
  def perform(name, docker_image_name, options) do
    {:ok, channel} = GRPC.Stub.connect("localhost:50051")

    request =
      Bossman.Protobuf.V1alpha1.Job.PerformRequest.new(
        name: name,
        docker_image_name: docker_image_name,
        options: options
      )

    {:ok, reply} = Bossman.Protobuf.V1alpha1.JobService.Stub.perform(channel, request)

    Task.start(fn -> GRPC.Stub.disconnect(channel) end)

    {:ok, reply}
  end

  def get() do
    {:ok, channel} = GRPC.Stub.connect("localhost:50051")

    request = Bossman.Protobuf.V1alpha1.Job.GetRequest.new(id: "get-id")
    {:ok, reply} = Bossman.Protobuf.V1alpha1.JobService.Stub.get(channel, request)

    Task.start(fn -> GRPC.Stub.disconnect(channel) end)
    reply.job
  end
end
