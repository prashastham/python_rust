import rust_module as rm

# main function
def main():
    res = rm.sum_as_string(5, 10)
    print(f"The sum is: {res}")

    res2 = rm.mult_as_string(5, 10)
    print(f"The product is: {res2}")

if __name__ == "__main__":
    main()