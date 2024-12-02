import unittest  # 含めない


def start():
    # modify
    args_type = [
        int,
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
    worst = [0, 0]
    boobie = [0, 0]
    for index, val in enumerate(args[1]):
        if val > worst[1]:
            boobie = worst.copy()
            worst = [index, val]
        elif val > boobie[1]:
            boobie = [index, val]

    return str(boobie[0] + 1)


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = [6, [1, 123, 12345, 12, 1234, 123456]]
        result = execute(args)
        self.assertEqual(result, "3")

    def test_no2(self):
        args = [5, [3, 1, 4, 15, 9]]
        result = execute(args)
        self.assertEqual(result, "5")


if __name__ == '__main__':
    unittest.main()
