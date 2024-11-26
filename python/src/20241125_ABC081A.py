import unittest  # 含めない


def start():
    # modify
    args_type = [
        str,
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
    return str(args[0].count('1'))


def execute2(args: list) -> str:
    # modify
    num: str = args[0]
    before = len(num)
    replace = num.replace('1', '')
    after = len(replace)
    return str(before - after)


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = ['101']
        result = execute(args)
        self.assertEqual(result, "2")

    def test_no2(self):
        args = ['000']
        result = execute(args)
        self.assertEqual(result, "0")


if __name__ == '__main__':
    unittest.main()
