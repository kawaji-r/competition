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
    arr = args[1]
    cnt = 0
    while all(i % 2 == 0 for i in arr):
        arr = [i / 2 for i in arr]
        cnt += 1
    return str(cnt)


def please_unwrap(self):
    start()

# please_unwrapを外してここから上を提出する


class TestMyModule(unittest.TestCase):
    def test_no1(self):
        args = [3, [8, 12, 40]]
        result = execute(args)
        self.assertEqual(result, "2")

    def test_no2(self):
        args = [4, [5, 6, 8, 10]]
        result = execute(args)
        self.assertEqual(result, "0")

    def test_no3(self):
        args = [6, [382253568, 723152896, 37802240, 379425024, 404894720, 471526144]]
        result = execute(args)
        self.assertEqual(result, "8")


if __name__ == '__main__':
    unittest.main()
