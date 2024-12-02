import unittest  # 含めない


def start():
    # modify
    args_type = [
        list[int],
    ]
    args = []
    for i in args_type:
        if i == int:
            args.append(int(input()))
        elif i == list[int]:
            args.append(list(map(int, input().split())))
        elif i == str:
            args.append(input())
    result = execute(args)
    print(result)


def execute(args: list) -> str:
    # modify
    result = args[0][0] ^ args[0][1]
    return str(result)


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = [[3, 6]]
        result = execute(args)
        self.assertEqual(result, "5")

    def test_no2(self):
        args = [[10, 12]]
        result = execute(args)
        self.assertEqual(result, "6")


if __name__ == '__main__':
    unittest.main()
