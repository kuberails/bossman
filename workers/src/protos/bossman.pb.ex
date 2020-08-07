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
          backoff_limit: {atom, any},
          timeout: integer,
          docker_image: String.t(),
          image_pull_secret: String.t(),
          annotations: %{String.t() => String.t()},
          namespace: String.t(),
          completions: Google.Protobuf.Int32Value.t() | nil
        }
  defstruct [
    :backoff_limit,
    :timeout,
    :docker_image,
    :image_pull_secret,
    :annotations,
    :namespace,
    :completions
  ]

  oneof :backoff_limit, 0
  field :timeout, 1, type: :int64
  field :docker_image, 2, type: :string
  field :image_pull_secret, 3, type: :string
  field :annotations, 4, repeated: true, type: Bossman.Options.AnnotationsEntry, map: true
  field :namespace, 5, type: :string
  field :int32, 6, type: :int32, oneof: 0
  field :completions, 7, type: Google.Protobuf.Int32Value
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
          name: String.t(),
          options: Bossman.Options.t() | nil
        }
  defstruct [:name, :options]

  field :name, 1, type: :string
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
