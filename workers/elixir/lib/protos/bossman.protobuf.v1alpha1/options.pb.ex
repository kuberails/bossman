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

defmodule Bossman.Protobuf.V1alpha1.Options.Env do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          env: {atom, any}
        }
  defstruct [:env]

  oneof :env, 0
  field :value, 2, type: Bossman.Protobuf.V1alpha1.Options.EnvValue, oneof: 0
  field :valueFrom, 3, type: Bossman.Protobuf.V1alpha1.Options.EnvFrom, oneof: 0
end

defmodule Bossman.Protobuf.V1alpha1.Options.EnvFrom do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          valueFrom: {atom, any},
          name: String.t()
        }
  defstruct [:valueFrom, :name]

  oneof :valueFrom, 0
  field :name, 1, type: :string
  field :secretKeyRef, 2, type: Bossman.Protobuf.V1alpha1.Options.SecretKeyRef, oneof: 0
  field :configMapKeyRef, 3, type: Bossman.Protobuf.V1alpha1.Options.ConfigMapKeyRef, oneof: 0
end

defmodule Bossman.Protobuf.V1alpha1.Options.EnvValue do
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

defmodule Bossman.Protobuf.V1alpha1.Options.ConfigMapKeyRef do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          key: String.t(),
          name: String.t()
        }
  defstruct [:key, :name]

  field :key, 1, type: :string
  field :name, 2, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options.SecretKeyRef do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          key: String.t(),
          name: String.t()
        }
  defstruct [:key, :name]

  field :key, 1, type: :string
  field :name, 2, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Options do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          timeout: {atom, any},
          docker_image: {atom, any},
          image_pull_secrets: {atom, any},
          namespace: {atom, any},
          backoff_limit: {atom, any},
          completions: {atom, any},
          parallelism: {atom, any},
          annotations: %{String.t() => String.t()},
          args: [String.t()],
          command: [String.t()],
          env: [Bossman.Protobuf.V1alpha1.Options.Env.t()],
          envFrom: [Bossman.Protobuf.V1alpha1.Options.EnvFrom.t()]
        }
  defstruct [
    :timeout,
    :docker_image,
    :image_pull_secrets,
    :namespace,
    :backoff_limit,
    :completions,
    :parallelism,
    :annotations,
    :args,
    :command,
    :env,
    :envFrom
  ]

  oneof :timeout, 0
  oneof :docker_image, 1
  oneof :image_pull_secrets, 2
  oneof :namespace, 3
  oneof :backoff_limit, 4
  oneof :completions, 5
  oneof :parallelism, 6
  field :_timeout, 1, type: :int64, oneof: 0
  field :_docker_image, 2, type: :string, oneof: 1
  field :_image_pull_secrets, 3, type: :string, oneof: 2

  field :annotations, 4,
    repeated: true,
    type: Bossman.Protobuf.V1alpha1.Options.AnnotationsEntry,
    map: true

  field :_namespace, 5, type: :string, oneof: 3
  field :_backoff_limit, 6, type: :int32, oneof: 4
  field :_completions, 7, type: :int32, oneof: 5
  field :_parallelism, 8, type: :int32, oneof: 6
  field :args, 9, repeated: true, type: :string
  field :command, 10, repeated: true, type: :string
  field :env, 11, repeated: true, type: Bossman.Protobuf.V1alpha1.Options.Env
  field :envFrom, 12, repeated: true, type: Bossman.Protobuf.V1alpha1.Options.EnvFrom
end
