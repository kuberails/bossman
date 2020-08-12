defmodule Bossman.Job do
  alias Bossman.Job.{Client, Options, Decode}
  use TypedStruct

  typedstruct do
    field :id, String.t()
    field :name, String.t(), required: true

    field :docker_image_name, String.t(), required: true
    field :options, Options.t(), required: true
    field :status, Bossman.Protobuf.V1alpha1.Job.Status.t()
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

  def get(id) do
    with {:ok, reply} <- Client.get(id),
         {:ok, job} <- Decode.decode(reply.job) do
      {:ok, job}
    end
  end
end
