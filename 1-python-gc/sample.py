class Data:
    def __init__(self, size):
        self.buffer = bytearray(size)
        print(f"Data allocated: {size} bytes")

    def __del__(self):
        print("Data released (via GC)")

def create_data():
    d = Data(10 * 1024 * 1024)  # 10 MB
    print("Exiting create_data")

create_data()
print("Back in main")