defmodule Bossman.Job.Error do
  defmodule NotFound, do: defexception([:message])
  defmodule Unknown, do: defexception([:message])
  defmodule InvalidArgument, do: defexception([:message])

  def new(%GRPC.RPCError{message: message, status: status}) do
    status_to_error_struct(status, URI.decode(message))
  end

  defp status_to_error_struct(2, msg), do: %Unknown{message: msg}
  defp status_to_error_struct(3, msg), do: %InvalidArgument{message: msg}
  defp status_to_error_struct(5, msg), do: %NotFound{message: msg}
end
