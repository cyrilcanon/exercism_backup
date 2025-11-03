defmodule Secrets do
  def secret_add(secret) do
    &(&1 + secret)
  end

  def secret_subtract(secret) do
    substrct_fct = fn param ->
      param - secret
    end
    substrct_fct
  end

  def secret_multiply(secret) do
    &(&1 * secret)
  end

  def secret_divide(secret) do
    &(div(&1, secret))
  end

  def secret_and(secret) do
    and_fct = fn param ->
      Bitwise.band(param, secret)
    end
    and_fct
  end

  def secret_xor(secret) do
    xor_fct = fn param ->
      Bitwise.bxor(param, secret)
    end
    xor_fct
  end

  def secret_combine(secret_function1, secret_function2) do
    combine_fct = fn param ->
      a = secret_function1.(param)
      secret_function2.(a)
    end
    combine_fct
  end
end
