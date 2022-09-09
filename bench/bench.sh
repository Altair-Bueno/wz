#!/usr/bin/env bash

# Simple bash script that runs some benchmarks using hyperfine
# USAGE
# ./bench.sh "path/to/files" "path/export/folder"

ARGS_LIST_HYPERFINE=$(ls $1 | xargs)

echo "Byte count"
hyperfine -m 100 -w 2 -n "wz" "wz -b $ARGS_LIST_HYPERFINE" "wc -c $ARGS_LIST_HYPERFINE" --export-markdown "$2/bytes.md"
echo "********************************************************************************"
echo "Word count"
hyperfine -m 100 -w 2 -n "wz" "wz -w $ARGS_LIST_HYPERFINE" "wc -w $ARGS_LIST_HYPERFINE" --export-markdown "$2/words.md"
echo "********************************************************************************"
echo "Char count"
hyperfine -m 100 -w 2 -n "wz" "wz -c $ARGS_LIST_HYPERFINE" "wc -m $ARGS_LIST_HYPERFINE" --export-markdown "$2/chars.md"
echo "********************************************************************************"
echo "Line count"
hyperfine -m 100 -w 2 -n "wz" "wz -l $ARGS_LIST_HYPERFINE" "wc -l $ARGS_LIST_HYPERFINE" --export-markdown "$2/lines.md"
echo "********************************************************************************"
# echo "max length"
# hyperfine -m 100 -w 2 -n "wz" "wz -L $ARGS_LIST_HYPERFINE" "wc -L $ARGS_LIST_HYPERFINE" --export-markdown "$2/max-length.md"
# echo "********************************************************************************"
echo "Default mode"
hyperfine -m 100 -w 2 -n "wz" "wz $ARGS_LIST_HYPERFINE" "wc $ARGS_LIST_HYPERFINE" --export-markdown "$2/default.md"
echo "********************************************************************************"
echo "High load"
hyperfine -m 100 -w 2 -n "wz" "wz $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE" "wc $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE" --export-markdown "$2/high-load.md"