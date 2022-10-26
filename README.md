# ya-multi-payments

Polygon contract:
0x21cCe3a0F851394fcDD27b28c65232be98fc6Ce2

Rinkeby contract:
0x1939850BD3448706a0fb46F781950C510EC6A966

Goerli contract:
0xCaD0EB5813d220820B2b7Fb54Ad314700881125C

Mumbai contract:
0x32d22cb5303a18a6f613ed77307e791273d8a472

# Polygon gas costs:

Base cost: 41000
Each ERC20 transfer: 14300
Each ERC20 transfer to zero balance account: 31400 (+17100)

For comparison
Standard ERC20 transfer: 41000 (+17100 when on zero balance)

For 1 nonzero target address extra gas cost is 35%
For 2 nonzero target addresses cost reduction is 15%
For 7 nonzero target addresses gas price reduction is 50%
For 20 nonzero target addresses gas price reduction is 60%



