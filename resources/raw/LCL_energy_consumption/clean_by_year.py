import pandas as pd

# ===== CONFIG =====
input_csv = "LCL-June2015v2_3.csv"
output_csv = "output_cleaned3.csv"
datetime_column = "DateTime"
kwh_column = "KWH/hh (per half hour) "
# ===================

# Load CSV
df = pd.read_csv(input_csv)

# Ensure DateTime column is proper datetime
df[datetime_column] = pd.to_datetime(df[datetime_column], errors="coerce")

# Remove rows where year == 2013
df = df[df[datetime_column].dt.year == 2013]

# Convert KWH column to numeric FIRST
df[kwh_column] = pd.to_numeric(
    df[kwh_column], 
    errors="coerce"
)

# Convert KWH/hh to Wh
# 1 kWh = 1000 Wh
df[kwh_column] = df[kwh_column] * 1000

# Optional: rename column to reflect new unit
df.rename(columns={kwh_column: "consumption_Wh"}, inplace=True)

# Save cleaned file
df.to_csv(output_csv, index=False)

print("Finished. Cleaned file saved as:", output_csv)
