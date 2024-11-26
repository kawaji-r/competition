import unittest  # 含めない


def start():
    # modify
    args_type = [
        int,
        list[int],
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
    return f"{args[0] + args[1][0] + args[1][1]} {args[2]}"


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = [1, [2, 3], "test"]
        result = execute(args)
        self.assertEqual(result, "6 test")


if __name__ == '__main__':
    unittest.main()
