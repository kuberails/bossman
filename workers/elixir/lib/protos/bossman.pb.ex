defmodule Bossman.Protobuf.V1alpha1.Status do
  @moduledoc false
  use Protobuf, enum: true, syntax: :proto3

  @type t :: integer | :WAITING | :PROCESSING | :COMPLETE | :ERROR

  field :WAITING, 0
  field :PROCESSING, 1
  field :COMPLETE, 2
  field :ERROR, 3
end

defmodule Bossman.Protobuf.V1alpha1.Job do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t(),
          id: String.t(),
          options: Bossman.Protobuf.V1alpha1.Options.t() | nil,
          status: Bossman.Protobuf.V1alpha1.Status.t()
        }
  defstruct [:name, :id, :options, :status]

  field :name, 1, type: :string
  field :id, 2, type: :string
  field :options, 3, type: Bossman.Protobuf.V1alpha1.Options
  field :status, 4, type: Bossman.Protobuf.V1alpha1.Status, enum: true
end

defmodule Bossman.Protobuf.V1alpha1.PerformRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: Google.Protobuf.StringValue.t() | nil,
          options: Bossman.Protobuf.V1alpha1.Options.t() | nil
        }
  defstruct [:name, :options]

  field :name, 1, type: Google.Protobuf.StringValue
  field :options, 2, type: Bossman.Protobuf.V1alpha1.Options
end

defmodule Bossman.Protobuf.V1alpha1.PerformResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          id: String.t(),
          status: Bossman.Protobuf.V1alpha1.Status.t()
        }
  defstruct [:id, :status]

  field :id, 1, type: :string
  field :status, 2, type: Bossman.Protobuf.V1alpha1.Status, enum: true
end

defmodule Bossman.Protobuf.V1alpha1.GetStatusRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.GetRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          id: String.t()
        }
  defstruct [:id]

  field :id, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.GetResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job: Bossman.Protobuf.V1alpha1.Job.t() | nil
        }
  defstruct [:job]

  field :job, 1, type: Bossman.Protobuf.V1alpha1.Job
end

defmodule Bossman.Protobuf.V1alpha1.GetStatusResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job_id: String.t()
        }
  defstruct [:job_id]

  field :job_id, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.GetListRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.GetListResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          jobs: [Bossman.Protobuf.V1alpha1.Job.t()]
        }
  defstruct [:jobs]

  field :jobs, 1, repeated: true, type: Bossman.Protobuf.V1alpha1.Job
end
