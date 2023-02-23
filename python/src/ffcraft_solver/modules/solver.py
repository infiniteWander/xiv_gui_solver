import rich
import xiv_craft_solver
from ffcraft_solver.__main__ import User, Recipe
from ffcraft_solver.modules import log

loggers = log.Loggers()


class Result:
    def __init__(self, result, title: str):
        self.title = title
        self.original = result
        self.quality = result.quality
        self.actions = result.actions
        self.steps = result.steps
        self.remaining_cp = result.cp
        self.readable_actions = self.text_wrap(self.actions)
        self.readable = f"""/// {self.title}
Rotation:
{self.readable_actions}
Quality: {self.quality}
Steps: {self.steps}
Remaining CP: {self.remaining_cp}"""

    def __str__(self) -> str:
        return self.readable

    @staticmethod  # il est con ou quoi?
    def text_wrap(text: list[str]) -> str:
        """Transforms a list of words into a string and wraps them at 60 characters with an indent."""

        line = '    '
        saved_lines = []
        justified_text = ''
        for w in text:
            if len(line) < 56:
                line += w + ' '
            else:
                saved_lines.append(line)
                line = '    ' + w + ' '

        for line in saved_lines:
            justified_text += line + '\n'

        return justified_text[:-1]


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
        self.compute_all()

    def solve(self) -> str:
        self.solutions = xiv_craft_solver.solve_from_python(self)
        return xiv_craft_solver.solve_from_python(self)

    def compute_all(self) -> True:
        self.compute_best_quality()

    def compute_best_quality(self) -> Result:
        best_quality = 0
        output = 0
        if self.solutions:
            for e in self.solutions:
                if e.quality > best_quality:
                    best_quality = e.quality
                    output = e
            self.best_quality = Result(output, 'Best quality')

        else:
            loggers.add_log('Could not complete craft with current effective stats.')
        return self.best_quality


