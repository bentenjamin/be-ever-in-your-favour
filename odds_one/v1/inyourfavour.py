import sys
import polars as pl

with open(sys.argv[1]) as f:
   print(
      pl.LazyFrame(["arrs", pl.List(pl.Utf8)])
      .select(arrs = pl.lit(f.read()).str.json_extract(pl.List(pl.List(pl.UInt8))).explode())
      .select(pl.col("arrs").list.eval(pl.element().value_counts()))
      .explode("arrs")
      .filter(pl.col("arrs").struct["counts"].mod(2) == 1)
      .select(arrs = pl.col("arrs").struct[""])
      .collect()
      .get_column("arrs")
      .to_list()
   , sep='')