defmodule Bossman.Job.Status do
  alias Bossman.Protobuf.V1alpha1, as: BossmanProtobuf
  use TypedStruct

  defmodule Error do
    defexception [:message]
  end

  @type t :: :waiting | Active.t() | Completed.t() | Failed.t()

  defmodule Active do
    typedstruct do
      field :started_at, String.t()
    end
  end

  defmodule Completed do
    typedstruct do
      field :started_at, String.t()
      field :completed_at, String.t()
    end
  end

  defmodule Failed do
    typedstruct do
      field :started_at, String.t()
      field :failed_at, String.t()
    end
  end

  @spec decode(BossmanProtobuf.Status.t()) :: {:ok, Status.t()} | {:error, String.t()}
  def decode(status) do
    try do
      {:ok, decode!(status)}
    catch
      error ->
        {:error, %Error{message: "Unable to decode status: #{inspect(error)}"}}
    end
  end

  @spec decode!(BossmanProtobuf.Job.Status.t()) :: Status.t()
  def decode!(%_{status: {:waiting, _waiting}}), do: :waiting
  def decode!(%_{status: {:active, active}}), do: %Active{started_at: active.started_at}

  def decode!(%_{status: {:completed, completed}}),
    do: %Completed{started_at: completed.started_at, completed_at: completed.completed_at}

  def decode!(%_{status: {:failed, failed}}),
    do: %Failed{started_at: failed.started_at, failed_at: failed.failed_at}
end
