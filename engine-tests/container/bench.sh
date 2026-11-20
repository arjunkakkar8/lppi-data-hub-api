#!/usr/bin/env bash
# Usage: ./bench.sh https://<preview-url>   (uses curl; prints JSON lines)
set -u
U="$1"; OUT="results-$(date +%H%M%S).jsonl"
names=("enum, 1 group, national" "enum, 3 groups, filtered" "enum, 3 groups, CA+TX" "enum, by state, age range" "enum, county filter, 2 groups" "enum, many filters" "mean, national" "median age, national" "median hhincome, CA, 2 groups" "median wage, by state" "latino_origin_detailed, by state")
hit() { curl -s -o >(cat >> "$OUT"; echo >> "$OUT") -w "%{time_total}\n" -G "$U/api/query" --data-urlencode "q=$1" ${BYPASS:+-H "x-vercel-protection-bypass: $BYPASS"}; }
echo "first (cold) request:"; hit "${names[0]}"
sleep 2
for round in 1 2 3; do for n in "${names[@]}"; do t=$(hit "$n"); echo "$round | $n | client ${t}s"; done; done
echo "results in $OUT"
