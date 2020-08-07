defmodule Bossman.Status do
  @moduledoc false
  use Protobuf, enum: true, syntax: :proto3

  @type t :: integer | :WAITING | :PROCESSING | :COMPLETE | :ERROR

  field :WAITING, 0
  field :PROCESSING, 1
  field :COMPLETE, 2
  field :ERROR, 3
end

defmodule Bossman.Options.AnnotationsEntry do
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

defmodule Bossman.Options do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          timeout: {atom, any},
          docker_image: {atom, any},
          image_pull_secrets: {atom, any},
          namespace: {atom, any},
          backoff_limit: {atom, any},
          completions: {atom, any},
          annotations: %{String.t() => String.t()}
        }
  defstruct [
    :timeout,
    :docker_image,
    :image_pull_secrets,
    :namespace,
    :backoff_limit,
    :completions,
    :annotations
  ]

  oneof :timeout, 0
  oneof :docker_image, 1
  oneof :image_pull_secrets, 2
  oneof :namespace, 3
  oneof :backoff_limit, 4
  oneof :completions, 5
  field :_timeout, 1, type: :int64, oneof: 0
  field :_docker_image, 2, type: :string, oneof: 1
  field :_image_pull_secrets, 3, type: :string, oneof: 2
  field :_namespace, 5, type: :string, oneof: 3
  field :_backoff_limit, 6, type: :int32, oneof: 4
  field :_completions, 7, type: :int32, oneof: 5
  field :annotations, 4, repeated: true, type: Bossman.Options.AnnotationsEntry, map: true
end

defmodule Bossman.Job do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t(),
          id: String.t(),
          options: Bossman.Options.t() | nil,
          status: Bossman.Status.t()
        }
  defstruct [:name, :id, :options, :status]

  field :name, 1, type: :string
  field :id, 2, type: :string
  field :options, 3, type: Bossman.Options
  field :status, 4, type: Bossman.Status, enum: true
end

defmodule Bossman.PerformRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: {atom, any},
          options: Bossman.Options.t() | nil
        }
  defstruct [:name, :options]

  oneof :name, 0
  field :_name, 1, type: :string, oneof: 0
  field :options, 2, type: Bossman.Options
end

defmodule Bossman.PerformResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          id: String.t(),
          status: Bossman.Status.t()
        }
  defstruct [:id, :status]

  field :id, 1, type: :string
  field :status, 2, type: Bossman.Status, enum: true
end

defmodule Bossman.GetStatusRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.GetRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          id: String.t()
        }
  defstruct [:id]

  field :id, 1, type: :string
end

defmodule Bossman.GetResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job: Bossman.Job.t() | nil
        }
  defstruct [:job]

  field :job, 1, type: Bossman.Job
end

defmodule Bossman.GetStatusResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job_id: String.t()
        }
  defstruct [:job_id]

  field :job_id, 1, type: :string
end

defmodule Bossman.GetListRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.GetListResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          jobs: [Bossman.Job.t()]
        }
  defstruct [:jobs]

  field :jobs, 1, repeated: true, type: Bossman.Job
end
