import pandas as pd

rows_to_show = 1000

pd.set_option('display.max_rows', rows_to_show)  # Show up to 100 rows
df = pd.read_parquet('/home/samuelhill/uk_pv/30_minutely/year=2013/month=02/data.parquet')

print(df.head(rows_to_show))
