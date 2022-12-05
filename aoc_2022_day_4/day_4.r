input = read.csv("day_4.input", header=FALSE)
#forward
f_start_ok = input[3]>=input[1]
f_end_ok = input[4]<=input[2]
f_ok = f_start_ok & f_end_ok
# backward
b_start_ok = input[3]<=input[1]
b_end_ok = input[4]>=input[2]
b_ok = b_start_ok & b_end_ok

result = f_ok | b_ok
print(sum(result))


cat("Amount of overlaps: ")
ok = input[1]<=input[4] & input[2]>=input[3]
print(sum(ok))

