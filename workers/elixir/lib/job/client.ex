defmodule Bossman.Job.Client do
  alias Bossman.Protobuf.V1alpha1.Options
  use GRPC.Server, service: Bossman.Protobuf.V1alpha1.JobService.Service

  @spec perform(%{value: String.t()}, %{value: String.t()}, Options.t()) :: {:ok, any}
  def perform(name, docker_image_name, options) do
    with {:connect, {:ok, channel}} <- {:connect, GRPC.Stub.connect("localhost:50051")},
         request <-
           Bossman.Protobuf.V1alpha1.Job.PerformRequest.new(
             name: name,
             docker_image_name: docker_image_name,
             options: options
           ),
         {:reply, {:ok, reply}} <-
           {:reply, Bossman.Protobuf.V1alpha1.JobService.Stub.perform(channel, request)} do
      Task.start(fn -> GRPC.Stub.disconnect(channel) end)
      {:ok, reply}
    end
  end

  def get(id) do
    {:ok, channel} = GRPC.Stub.connect("localhost:50051")

    request = Bossman.Protobuf.V1alpha1.Job.GetRequest.new(id: id)
    {:ok, reply} = Bossman.Protobuf.V1alpha1.JobService.Stub.get(channel, request)

    Task.start(fn -> GRPC.Stub.disconnect(channel) end)
    reply.job
  end
end
