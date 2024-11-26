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
    if args[0][0] == 0:
        return 'Silver'
    if args[0][1] == 0:
        return 'Gold'
    else:
        return 'Alloy'


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = [[50, 50]]
        result = execute(args)
        self.assertEqual(result, "Alloy")

    def test_no2(self):
        args = [[100, 0]]
        result = execute(args)
        self.assertEqual(result, "Gold")

    def test_no3(self):
        args = [[0, 100]]
        result = execute(args)
        self.assertEqual(result, "Silver")

    def test_no4(self):
        args = [[100, 2]]
        result = execute(args)
        self.assertEqual(result, "Alloy")


if __name__ == '__main__':
    unittest.main()
