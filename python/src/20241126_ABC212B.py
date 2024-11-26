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
    equal_flg = all(args[0][i] == args[0][i+1] for i in range(len(args[0])) if i <= 2)
    next_flg = all((int(args[0][i]) + 1) % 10 == int(args[0][i+1]) for i in range(len(args[0])) if i <= 2)
    return "Weak" if equal_flg or next_flg else "Strong"


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = ['7777']
        result = execute(args)
        self.assertEqual(result, "Weak")

    def test_no2(self):
        args = ['0112']
        result = execute(args)
        self.assertEqual(result, "Strong")

    def test_no3(self):
        args = ['9012']
        result = execute(args)
        self.assertEqual(result, "Weak")


if __name__ == '__main__':
    unittest.main()
