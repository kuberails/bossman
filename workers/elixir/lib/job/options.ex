defmodule Bossman.Job.Options do
  alias Bossman.Job.Options
  alias Bossman.Job.Options.Encode
  alias Bossman.Job.Options.Decode
  use TypedStruct

  @optional_fields [
    :namespace,
    :image_pull_secrets,
    :retries,
    :completions,
    :parallelism,
    :timeout
  ]

  @doc false
  def optional_fields(), do: @optional_fields

  typedstruct do
    @typedoc "Options"
    field :namespace, String.t()

    field :image_pull_secrets, String.t()
    field :annotations, %{String.t() => String.t()}

    field :retries, integer(), default: 5
    field :completions, integer(), default: 1
    field :parallelism, integer(), default: 1

    field :timeout, integer
    field :args, [String.t()]
    field :command, [String.t()]

    field :env, Env.t(), default: []
    field :env_from, EnvFrom.t(), default: []
  end

  defmodule Env do
    @type t ::
            %{name: String.t(), value: String.t()}
            | %{
                type: :secret_key_ref,
                name: String.t(),
                secret_name: String.t(),
                secret_key: String.t()
              }
            | %{
                type: :config_map_key_ref,
                name: String.t(),
                config_map_name: String.t(),
                config_map_key: String.t()
              }
  end

  defmodule EnvFrom do
    @type t ::
            %{type: :secret_key_ref, name: String.t()}
            | %{type: :config_map_key_ref, name: String.t()}
  end

  @spec new(map) :: Options.t()
  def new(options \\ %{}) do
    struct(Options, options)
  end

  @spec encode!(Options.t()) :: Bossman.Protobuf.V1alpha1.Options.t()
  def encode!(options), do: Encode.encode!(options)

  @spec encode(Options.t()) :: {:ok, Bossman.Protobuf.V1alpha1.Options.t()} | {:error, String.t()}
  def encode(options), do: Encode.encode(options)

  @spec decode!(Bossman.Protobuf.V1alpha1.Options.t()) :: Options.t()
  def decode!(options), do: Decode.decode!(options)

  @spec decode(Bossman.Protobuf.V1alpha1.Options.t()) :: {:ok, Options.t()} | {:error, String.t()}
  def decode(options), do: Decode.decode(options)
end
