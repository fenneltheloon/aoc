from hashlib import md5

key = "bgvyzdsv"
i = 1

while True:
    print(i)
    input = key + str(i)
    h = md5(input.encode("utf-8")).hexdigest()
    if h[:6] == "000000":
        break
    i += 1
