defmodule Bossman.Job.Error do
  defmodule Cancelled, do: defexception([:message])
  defmodule Unknown, do: defexception([:message])
  defmodule InvalidArgument, do: defexception([:message])
  defmodule DeadlineExceeded, do: defexception([:message])
  defmodule NotFound, do: defexception([:message])
  defmodule AlreadyExists, do: defexception([:message])

  def new(%GRPC.RPCError{message: message, status: status}) do
    status_to_error_struct(status, URI.decode(message))
  end

  defp status_to_error_struct(1, msg), do: %Cancelled{message: msg}
  defp status_to_error_struct(2, msg), do: %Unknown{message: msg}
  defp status_to_error_struct(3, msg), do: %InvalidArgument{message: msg}
  defp status_to_error_struct(4, msg), do: %DeadlineExceeded{message: msg}
  defp status_to_error_struct(5, msg), do: %NotFound{message: msg}
  defp status_to_error_struct(6, msg), do: %AlreadyExists{message: msg}
  defp status_to_error_struct(_, msg), do: %Unknown{message: msg}
end
