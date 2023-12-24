import sys
from z3 import Solver, Int, sat

px, py, pz = Int("px"), Int("py"), Int("pz")
vx, vy, vz = Int("vx"), Int("vy"), Int("vz")
solver = Solver()

for i, line in enumerate(sys.stdin):
    pos, vel = ([int(x) for x in part.split(", ")] for part in line.split(" @ "))
    time = Int(f"t{i}")
    solver.add(pos[0] + vel[0] * time == px + vx * time)
    solver.add(pos[1] + vel[1] * time == py + vy * time)
    solver.add(pos[2] + vel[2] * time == pz + vz * time)

assert solver.check() == sat
model = solver.model()

print("Result:", int(model[px].as_string()) + int(model[py].as_string()) + int(model[pz].as_string())) # type: ignore
