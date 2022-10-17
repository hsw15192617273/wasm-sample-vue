# frozen_string_literal: true

require_relative "num_ext/version"
require "prime"

module NumExt
  class Error < StandardError; end

  # 第n位质数
  def nth_prime(n)
    Prime.first(n).last
  end
end
