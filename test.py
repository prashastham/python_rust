import rust_module

# main function
def main():
    res = rust_module.sum_as_string(5, 10)
    print(f"The sum is: {res}")

if __name__ == "__main__":
    main()