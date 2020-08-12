defmodule Bossman.Job do
  alias Bossman.Job.Client
  alias Bossman.Job.Options

  @spec perform(String.t(), String.t(), map()) :: {:ok, any} | {:error, any}
  def perform(name, docker_image_name, options \\ %{}) do
    with options <- Options.new(options),
         {:ok, options} <- Options.encode(options),
         {:ok, reply} <- Client.perform(%{value: name}, %{value: docker_image_name}, options) do
      {:ok, reply}
    else
      error -> error
    end
  end

  def get(id) do
    Client.get(id)
  end
end
