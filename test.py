import time
import numpy as np
import rust_module as rm

# main function
def main():
    res = rm.sum_as_string(5, 10)
    print(f"The sum is: {res}")

    res2 = rm.mult_as_string(5, 10)
    print(f"The product is: {res2}")

    res3 = rm.sub_as_string(10, 5)
    print(f"The difference is: {res3}")

    # Test sum_array_fast and calculate the time taken for a large array.
    arr = np.arange(1, 1_000_001, dtype=np.int64) # Array of 1 million numbers.
    start_time = time.time()
    res4 = rm.sum_array_fast(arr)
    end_time = time.time()
    print(f"The sum of the array is: {res4}")
    print(f"Time taken to sum the array: {end_time - start_time:.4f} seconds")

    # Test with python's built-in sum for comparison.
    start_time = time.time()
    res5 = sum(arr)
    end_time = time.time()
    print(f"The sum of the array using Python's built-in sum is: {res5}")
    print(f"Time taken to sum the array using Python's built-in sum: {end_time - start_time:.4f} seconds")

if __name__ == "__main__":
    main()