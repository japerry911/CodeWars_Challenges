defmodule StrongestEvenNumberTest do
  use ExUnit.Case
  doctest StrongestEvenNumber

  test "sample test 1" do
    assert StrongestEvenNumber.strongest_even(1, 2) === 2
  end

  test "sample test 2" do
    assert StrongestEvenNumber.strongest_even(5, 10) === 8
  end

  test "sample test 3" do
    assert StrongestEvenNumber.strongest_even(48, 56) === 48
  end

  test "sample test 4" do
    assert StrongestEvenNumber.strongest_even(129, 193) === 192
  end
end
