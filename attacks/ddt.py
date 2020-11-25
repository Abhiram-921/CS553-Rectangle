# RECTANGLE S-box
sb = [6, 5, 0xc, 0xa, 1, 0xe, 7, 9, 0xb, 0, 3, 0xd, 8, 0xf, 4, 2]

print(" |  " + "\t".join(map(str, range(2**4))))
for i in range(2**4):
    print(i, end="|\t")
    for j in range(2**4):
        # Count number of pairs(i^x,x) for which input xor and output xor is same
        print([sb[p] ^ sb[q] for p, q in [(i ^ x, x) for x in range(2**4)]].count(j), end="\t")
    print("")