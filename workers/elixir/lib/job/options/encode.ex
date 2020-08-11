defmodule Bossman.Job.Options.Encode do
  alias Bossman.Job.Options

  defmodule Error do
    defexception [:message]
  end

  @spec encode(Options.t()) :: {:ok, Bossman.Protobuf.V1alpha1.Options.t()} | {:error, String.t()}
  def encode(options) do
    try do
      {:ok, encode!(options)}
    catch
      error ->
        {:error, %Error{message: "Unable to encode options: #{inspect(error)}"}}
    end
  end

  @spec encode!(Options.t()) :: Bossman.Protobuf.V1alpha1.Options.t()
  def encode!(options) do
    env = encode_env(options.env)
    env_from = encode_env_from(options.env_from)

    options =
      options
      |> Map.from_struct()
      |> Map.take(Options.optional_fields())
      |> Enum.map(fn {key, value} -> {key, encode_optional(value)} end)
      |> Enum.into(%{})

    options
    |> Map.merge(%{env: env, env_from: env_from})
    |> Bossman.Protobuf.V1alpha1.Options.new()
  end

  defp encode_optional(nil), do: nil
  defp encode_optional(value), do: %{value: value}

  defp encode_env(envs) when is_list(envs), do: Enum.map(envs, &encode_env/1)

  defp encode_env(%{name: name, value: value}) do
    %{env: {:value, %{name: name, value: value}}}
  end

  defp encode_env(%{
         type: :secret_key_ref,
         name: name,
         secret_name: secret_key_name,
         secret_key: secret_key
       }) do
    %{
      env:
        {:value_from,
         %{name: name, value_from: {:secret_key_ref, %{name: secret_key_name, key: secret_key}}}}
    }
  end

  defp encode_env(%{
         type: :config_map_key_ref,
         name: name,
         config_map_name: config_map_key_name,
         config_map_key: config_map_key
       }) do
    %{
      env:
        {:value_from,
         %{
           name: name,
           value_from: {:config_map_key_ref, %{name: config_map_key_name, key: config_map_key}}
         }}
    }
  end

  defp encode_env_from(env_from) when is_list(env_from),
    do: Enum.map(env_from, &encode_env_from/1)

  defp encode_env_from(%{type: :secret_key_ref, name: name}) do
    %{env_from: {:secret_key_ref, %{name: name}}}
  end

  defp encode_env_from(%{type: :config_map_key_ref, name: name}) do
    %{env_from: {:config_map_key_ref, %{name: name}}}
  end
end
