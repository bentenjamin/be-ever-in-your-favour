import sys
import polars as pl

def write_out(var: str):
    sys.stdout.write(var)
    sys.stdout.write("\n")
    sys.stdout.flush()

for line in sys.stdin:
    if line == "q\n": break
    if line == "\n":
        write_out('0')
        continue
    input_ints = line.rstrip().split(',')
    write_out(str(
        pl.LazyFrame(["arrs", pl.List(pl.Utf8)]).select(arrs = input_ints)
        .with_row_count("id").explode("arrs")
        .group_by("id", "arrs", maintain_order=True).count()
        .filter(pl.col("count").mod(2) == 1)
        .collect().item(0,1)
    ))