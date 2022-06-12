# _*_ encoding=utf-8 _*_

# abstract factory
def abFactory(factory):
    item1 = factory.createItem("sushi",100)
    item2 = factory.createItem("ra-men",50)
    tenant = factory.Tenant("sugoi mise") 
    tenant.setItems(item1)
    tenant.setItems(item2)
    return tenant

# factoryA
class FactoryA:

    @classmethod
    def createItem(cls, name, price):
        return cls.Item(name, price)

    @classmethod
    def createTenant(cls, tenantName):
        return cls.tenant(tenantName)

    # ItemA
    class Item:

        def __init__(self, name, price):
            self.itemName = name
            self.price = price

        def getItemName(self):
            return self.itemName
        
        def getPrice(self):
            return self.price 
    
    class Tenant:

        def __init__(self, tenant_name):
            self.tenantName = tenant_name
            self.items = {}
            self.profit = 0
        
        def setItems(self, item):
            self.items[item.getItemName()] = item.getPrice() 

        def sell(self, itemName, num):
            value = self.items[itemName]
            self.profit += value * num

# factoryB
class FactoryB(FactoryA):

    # ItemB (tokubai)
    class Item:

        def __init__(self, name, price):
            self.itemName = name
            self.price = int(price * 0.8)

        def getItemName(self):
            return self.itemName
        
        def getPrice(self):
            return self.price 

def main():
    tenantA = abFactory(FactoryA)
    tenantB_tokubai = abFactory(FactoryB)

if __name__ == "__main__":
    main()