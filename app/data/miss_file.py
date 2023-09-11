import os

total_files = 66
missing_files = []

for i in range(1, total_files + 1):
    filename = f"rustcc_{i}.js"
    if not os.path.exists(filename):
        missing_files.append(filename)

print("Missing files:", missing_files)
