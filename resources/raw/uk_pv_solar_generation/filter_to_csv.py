import os
import pandas as pd

# ===== CONFIG =====
year_folder = "year=2013"
output_folder = "output_csv"
id_column = "ss_id"                     # Column name containing IDs
filter_ids = {2657, 2910, 3116, 3217, 3491, 4029, 5120, 5286, 5392, 5899, 8184, 8334, 8387, 8401, 8576, 8603, 8621, 8658, 9154}         # IDs you want to extract
# ===================

os.makedirs(output_folder, exist_ok=True)

# Dictionary to collect data per ID
collected_data = {i: [] for i in filter_ids}

# Loop through month folders
for month in sorted(os.listdir(year_folder)):
    month_path = os.path.join(year_folder, month)

    if not os.path.isdir(month_path):
        continue

    print(f"Processing month: {month}")

    # Loop through parquet files in month folder
    for file in os.listdir(month_path):
        if file.endswith(".parquet"):
            file_path = os.path.join(month_path, file)

            try:
                df = pd.read_parquet(file_path)

                # Filter rows by ID
                filtered = df[df[id_column].isin(filter_ids)]

                # Store data per ID
                for i in filter_ids:
                    id_rows = filtered[filtered[id_column] == i]
                    if not id_rows.empty:
                        collected_data[i].append(id_rows)

            except Exception as e:
                print(f"Error reading {file_path}: {e}")

# Write each ID's data to separate CSV
for i, dataframes in collected_data.items():
    if dataframes:
        final_df = pd.concat(dataframes, ignore_index=True)
        output_path = os.path.join(output_folder, f"id_{i}.csv")
        final_df.to_csv(output_path, index=False)
        print(f"Saved {output_path}")
    else:
        print(f"No data found for ID {i}")

print("Done.")
