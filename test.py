import sys
import re

input_data = sys.stdin.read()

regex = re.compile(pattern="-\\d+")

print(regex.search(input_data))
