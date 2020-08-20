defmodule Bossman.Protobuf.V1alpha1.Options.AnnotationsEntry do
  @moduledoc false
  use Protobuf, map: true, syntax: :proto3

  @type t :: %__MODULE__{
          key: String.t(),
          value: String.t()
        }
  defstruct [:key, :value]

  field :key, 1, type: :string
  field :value, 2, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.Env.EnvFrom do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          value_from: {atom, any},
          name: String.t()
        }
  defstruct [:value_from, :name]

  oneof :value_from, 0
  field :name, 1, type: :string
  field :secret_key_ref, 2, type: Bossman.Protobuf.V1alpha1.Options.Env.SecretKeyRef, oneof: 0

  field :config_map_key_ref, 3,
    type: Bossman.Protobuf.V1alpha1.Options.Env.ConfigMapKeyRef,
    oneof: 0
end

defmodule Bossman.Protobuf.V1alpha1.Options.Env.EnvValue do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t(),
          value: String.t()
        }
  defstruct [:name, :value]

  field :name, 1, type: :string
  field :value, 2, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.Env.ConfigMapKeyRef do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t(),
          key: String.t()
        }
  defstruct [:name, :key]

  field :name, 2, type: :string
  field :key, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.Env.SecretKeyRef do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t(),
          key: String.t()
        }
  defstruct [:name, :key]

  field :name, 2, type: :string
  field :key, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.Env do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          env: {atom, any}
        }
  defstruct [:env]

  oneof :env, 0
  field :value, 2, type: Bossman.Protobuf.V1alpha1.Options.Env.EnvValue, oneof: 0
  field :value_from, 3, type: Bossman.Protobuf.V1alpha1.Options.Env.EnvFrom, oneof: 0
end

defmodule Bossman.Protobuf.V1alpha1.Options.EnvFrom.ConfigMapKeyRef do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.EnvFrom.SecretKeyRef do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.EnvFrom do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          env_from: {atom, any}
        }
  defstruct [:env_from]

  oneof :env_from, 0
  field :secret_key_ref, 1, type: Bossman.Protobuf.V1alpha1.Options.EnvFrom.SecretKeyRef, oneof: 0

  field :config_map_key_ref, 2,
    type: Bossman.Protobuf.V1alpha1.Options.EnvFrom.ConfigMapKeyRef,
    oneof: 0
end

defmodule Bossman.Protobuf.V1alpha1.Options do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          timeout: Google.Protobuf.Int64Value.t() | nil,
          image_pull_secrets: Google.Protobuf.StringValue.t() | nil,
          annotations: %{String.t() => String.t()},
          namespace: Google.Protobuf.StringValue.t() | nil,
          retries: Google.Protobuf.Int32Value.t() | nil,
          completions: Google.Protobuf.Int32Value.t() | nil,
          parallelism: Google.Protobuf.Int32Value.t() | nil,
          args: [String.t()],
          command: [String.t()],
          env: [Bossman.Protobuf.V1alpha1.Options.Env.t()],
          env_from: [Bossman.Protobuf.V1alpha1.Options.EnvFrom.t()]
        }
  defstruct [
    :timeout,
    :image_pull_secrets,
    :annotations,
    :namespace,
    :retries,
    :completions,
    :parallelism,
    :args,
    :command,
    :env,
    :env_from
  ]

  field :timeout, 1, type: Google.Protobuf.Int64Value
  field :image_pull_secrets, 3, type: Google.Protobuf.StringValue

  field :annotations, 4,
    repeated: true,
    type: Bossman.Protobuf.V1alpha1.Options.AnnotationsEntry,
    map: true

  field :namespace, 5, type: Google.Protobuf.StringValue
  field :retries, 6, type: Google.Protobuf.Int32Value
  field :completions, 7, type: Google.Protobuf.Int32Value
  field :parallelism, 8, type: Google.Protobuf.Int32Value
  field :args, 9, repeated: true, type: :string
  field :command, 10, repeated: true, type: :string
  field :env, 11, repeated: true, type: Bossman.Protobuf.V1alpha1.Options.Env
  field :env_from, 12, repeated: true, type: Bossman.Protobuf.V1alpha1.Options.EnvFrom
end
