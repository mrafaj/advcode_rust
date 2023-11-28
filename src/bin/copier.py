from os import system

for i in range(1, 26):
    system(f"copy task01a.rs task{i:02}a.rs")
    system(f"copy task01a.rs task{i:02}b.rs")
