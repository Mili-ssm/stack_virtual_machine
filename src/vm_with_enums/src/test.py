import time


def iterador():
    INCREMENT = 1.000001
    MAX = 1_000_000_000_000.0
    start_time = time.time()

    counter = 1.0
    while counter < MAX:
        counter *= INCREMENT

    print("Counter:", counter)
    print("Execution time:", time.time() - start_time)


if __name__ == "__main__":
    iterador()
