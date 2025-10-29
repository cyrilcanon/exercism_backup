defmodule Lasagna do

  def expected_minutes_in_oven() do
    40
  end

  def remaining_minutes_in_oven(time_already_spent) do
    expected_minutes_in_oven() - time_already_spent
  end

  def preparation_time_in_minutes(nb_layer) do
    nb_layer * 2
  end

  def total_time_in_minutes(nb_layer, time_already_spent) do
    preparation_time_in_minutes(nb_layer) + time_already_spent
  end

  def alarm() do
    "Ding!"
  end
end
