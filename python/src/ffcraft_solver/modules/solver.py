import xiv_craft_solver
from __main__ import User, Recipe


class Solver:
    def __init__(self, user: User, recipe: Recipe):
        self.durability = recipe.durability
        self.progress = recipe.progress
        self.quality = recipe.quality
        self.progress_divider = recipe.progress_difficulty
        self.quality_divider = recipe.quality_difficulty
        self.progress_modifier = recipe.extra_progress_difficulty
        self.quality_modifier = recipe.extra_quality_difficulty

        self.craftsmanship = user.craftsmanship
        self.control = user.control
        self.max_cp = user.cp

        self.depth = 10
        self.byregot_step = 10
        self.desperate = False
        self.threads = 8
        self.verbose = 0

    def solve(self) -> str:
        return xiv_craft_solver.solve_from_python(self)
