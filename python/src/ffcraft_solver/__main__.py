import dearpygui.dearpygui as dpg
import xiv_craft_solver as xcs
from ffcraft_solver.modules import loader, log, solver
import rich

dpg.create_context()
dpg.configure_app(manual_callback_management=True)
dpg.create_viewport(title='XIV Solver', width=500, height=700)


class User:
    name = '   '
    food = ''
    pot = ''
    specialist = False
    initial_craftsmanship = 0
    initial_control = 0
    initial_cp = 0
    craftsmanship = 0
    control = 0
    cp = 0

    def refresh_gui(self) -> True:
        self.compute_stats()
        dpg.set_value(user_combo, self.name)
        dpg.set_value(food_combo, self.food)
        dpg.set_value(pot_combo, self.pot)
        dpg.set_value(user_effective_stats, [self.craftsmanship, self.control, self.cp])
        if self.name != '   ':
            dpg.set_value(save_user_as, self.name)
        else:
            dpg.set_value(save_user_as, '')
        return True

    def compute_stats(self) -> list:
        food = []
        try:
            full_food = full_config.get_foods_dict()[self.food]
            if user.initial_craftsmanship * full_food[0][0] / 100 > full_food[0][1]:
                food.append(full_food[0][1])
            else:
                food.append(round(user.initial_craftsmanship * full_food[0][0] / 100))
            if user.initial_control * full_food[1][0] / 100 > full_food[1][1]:
                food.append(full_food[1][1])
            else:
                food.append(round(user.initial_control * full_food[1][0] / 100))
            if user.initial_cp * full_food[2][0] / 100 > full_food[2][1]:
                food.append(full_food[2][1])
            else:
                food.append(round(user.initial_cp * full_food[2][0] / 100))
        except KeyError as _:
            food = [0, 0, 0]
            pass

        pot = []
        try:
            full_pot = full_config.get_pots_dict()[self.pot]
            if user.initial_craftsmanship * full_pot[0][0] / 100 > full_pot[0][1]:
                pot.append(full_pot[0][1])
            else:
                pot.append(round(user.initial_craftsmanship * full_pot[0][0] / 100))
            if user.initial_control * full_pot[1][0] / 100 > full_pot[1][1]:
                pot.append(full_pot[1][1])
            else:
                pot.append(round(user.initial_control * full_pot[1][0] / 100))
            if user.initial_cp * full_pot[2][0] / 100 > full_pot[2][1]:
                pot.append(full_pot[2][1])
            else:
                pot.append(round(user.initial_cp * full_pot[2][0] / 100))
        except KeyError as _:
            pot = [0, 0, 0]
            pass

        if user.specialist:
            specialist = [20, 20, 15]
        else:
            specialist = [0, 0, 0]

        self.craftsmanship = self.initial_craftsmanship + food[0] + pot[0] + specialist[0]
        self.control = self.initial_control + food[1] + pot[1] + specialist[1]
        self.cp = self.initial_cp + food[2] + pot[2] + specialist[2]

        return [self.craftsmanship, self.control, self.cp]

    def set_initial_stats(self, _, app_data, __) -> list:
        self.initial_craftsmanship = app_data[0]
        self.initial_control = app_data[1]
        self.initial_cp = app_data[2]
        self.refresh_gui()
        return [self.initial_craftsmanship, self.initial_control, self.initial_cp]

    def set_specialist(self, _, app_data, __) -> bool:
        if app_data:
            self.specialist = True
        else:
            self.specialist = False
        self.refresh_gui()
        return self.specialist

    def set_food(self, _, app_data, __) -> str:
        self.food = app_data
        self.refresh_gui()
        return self.food

    def set_pot(self, _, app_data, __) -> str:
        self.pot = app_data
        self.refresh_gui()
        return self.food

    def set_name_combo(self, _, app_data, __) -> str:
        self.name = app_data
        self.set_initial_stats(0, full_config.get_users_dict()[self.name], None)
        dpg.set_value(user_stats, [self.initial_craftsmanship, self.initial_control, self.initial_cp])
        return self.name


