defmodule Bossman.Job.Options.Decode do
  alias Bossman.Job.Options

  defmodule Error do
    defexception [:message]
  end

  @spec decode(Bossman.Protobuf.V1alpha1.Options.t()) :: {:ok, Options.t()} | {:error, String.t()}
  def decode(options) do
    try do
      {:ok, decode!(options)}
    catch
      error ->
        {:error, %Error{message: "Unable to decode options: #{inspect(error)}"}}
    end
  end

  @spec decode!(Bossman.Protobuf.V1alpha1.Options.t()) :: Options.t()
  def decode!(options) do
    env = decode_env(options.env)
    env_from = decode_env_from(options.env_from)

    options =
      options
      |> Map.from_struct()
      |> Map.take(Options.optional_fields())
      |> Enum.map(fn {key, value} -> {key, decode_optional(value)} end)
      |> Enum.into(%{})

    options
    |> Map.merge(%{env: env, env_from: env_from})
    |> Options.new()
  end

  defp decode_optional(nil), do: nil
  defp decode_optional(%_{value: value}), do: value

  defp decode_env(envs) when is_list(envs), do: Enum.map(envs, &decode_env/1)

  defp decode_env(%{env: {:value, %{name: name, value: value}}}) do
    %{name: name, value: value}
  end

  defp decode_env(%{
         env:
           {:value_from,
            %{
              name: name,
              value_from: {:secret_key_ref, %{name: secret_key_name, key: secret_key}}
            }}
       }) do
    %{
      type: :secret_key_ref,
      name: name,
      secret_name: secret_key_name,
      secret_key: secret_key
    }
  end

  defp decode_env(%{
         env:
           {:value_from,
            %{
              name: name,
              value_from: {:config_map_key_ref, %{name: config_map_key_name, key: config_map_key}}
            }}
       }) do
    %{
      type: :config_map_key_ref,
      name: name,
      config_map_name: config_map_key_name,
      config_map_key: config_map_key
    }
  end

  defp decode_env_from(env_from) when is_list(env_from),
    do: Enum.map(env_from, &decode_env_from/1)

  defp decode_env_from(%_{env_from: {:secret_key_ref, %_{name: name}}}) do
    %{type: :secret_key_ref, name: name}
  end

  defp decode_env_from(%_{env_from: {:config_map_key_ref, %_{name: name}}}) do
    %{type: :config_map_key_ref, name: name}
  end
end
