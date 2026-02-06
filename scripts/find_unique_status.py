import json
import sys

def extract_status_values(obj):
    statuses = []
    if isinstance(obj, dict):
        for key, value in obj.items():
            if key == "status":
                statuses.append(value)
            else:
                statuses.extend(extract_status_values(value))
    elif isinstance(obj, list):
        for item in obj:
            statuses.extend(extract_status_values(item))
    return statuses

def main():
    if len(sys.argv) != 2:
        print("Usage: python extract_status.py <json_file>")
        sys.exit(1)

    json_file = sys.argv[1]

    with open(json_file, "r") as f:
        data = json.load(f)

    unique_statuses = set(extract_status_values(data))

    for status in sorted(unique_statuses):
        print(status)

if __name__ == "__main__":
    main()
