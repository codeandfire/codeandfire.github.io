"""The little addition program in Python."""

def add_two(a, b):
    return a + b

# this works
assert add_two(1, 2) == 3

class Bar:
    def __add__(self, other):
        return Bar()

a = Bar()
b = Bar()
assert add_two(a, b) == Bar()   # this works

class Foo:
    pass

a = Foo()
b = Foo()
add_two(a, b)   # this won't work
