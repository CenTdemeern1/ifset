# Proves ifset is turing complete by demonstrating that any brainfuck program can be translated into ifset
# (Assuming an infinite file size and infinite memory to mirror brainfuck's infinite tape)

# TODO add input in generate cell after the decrement function
# TODO add truthy variable so loops can be handled -- done?
# TODO (finnaly) add smaller functions to actually compile the brainfuck
# Voila it's done

TAPE_LENGTH = 11
CELL_LIMIT = 256

def indent(i):
	return "\t"*i
ind = indent #shorthand

def sanitize(x):
	if x == "\\":
		return r"\\"
	if x == "\n":
		return r"\n"
	if x == "\r":
		return r"\r"
	return x

def generate_cell(cellnum : int, indentation : int = 0):
	s = f"""{ind(indentation)}IF cell == {cellnum}
{ind(indentation+1)}truthy = 1
{ind(indentation+1)}IF cell{cellnum} == 0
{ind(indentation+2)}truthy = 0
{ind(indentation+1)}DEF increment"""
	for i in range(CELL_LIMIT):
		v = (i+1)%CELL_LIMIT
		s+=f"""
{ind(indentation + 2)}IF cell{cellnum} == {i}
{ind(indentation + 3)}cell{cellnum} = {v}
{ind(indentation + 3)}truthy = {int(bool(v))}
{ind(indentation + 3)}DEF output
{ind(indentation + 4)}OUTPUT = {sanitize(chr(v))}
{ind(indentation + 4)}RETURN
{ind(indentation + 3)}RETURN"""

	s += f"""
{ind(indentation+1)}DEF decrement"""
	for i in range(CELL_LIMIT):
		v = ((CELL_LIMIT-2)-i)%CELL_LIMIT
		s+=f"""
{ind(indentation + 2)}IF cell{cellnum} == {(CELL_LIMIT-1)-i}
{ind(indentation + 3)}cell{cellnum} = {v}
{ind(indentation + 3)}truthy = {int(bool(v))}
{ind(indentation + 3)}DEF output
{ind(indentation + 4)}OUTPUT = {sanitize(chr(v))}
{ind(indentation + 4)}RETURN
{ind(indentation + 3)}RETURN"""

	s += f"""
{ind(indentation+1)}DEF defoutput"""
	for i in range(CELL_LIMIT):
		s+=f"""
{ind(indentation + 2)}IF cell{cellnum} == {i}
{ind(indentation + 3)}DEF output
{ind(indentation + 4)}OUTPUT = {sanitize(chr(i))}
{ind(indentation + 4)}RETURN
{ind(indentation + 3)}RETURN"""
	s += f"""
{ind(indentation+1)}defoutput"""

	s += f"""
{ind(indentation+1)}DEF input
{ind(indentation+2)}in = \\INPUT"""
	for i in range(CELL_LIMIT):
		s+=f"""
{ind(indentation + 2)}IF in == {sanitize(chr(i))}
{ind(indentation + 3)}cell{cellnum} = {i}
{ind(indentation + 3)}truthy = {int(bool(i))}
{ind(indentation + 3)}DEF output
{ind(indentation + 4)}OUTPUT = {sanitize(chr(i))}
{ind(indentation + 4)}RETURN
{ind(indentation + 3)}RETURN"""
	return s

def generate_boilerplate():
	# Cell pointer & truthy variable init
	s = """cell = 0
truthy = 0"""
	# Init all cells
	for i in range(TAPE_LENGTH):
		s += f"""
cell{i} = 0"""
	# Define moving the pointer right
	s += """
DEF right"""
	for i in range(TAPE_LENGTH):
		s+=f"""
	IF cell == {i}
		cell = {(i+1)%TAPE_LENGTH}
		defcell
		RETURN"""
	# Define moving the pointer left
	s += f"""
DEF left"""
	for i in range(TAPE_LENGTH):
		s+=f"""
	IF cell == {(TAPE_LENGTH-1)-i}
		cell = {((TAPE_LENGTH-2)-i)%TAPE_LENGTH}
		defcell
		RETURN"""
	# Define cell increment/decrement/input/output definer
	s+="""
DEF defcell"""
	for i in range(TAPE_LENGTH):
		s+="\n"+generate_cell(i, 1)
	# Call the definer on the first runthrough for initialization purposes
	# Also define the initial output function
	s+="""
\tRETURN
defcell"""
	return s

def generate_code(bfcode : str):
	s = generate_boilerplate()
	indentation = 0
	loop_stack = []
	latest_loop = -1
	for opcode in bfcode:
		if opcode == "+":
			s += f"\n{ind(indentation)}increment"
		elif opcode == "-":
			s += f"\n{ind(indentation)}decrement"
		elif opcode == "<":
			s += f"\n{ind(indentation)}left"
		elif opcode == ">":
			s += f"\n{ind(indentation)}right"
		elif opcode == ",":
			s += f"\n{ind(indentation)}input"
		elif opcode == ".":
			s += f"\n{ind(indentation)}output"
		elif opcode == "[":
			latest_loop += 1
			loop_stack.append(latest_loop)
			s += f"\n{ind(indentation)}DEF loop{latest_loop}"
			indentation += 1
		elif opcode == "]":
			loop = loop_stack.pop()
			indentation -= 1
			s += f"""
{ind(indentation)}IF truthy == 1
{ind(indentation+1)}loop{loop}""" # I am aware this leaks memory on the call stack but I feel like RETURNing might break out of any parent loops too so I'll keep this for now
		# if opcode in "+-<>.,[]": s += f"\n{ind(indentation)}debug"
	return s

with open("generated_output.ifset", "w") as file:
	file.write(
		generate_code("""
+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.
""")
	)
