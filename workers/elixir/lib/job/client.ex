defmodule Bossman.Job.Client do
  use GRPC.Server, service: Bossman.Protobuf.V1alpha1.JobService.Service

  def perform() do
    {:ok, channel} = GRPC.Stub.connect("localhost:50051")

    request = Bossman.Protobuf.V1alpha1.Job.PerformRequest.new(name: %{value: "samplejob"})

    {:ok, reply} = Bossman.Protobuf.V1alpha1.JobService.Stub.perform(channel, request)

    Task.start(fn -> GRPC.Stub.disconnect(channel) end)
    reply
  end

  def get() do
    {:ok, channel} = GRPC.Stub.connect("localhost:50051")

    request = Bossman.Protobuf.V1alpha1.Job.GetRequest.new(id: "get-id")
    {:ok, reply} = Bossman.Protobuf.V1alpha1.JobService.Stub.get(channel, request)

    Task.start(fn -> GRPC.Stub.disconnect(channel) end)
    reply.job
  end
end