class Recipe:
    name = '   '
    progress = 0
    quality = 0
    durability = 0
    quality_difficulty = 0
    progress_difficulty = 0
    quality_extra_difficulty = 0
    progress_extra_difficulty = 0

    def refresh_gui(self):
        dpg.set_value(recipe_stats, self.get_recipe_stats()[0:3])
        dpg.set_value(advanced_recipe_stats, self.get_recipe_stats()[3:7])
        if self.name != '   ':
            dpg.set_value(save_recipe_as, self.name)
        else:
            dpg.set_value(save_recipe_as, '')

    def set_recipe_stats(self, tag, app_data, _) -> list[int, int, int, int, int, int, int]:
        stats = [0, 0, 0]
        advanced_stats = [0, 0, 0, 0]

        if tag == 'recipe_tooltip':
            stats = app_data
            advanced_stats = dpg.get_value(advanced_recipe_stats)
        elif tag == 'advanced_recipe_tooltip':
            stats = dpg.get_value(recipe_stats)
            advanced_stats = app_data
        elif tag == 0:
            stats = [app_data[0], app_data[1], app_data[2]]
            advanced_stats = [app_data[3], app_data[4], app_data[5], app_data[6]]
        else:
            print('Log: Something went very wrong in recipe stat assignment.')

        self.progress = stats[0]
        self.quality = stats[1]
        self.durability = stats[2]
        self.progress_difficulty = advanced_stats[0]
        self.quality_difficulty = advanced_stats[1]
        self.progress_extra_difficulty = advanced_stats[2]
        self.quality_extra_difficulty = advanced_stats[3]

        return [self.progress, self.quality, self.durability, self.progress_difficulty, self.quality_difficulty,
                self.progress_extra_difficulty, self.quality_extra_difficulty]

    def get_recipe_stats(self) -> list[int, int, int, int, int, int, int]:
        return [self.progress, self.quality, self.durability, self.progress_difficulty, self.quality_difficulty,
                self.progress_extra_difficulty, self.quality_extra_difficulty]

    def set_recipe_name(self, _, app_data, __):
        self.name = app_data
        self.set_recipe_stats(0, full_config.get_recipes_dict()[self.name], 0)
        self.refresh_gui()
        return self.name

    def get_recipe_name(self) -> str:
        return self.name


def request_solve(self):
    solve = solver.Solver(user, recipe)
    solve.solve()


with dpg.window(label="Settings", width=500, height=700, no_resize=True, no_title_bar=True, no_move=True):
    # Initialization of all our classes
    user = User()
    recipe = Recipe()

    full_config = loader.Loader()
    characters_names = full_config.get_users_names()
    foods_names = full_config.get_foods_names()
    pots_names = full_config.get_pots_names()
    recipe_names = full_config.get_recipes_names()

    loggers = log.Loggers()

    # Start of GUI drawing
    # CHARACTER
    dpg.add_text("Character")
    user_combo = dpg.add_combo(items=characters_names, label="Your character", callback=user.set_name_combo)
    dpg.set_value(user_combo, "   ")
    # TODO: Load 1rst user in users.yaml by default
    user_stats = dpg.add_input_intx(size=3, label="Stats", tag="stats_tooltip", callback=user.set_initial_stats)
    with dpg.tooltip("stats_tooltip"):
        dpg.add_text("Craftsmanship / Control / CP")
    with dpg.group(horizontal=True):
        save_user_as = dpg.add_input_text(hint='Save as...')
        dpg.add_button(label="Save!")
        # TODO: Can't save if value == '   '

    # EFFECTIVE STATS
    dpg.add_separator()
    user_effective_stats = dpg.add_input_intx(size=3, label="Effective stats", tag="effective_stats_tooltip", readonly=True)
    with dpg.tooltip("effective_stats_tooltip"):
        dpg.add_text("Craftsmanship / Control / CP")
    dpg.add_separator()

    # MODIFIERS
    dpg.add_text("Modifiers")
    food_combo = dpg.add_combo(items=foods_names, label="Food", callback=user.set_food)
    pot_combo = dpg.add_combo(items=pots_names, label="Pot", callback=user.set_pot)
    specialist_checkbox = dpg.add_checkbox(label="Specialist", tag="specialist", callback=user.set_specialist)

    # RECIPE
    dpg.add_separator()
    dpg.add_text("Recipe")
    recipe_combo = dpg.add_combo(items=recipe_names, label="Recipe", callback=recipe.set_recipe_name)
    dpg.set_value(recipe_combo, "   ")
    recipe_stats = dpg.add_input_intx(size=3, label="Stats", tag="recipe_tooltip", callback=recipe.set_recipe_stats)
    with dpg.tooltip("recipe_tooltip"):
        dpg.add_text("Progress / Quality / Durability")
    advanced_recipe_stats = dpg.add_input_intx(
        size=4, label="Advanced stats", tag="advanced_recipe_tooltip", callback=recipe.set_recipe_stats
    )
    with dpg.tooltip("advanced_recipe_tooltip"):
        dpg.add_text("Progress / Quality difficulty // Progress / Quality extra difficulty")
    # TODO: add a log if recipe matches a known recipe
    with dpg.group(horizontal=True):
        save_recipe_as = dpg.add_input_text(hint='Save as...')
        dpg.add_button(label="Save!", callback=request_solve)
        # TODO: Can't save if value == '   '

    # SOLVE
    dpg.add_separator()
    dpg.add_button(label="Solve!")
    dpg.add_text()
    dpg.add_collapsing_header(label="Result")

    # LOG
    log_window = dpg.add_input_text(multiline=True, default_value=loggers.log, height=50, width=500,
                                    tab_input=True, pos=[0, 642], tracked=True)

dpg.setup_dearpygui()
dpg.show_viewport()
while dpg.is_dearpygui_running():
    jobs = dpg.get_callback_queue()  # retrieves and clears queue
    dpg.run_callbacks(jobs)
    dpg.render_dearpygui_frame()
dpg.start_dearpygui()
dpg.destroy_context()
