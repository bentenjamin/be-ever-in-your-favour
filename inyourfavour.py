import argparse
import polars as pl

parser = argparse.ArgumentParser(description='Odd int counts')
parser.add_argument('filename', type=str, help='File path of json list of lists of ints')
args = parser.parse_args()

data: str
with open(args.filename, 'r') as file:
    data = file.read()


result = pl.LazyFrame({"arrs": data}).select(pl.col("arrs").str.json_extract(pl.List(pl.List(pl.UInt8))).explode()).with_columns(
    id= pl.int_range(0, pl.col('arrs').count())
).explode("arrs").group_by('id').agg(pl.col('arrs').value_counts()).explode("arrs").unnest('arrs').group_by('id').agg([
    pl.col('arrs').filter(pl.col('counts').mod(2) == 1)
]).explode('arrs').collect().get_column('arrs').to_list()

print(result, sep='')