defmodule Bossman.Job.Client do
  use GRPC.Server, service: Bossman.Protobuf.V1alpha1.JobService.Service

  def get() do
    {:ok, channel} = GRPC.Stub.connect("localhost:50051")

    request = Bossman.Protobuf.V1alpha1.Job.GetRequest.new(id: "get-id")
    {:ok, reply} = channel |> Bossman.Protobuf.V1alpha1.JobService.Stub.get(request)
    IO.inspect(reply)
  end
end
