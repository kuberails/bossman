defmodule Bossman.Job.Decode do
  alias Bossman.Job
  alias Bossman.Job.Options

  defmodule Error do
    defexception [:message]
  end

  @spec decode(Bossman.Protobuf.V1alpha1.Job.t()) :: {:ok, Job.t()} | {:error, any()}
  def decode(job) do
    try do
      {:ok, decode!(job)}
    catch
      error ->
        {:error, %Error{message: "Unable to decode job: #{inspect(error)}"}}
    end
  end

  @spec decode!(Bossman.Protobuf.V1alpha1.Job.t()) :: Job.t()
  def decode!(job) do
    %Job{
      docker_image_name: job.docker_image_name,
      id: job.id,
      name: job.name,
      options: Options.decode!(job.options),
      status: job.status
    }
  end
end
