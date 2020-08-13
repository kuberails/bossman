defmodule Bossman.Util do
  defmodule Result do
    def filter_map(list) when is_list(list) do
      list
      |> Enum.filter(fn
        {:ok, _job} -> true
        {:error, _} -> false
      end)
      |> Enum.map(fn {:ok, item} -> item end)
    end
  end
end
