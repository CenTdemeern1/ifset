import re
import getch

class Memory(dict):
    def __setitem__(self, key: str, value: str) -> None:
        if key == "OUTPUT":
            print(value, end="")
            return
        return super().__setitem__(key, value)

class IfSet():
    def __init__(self) -> None:
        self.memory = Memory()
        self.functions = {}
        self.stack = []
        self.indentation = 0

    def run(self, script):
        script = script.replace("\r\n", "\n") # For sanity
        linelist = script.split("\n")
        linenumber = 0
        while True:
            if linenumber >= len(linelist):
                break
            line = linelist[linenumber]
            linenumber = self.run_line(line, linenumber)
    
    def sanitize_value(self, v):
        while r"\INPUT" in v:
            i = getch.getch()
            if type(i) == bytes:
                i = i.decode()
            v = v.replace(r"\INPUT", i, 1)
        return v.replace(r"\n", "\n").replace(r"\r", "\r").replace(r"\\", "\\")

    def run_line(self, line, linenumber):
        indentation = len(line) - len(line.lstrip("\t")) # Get amount of tabs at the start of the line
        if indentation > self.indentation: # If the line's indentation level is greater than the current indentation level:
            return linenumber + 1 # Skip any lines greater than the current indentation level.
        if indentation < self.indentation: # If the line's indentation level is less than the current indentation level:
            self.indentation = indentation # Drop the indentation level down to the line's
            #Continue execution
        sline = line.lstrip("\t")
        if match := re.match(r"(.+) = (.+)", sline):
            key, value = match.groups()
            self.memory[key] = self.sanitize_value(value) # Update memory to set the specified variable to the specified value.
        elif match := re.match(r"IF (.+) == (.+)", sline):
            key, value = match.groups()
            if self.memory[key] == self.sanitize_value(value): # If the specified variable holds the specified value:
                self.indentation += 1 # Increase indentation by 1.
        elif match := re.match(r"DEF (.+)", sline):
            functionname = match.groups()[0]
            self.functions[functionname] = (linenumber, self.indentation + 1) # Use the current line number, because after a jump to this line we move one line forward anyway
        elif match := re.match(r"RETURN", sline):
            linenumber, self.indentation = self.stack.pop()
        elif match := re.match(r"LOOP", sline):
            # Returns without popping the stack, so infinite loops don't cause a memory leak.
            # I want the language to be usable, at least somewhat (mainly by having people generate code)
            # So this should help with that. You're welcome, memory.
            if len(self.stack) > 0:
                linenumber, self.indentation = self.stack[-1]
        elif sline != "":
            self.stack.append((linenumber, self.indentation))
            linenumber, self.indentation = self.functions[sline]
        return linenumber + 1

if __name__ == "__main__":
    import sys
    filename = sys.argv[1]
    with open(filename) as file:
        script = file.read()
    
    ifset = IfSet()
    ifset.run(script)
