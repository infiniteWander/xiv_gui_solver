import rich
import xiv_craft_solver
from ffcraft_solver.__main__ import User, Recipe


class Solver:
    def __init__(self, user: User, recipe: Recipe):
        self.durability = recipe.durability
        self.progress = recipe.progress
        self.quality = recipe.quality
        self.progress_divider = recipe.progress_difficulty
        self.quality_divider = recipe.quality_difficulty
        self.progress_modifier = recipe.progress_extra_difficulty
        self.quality_modifier = recipe.quality_extra_difficulty

        self.craftsmanship = user.craftsmanship
        self.control = user.control
        self.max_cp = user.cp

        self.depth = 10
        self.byregot_step = 10
        self.desperate = False
        self.threads = 8
        self.verbose = 0

        self.solutions = []
        self.best_quality = None

        self.solve()
        self.return_best_quality()
        rich.inspect(self.best_quality)

    def solve(self) -> str:
        self.solutions = xiv_craft_solver.solve_from_python(self)
        return xiv_craft_solver.solve_from_python(self)

    def return_best_quality(self) -> str:
        best_quality = 0
        output = 0
        for e in self.solutions:
            if e.quality > best_quality:
                best_quality = e.quality
                output = e
        self.best_quality = output
        return self.best_quality

