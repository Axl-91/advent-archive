defmodule Day1 do
  @moduledoc """
  Documentation for `Day1`.
  """

  @doc """
  Solver for part 1 of day 1
  """
  def solve_part_1 do
    File.read!("input")
    |> String.split(~r/[\s\n]+/, trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.with_index()
    |> Enum.split_with(fn {_val, idx} -> rem(idx, 2) == 0 end)
    |> Tuple.to_list()
    |> Enum.map(fn arr -> Enum.map(arr, fn {v, _} -> v end) |> Enum.sort() end)
    |> Enum.zip()
    |> Enum.map(fn {a, b} -> abs(a - b) end)
    |> Enum.sum()
  end

  @doc """
  Solver for part 2 of day 1
  """
  def solve_part_2 do
    File.read!("input")
    |> String.split(~r/[\s\n]+/, trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.with_index()
    |> Enum.split_with(fn {_val, idx} -> rem(idx, 2) == 0 end)
    |> Tuple.to_list()
    |> Enum.map(fn arr -> Enum.map(arr, fn {v, _} -> v end) end)
    |> then(fn [left, right] ->
      [
        Enum.frequencies(left),
        Enum.frequencies(Enum.filter(right, fn v -> v in left end))
      ]
    end)
    |> then(fn [left, right] ->
      Enum.map(right, fn {val, freq} -> val * freq * Map.get(left, val) end)
    end)
    |> Enum.sum()
  end
end
