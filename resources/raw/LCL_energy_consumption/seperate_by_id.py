import os
import pandas as pd

# ===== CONFIG =====
input_csv = "output_cleaned3.csv"
output_folder = "greater_london_2013"
id_column = "LCLid"
# ===================

# Create output folder if it doesn't exist
os.makedirs(output_folder, exist_ok=True)

# Load CSV
df = pd.read_csv(input_csv)

# Group by LCLid and write each to separate file
for lclid, group in df.groupby(id_column):
    output_path = os.path.join(output_folder, f"{lclid}.csv")
    group.to_csv(output_path, index=False)
    print(f"Saved: {output_path}")

print("Done.")
