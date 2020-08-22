defmodule Bossman.Protobuf.V1alpha1.Job.Status.Waiting do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{}
  defstruct []
end

defmodule Bossman.Protobuf.V1alpha1.Job.Status.Active do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          started_at: String.t()
        }
  defstruct [:started_at]

  field :started_at, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Job.Status.Completed do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          started_at: String.t(),
          completed_at: String.t()
        }
  defstruct [:started_at, :completed_at]

  field :started_at, 1, type: :string
  field :completed_at, 2, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Job.Status.Failed do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          started_at: String.t(),
          failed_at: String.t()
        }
  defstruct [:started_at, :failed_at]

  field :started_at, 1, type: :string
  field :failed_at, 2, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Job.Status do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          status: {atom, any}
        }
  defstruct [:status]

  oneof :status, 0
  field :waiting, 1, type: Bossman.Protobuf.V1alpha1.Job.Status.Waiting, oneof: 0
  field :active, 2, type: Bossman.Protobuf.V1alpha1.Job.Status.Active, oneof: 0
  field :completed, 3, type: Bossman.Protobuf.V1alpha1.Job.Status.Completed, oneof: 0
  field :failed, 4, type: Bossman.Protobuf.V1alpha1.Job.Status.Failed, oneof: 0
end

defmodule Bossman.Protobuf.V1alpha1.Job.PerformRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: Google.Protobuf.StringValue.t() | nil,
          docker_image_name: Google.Protobuf.StringValue.t() | nil,
          options: Bossman.Protobuf.V1alpha1.Options.t() | nil
        }
  defstruct [:name, :docker_image_name, :options]

  field :name, 1, type: Google.Protobuf.StringValue
  field :docker_image_name, 2, type: Google.Protobuf.StringValue
  field :options, 3, type: Bossman.Protobuf.V1alpha1.Options
end

defmodule Bossman.Protobuf.V1alpha1.Job.PerformResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job: Bossman.Protobuf.V1alpha1.Job.t() | nil
        }
  defstruct [:job]

  field :job, 1, type: Bossman.Protobuf.V1alpha1.Job
end

defmodule Bossman.Protobuf.V1alpha1.Job.GetRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          id: String.t()
        }
  defstruct [:id]

  field :id, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Job.GetResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job: Bossman.Protobuf.V1alpha1.Job.t() | nil
        }
  defstruct [:job]

  field :job, 1, type: Bossman.Protobuf.V1alpha1.Job
end

defmodule Bossman.Protobuf.V1alpha1.Job.GetStatusResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          status: Bossman.Protobuf.V1alpha1.Job.Status.t() | nil
        }
  defstruct [:status]

  field :status, 1, type: Bossman.Protobuf.V1alpha1.Job.Status
end

defmodule Bossman.Protobuf.V1alpha1.Job.GetListRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          name: String.t()
        }
  defstruct [:name]

  field :name, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Job.GetListResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          jobs: [Bossman.Protobuf.V1alpha1.Job.t()]
        }
  defstruct [:jobs]

  field :jobs, 1, repeated: true, type: Bossman.Protobuf.V1alpha1.Job
end

defmodule Bossman.Protobuf.V1alpha1.Job do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          id: String.t(),
          name: String.t(),
          docker_image_name: String.t(),
          options: Bossman.Protobuf.V1alpha1.Options.t() | nil,
          status: Bossman.Protobuf.V1alpha1.Job.Status.t() | nil
        }
  defstruct [:id, :name, :docker_image_name, :options, :status]

  field :id, 1, type: :string
  field :name, 2, type: :string
  field :docker_image_name, 3, type: :string
  field :options, 4, type: Bossman.Protobuf.V1alpha1.Options
  field :status, 5, type: Bossman.Protobuf.V1alpha1.Job.Status
end

defmodule Bossman.Protobuf.V1alpha1.JobService.Service do
  @moduledoc false
  use GRPC.Service, name: "bossman.protobuf.v1alpha1.JobService"

  rpc :Perform,
      Bossman.Protobuf.V1alpha1.Job.PerformRequest,
      Bossman.Protobuf.V1alpha1.Job.PerformResponse

  rpc :Get, Bossman.Protobuf.V1alpha1.Job.GetRequest, Bossman.Protobuf.V1alpha1.Job.GetResponse

  rpc :GetStatus,
      Bossman.Protobuf.V1alpha1.Job.GetRequest,
      Bossman.Protobuf.V1alpha1.Job.GetStatusResponse

  rpc :GetList,
      Bossman.Protobuf.V1alpha1.Job.GetListRequest,
      Bossman.Protobuf.V1alpha1.Job.GetListResponse
end

defmodule Bossman.Protobuf.V1alpha1.JobService.Stub do
  @moduledoc false
  use GRPC.Stub, service: Bossman.Protobuf.V1alpha1.JobService.Service
end
