defmodule Bossman.Protobuf.V1alpha1.Job.Status do
  @moduledoc false
  use Protobuf, enum: true, syntax: :proto3

  @type t :: integer | :WAITING | :PROCESSING | :COMPLETE | :ERROR

  field :WAITING, 0
  field :PROCESSING, 1
  field :COMPLETE, 2
  field :ERROR, 3
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

defmodule Bossman.Protobuf.V1alpha1.Job.GetStatusRequest do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          job_id: String.t()
        }
  defstruct [:job_id]

  field :job_id, 1, type: :string
end

defmodule Bossman.Protobuf.V1alpha1.Job.GetStatusResponse do
  @moduledoc false
  use Protobuf, syntax: :proto3

  @type t :: %__MODULE__{
          status: Bossman.Protobuf.V1alpha1.Job.Status.t()
        }
  defstruct [:status]

  field :status, 1, type: Bossman.Protobuf.V1alpha1.Job.Status, enum: true
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
          status: Bossman.Protobuf.V1alpha1.Job.Status.t()
        }
  defstruct [:id, :name, :docker_image_name, :options, :status]

  field :id, 1, type: :string
  field :name, 2, type: :string
  field :docker_image_name, 3, type: :string
  field :options, 4, type: Bossman.Protobuf.V1alpha1.Options
  field :status, 5, type: Bossman.Protobuf.V1alpha1.Job.Status, enum: true
end

defmodule Bossman.Protobuf.V1alpha1.JobService.Service do
  @moduledoc false
  use GRPC.Service, name: "bossman.protobuf.v1alpha1.JobService"

  rpc :Perform,
      Bossman.Protobuf.V1alpha1.Job.PerformRequest,
      Bossman.Protobuf.V1alpha1.Job.PerformResponse

  rpc :Get, Bossman.Protobuf.V1alpha1.Job.GetRequest, Bossman.Protobuf.V1alpha1.Job.GetResponse
end

defmodule Bossman.Protobuf.V1alpha1.JobService.Stub do
  @moduledoc false
  use GRPC.Stub, service: Bossman.Protobuf.V1alpha1.JobService.Service
end
