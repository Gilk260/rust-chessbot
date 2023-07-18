#!/bin/sh

if [ $# -ne 1 ]; then
    echo "Usage: $0 <perft_file>"
    exit 1
fi

while IFS= read -r line
do
  # Extract the depth from the last field
  depth=$(echo "$line" | awk '{print $NF}')

  # Remove the last field from the input string
  fen="${line% $depth}" 

  echo "position fen $fen
  go perft $depth" | ./stockfish | grep Nodes | cut -d ' ' -f 3
done < "$1"
