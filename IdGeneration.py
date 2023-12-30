import sys

# Check if the number of parts is provided as a command line argument
if len(sys.argv) > 1:
    try:
        num_parts = int(sys.argv[1])
    except ValueError:
        print("Please provide a valid integer for the number of parts.")
        sys.exit(1)
else:
    print("Please provide the number of parts as a command line argument.")
    sys.exit(1)

total_range = 3800000 - 1000  # Adjust to start from 1
part_size = total_range // num_parts  # Use integer division to get a whole number

# Initialize the start and end values
start_value = 1000
end_value = start_value + part_size - 1

# List to store the ranges
ranges = []

# Generate the ranges
for _ in range(num_parts):
    range_str = f"{start_value}-{end_value}"
    ranges.append(range_str)

    # Update start and end values for the next part
    start_value = end_value
    end_value = start_value + part_size - 1

# Print the ranges, starting from 1
for i, range_str in enumerate(ranges, start=1):
    print(f"{i} | {range_str}")
