import rich
import xiv_craft_solver as xcs
from ffcraft_solver.__main__ import User, Recipe
from ffcraft_solver.modules import log, translator


class Result:
    def __init__(self, result, title: str):
        self.title = title
        self.original = result
        self.quality = result.quality
        self.actions = result.actions
        self.steps = result.steps
        self.remaining_cp = result.cp

        self.all_buffs_list = ['manipulation', 'veneration', 'mastersMend', 'wasteNot', 'wasteNot2', 'greatStrides',
                               'innovation', 'finalAppraisal', 'carefulObservation', 'heartAndSoul']

        self.readable_actions = self.text_wrap(self.actions)

        self.macro1 = ''
        self.macro2 = ''
        self.macro3 = ''
        self.make_macro()

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
        """Transforms a list of words into a string and wraps them at 34 characters with an indent."""
        line = '    '
        saved_lines = []
        justified_text = ''
        for w in text:
            if len(line) < 31:
                line += w + ' '
            else:
                saved_lines.append(line)
                line = '    ' + w + ' '

        if line != saved_lines[-1]:
            saved_lines.append(line)

        for line in saved_lines:
            justified_text += line + '\n'

        return justified_text[:-1]

    def make_macro(self) -> True:
        saved_lines = ['']
        translated_actions = translator.translate_list(self.actions, "en")
        count = 0

        for action in translated_actions:
            line = f'/ac "{action}" <wait.{"2" if self.actions[count] in self.all_buffs_list else "3"}>'
            saved_lines.append(line)
            count += 1
        saved_lines = saved_lines[1:]

        if self.steps != 15 and self.steps != 29 and self.steps != 44:
            saved_lines.append('/echo Craft complete ! <se.3>')
        else:
            self.loggers.log(f'No completion message in macro because rotation is exactly {self.steps} steps.')

        if self.steps > 15:
            saved_lines.insert(14, '/echo Macro 1 complete <se.2>')

        if self.steps > 30:
            saved_lines.insert(30, '/echo Macro 2 complete <se.2>')
            # TODO : test index

        count = 0
        for line in saved_lines:
            count += 1
            if count < 16:
                self.macro1 += line + '\n'
            elif 15 < count < 31:
                self.macro2 += line + '\n'
            elif 30 < count < 46:
                self.macro3 += line + '\n'

        self.macro1 = self.macro1[:-1]
        self.macro2 = self.macro2[:-1]
        self.macro3 = self.macro3[:-1]
        return True


class Solver:
    def __init__(self, user: User, recipe: Recipe, loggers: log.Loggers):
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

        self.loggers = loggers

        self.solutions = []
        self.best_quality = None
        self.least_steps = None
        self.safe_50 = None

        self.solve()
        self.compute_all()

    def solve(self) -> str:
        self.solutions = xcs.solve_from_python(self)
        return xcs.solve_from_python(self)

    def compute_all(self) -> True:
        if self.solutions:
            self.sort_quality()
            self.compute_best_quality()
            self.compute_least_steps()
            self.compute_50percent_quality()

    def sort_quality(self) -> True:
        self.solutions.sort(key=lambda x: x.quality)
        return True

    def compute_best_quality(self) -> Result:
        self.best_quality = Result(self.solutions[-1], 'Best quality')
        return self.best_quality

    def compute_50percent_quality(self) -> Result:
        done = False
        threshold = self.quality * 1.5
        for s in reversed(self.solutions):
            if s.quality > threshold:
                self.safe_50 = Result(s, 'Safe 50')
                done = True

        if not done:
            self.loggers.add_log('Could not find a rotation for a 50% Safe Margin.\n'
                                 '    Defaulting to Best Quality.')
            self.safe_50 = self.best_quality

        # self.safe_50 = Result(self.solutions[round(len(self.solutions)/2)+1], 'Safe 50')  # old version
        return self.safe_50

    def compute_least_steps(self) -> Result:
        least_steps = 100
        output = None
        if self.solutions:
            for e in self.solutions:
                if e.steps < least_steps:
                    least_steps = e.steps
                    output = e
            self.least_steps = Result(output, 'Least steps')

        return self.least_steps


