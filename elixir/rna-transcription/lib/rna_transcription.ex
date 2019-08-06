defmodule RnaTranscription do
  @doc """
  Transcribes a character list representing DNA nucleotides to RNA

  ## Examples

  iex> RnaTranscription.to_rna('ACTG')
  'UGAC'
  """
  @spec to_rna([char]) :: [char]
  def to_rna([]) do [] end
  def to_rna([head | tail]) do
    [case head do
      ?G -> ?C
      ?C -> ?G
      ?T -> ?A
      ?A -> ?U
    end | to_rna tail]
  end
end
