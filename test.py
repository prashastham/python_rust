import rust_module as rm

# main function
def main():
    res = rm.sum_as_string(5, 10)
    print(f"The sum is: {res}")

    res2 = rm.mult_as_string(5, 10)
    print(f"The product is: {res2}")

    res3 = rm.sub_as_string(10, 5)
    print(f"The difference is: {res3}")

if __name__ == "__main__":
    main()