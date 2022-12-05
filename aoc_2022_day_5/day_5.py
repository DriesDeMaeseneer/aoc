pre_stacks = []

file_content = ""

with open("day_5.input", "r") as f:
    file_content = f.read()
    f.close()

file_content = file_content.split('\n')
while file_content[0] != "":
    line = file_content[0]
    better_line = []

    

    for i in range(0, len(line)):
        if i % 4 == 1:
            better_line.append(line[i])

    pre_stacks.append(better_line)
    file_content.pop(0)
    
stacks = []
for i in range(0, len(pre_stacks[0])):
    stacks.append([])
pre_stacks.pop()

for stackitem in pre_stacks:
    for i in range(0, len(stackitem)):
        if stackitem[i]!=" ":
            stacks[i].insert(0, stackitem[i])
rev_stacks = []
print(stacks)

file_content.pop(0)

for ugly_line in file_content:
    if ugly_line != "":
        line_split = ugly_line.split(' ')
        line = []
        line.append(int(line_split[1]))
        line.append(int(line_split[3]) - 1)
        line.append(int(line_split[5]) - 1)
        
        item_to_move = stacks[line[1]][-line[0]:]
        print(item_to_move)
        for item in item_to_move:
            
            stacks[line[2]].append(item)

        for i in range(0, line[0]):
            item_to_move = stacks[line[1]].pop()
        print(line, stacks)

to_return = ""
for stack in stacks:
    if len(stack)!=0:
        to_return += str(stack.pop())

print(to_return)

