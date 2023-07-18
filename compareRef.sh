#!/bin/sh

nodes=0
compare () {
  cat $1
  ./target/release/chessengine --perft $1 > "result$2.actual"
  ./wrap_stockfish.sh $1 > "result$2.expected"
  diff "result$2.actual" "result$2.expected"
  if [ $? -ne 0 ]
  then
      cat $1 >> diff.res;
      diff "result$2.actual" "result$2.expected" >> diff.res;
  fi
  typeset -i res=$(cat "result$2.expected")
  nodes=$((nodes+res));
  rm "file$2.perft" "result$2.actual" "result$2.expected"
}

input="default.tmp"

N=100
count=0;
while IFS= read -r line
do
  ((count=count%N)); ((count++==0)) && wait
  echo "$line" > "file${count}.perft";
  compare "file${count}.perft" $count &
  echo "New process: " $count
done < "$input"

echo $nodes
