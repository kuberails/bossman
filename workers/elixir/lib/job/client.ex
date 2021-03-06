defmodule Bossman.Job.Client do
  alias Bossman.Protobuf.V1alpha1.Options
  alias Bossman.Job.Error

  @spec perform(%{value: String.t()}, %{value: String.t()}, Options.t()) :: {:ok, any}
  def perform(name, docker_image_name, options) do
    connect_and_do(fn channel ->
      request =
        Bossman.Protobuf.V1alpha1.Job.PerformRequest.new(
          name: name,
          docker_image_name: docker_image_name,
          options: options
        )

      Bossman.Protobuf.V1alpha1.JobService.Stub.perform(channel, request)
    end)
  end

  def get(id) do
    connect_and_do(fn channel ->
      request = Bossman.Protobuf.V1alpha1.Job.GetRequest.new(id: id)
      Bossman.Protobuf.V1alpha1.JobService.Stub.get(channel, request)
    end)
  end

  def get_status(id) do
    connect_and_do(fn channel ->
      request = Bossman.Protobuf.V1alpha1.Job.GetRequest.new(id: id)
      Bossman.Protobuf.V1alpha1.JobService.Stub.get_status(channel, request)
    end)
  end

  def get_all() do
    connect_and_do(fn channel ->
      request = Bossman.Protobuf.V1alpha1.Job.GetAllRequest.new()
      Bossman.Protobuf.V1alpha1.JobService.Stub.get_all(channel, request)
    end)
  end

  def get_list(name) do
    connect_and_do(fn channel ->
      request = Bossman.Protobuf.V1alpha1.Job.GetListRequest.new(name: name)
      Bossman.Protobuf.V1alpha1.JobService.Stub.get_list(channel, request)
    end)
  end

  defp connect_and_do(func_block) do
    with {:connect, {:ok, channel}} <- {:connect, GRPC.Stub.connect("localhost:50051")},
         {:reply, {:ok, reply}} <- {:reply, func_block.(channel)} do
      Task.start(fn -> GRPC.Stub.disconnect(channel) end)
      {:ok, reply}
    else
      {:reply, {:error, e = %GRPC.RPCError{}}} ->
        {:error, Error.new(e)}

      error ->
        error
    end
  end
end
