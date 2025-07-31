def steps(num):
    if num <= 0:
        raise ValueError("Only positive integers are allowed")
    if num == 1:
        return 0
    if num&1==0:
        return steps(int(num//2)) + 1
    else:
        return steps((num*3)+1) + 1

