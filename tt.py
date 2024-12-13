from datetime import datetime

start = datetime.now()
result = 24 + 4
print(result)
result = result / 3
end = datetime.now()
print((end - start).microseconds)
print((datetime.now() - end).microseconds)
