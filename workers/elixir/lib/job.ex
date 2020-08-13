defmodule Bossman.Job do
  alias Bossman.Job.{Client, Options, Decode}
  alias Bossman.Util.Result
  use TypedStruct

  typedstruct do
    field :id, String.t()
    field :name, String.t(), required: true

    field :docker_image_name, String.t(), required: true
    field :options, Options.t(), required: true
    field :status, Job.Status.t()
  end

  defmodule Status do
    @type t :: Bossman.Protobuf.V1alpha1.Job.Status.t()
  end

  @spec perform(String.t(), String.t(), map()) :: {:ok, any} | {:error, any}
  def perform(name, docker_image_name, options \\ %{}) do
    with options <- Options.new(options),
         {:ok, options} <- Options.encode(options),
         {:ok, reply} <- Client.perform(%{value: name}, %{value: docker_image_name}, options),
         {:ok, job} <- Decode.decode(reply.job) do
      {:ok, job}
    else
      error -> error
    end
  end

  @spec get(String.t()) :: {:ok, Job.t()} | {:error, any}
  def get(id) do
    with {:ok, reply} <- Client.get(id),
         {:ok, job} <- Decode.decode(reply.job) do
      {:ok, job}
    else
      error -> error
    end
  end

  @spec get_list(String.t()) :: {:ok, [Job.t()]} | {:error, any}
  def get_list(job_name) do
    with {:ok, reply} <- Client.get_list(job_name),
         jobs <- Enum.map(reply.jobs, &Decode.decode/1),
         {:ok, jobs} <- Result.filter_map(jobs) do
      {:ok, jobs}
    else
      error -> error
    end
  end

  @spec get_status(String.t()) :: {:ok, Job.Status.t()} | {:error, any}
  def get_status(id) do
    with {:ok, reply} <- Client.get_status(id) do
      {:ok, reply.status}
    else
      error -> error
    end
  end
end
