# _*_ encoding=utf-8 _*_

# abstract factory
def abFactory(factory):
    return factory.createItem(5)

# factoryA
class FactoryA:

    @classmethod
    def createItem(cls, item_num):
        return cls.Item(item_num)

    # ItemA
    class Item:

        def __init__(self, item_num):
            self.num = item_num
            self.price = 100
            self.profit = 0
        
        def sell(self):
            self.num -= 1
            if self.num < 0: return
            self.profit += self.price

# factoryB
class FactoryB(FactoryA):

    # ItemB
    class Item:

        def __init__(self, item_num):
            self.num = item_num
            self.price = 50
            self.profit = -50 * item_num
        
        def sell(self):
            self.num -= 1
            if self.num < 0: return
            self.profit += self.price + 25

def main():
    itemA = abFactory(FactoryA)
    itemB = abFactory(FactoryB)

if __name__ == "__main__":
    main()