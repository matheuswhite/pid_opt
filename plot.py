import os
import sys
import matplotlib.pyplot as plt
import pandas as pd

directory = sys.argv[1]

# Process and plot all CSV files
for root, dirs, files in os.walk(f"{directory}"):
    for file in files:
        if not file.endswith(".csv"):
            continue

        file_path = os.path.join(root, file)
        print(f"Processing: {file_path}")

        df = pd.read_csv(file_path)

        plt.figure(figsize=(10, 6))
        plt.plot(df["t"], df["input"], label="Referência", linewidth=2)
        plt.plot(df["t"], df["output"], label="Saída", linewidth=2)
        plt.title(f"{file}")
        plt.xlabel("Tempo (s)")
        plt.ylabel("Amplitude")
        plt.legend()
        plt.grid(True, alpha=0.3)

        # Save with better filename
        output_filename = f"{directory}/plot_{file.replace('.csv', '')}.png"
        plt.savefig(output_filename, dpi=300, bbox_inches="tight")
        plt.close()

        print(f"Saved plot: {output_filename}")
