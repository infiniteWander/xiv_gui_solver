import dearpygui.dearpygui as dpg
import xiv_craft_solver as xcs
from ffcraft_solver.modules import loader
import rich

dpg.create_context()
dpg.configure_app(manual_callback_management=True)
dpg.create_viewport(title='XIV Solver', width=500, height=700)

class User:
    name = 'NEW'
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
        return True

    def compute_stats(self) -> list:
        food = []
        try:
            full_food = full_config.get_foods_dict()[self.food]
            # [[5, 120], [0, 0], [26, 82]]
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
            # [(0, 0), (0, 0), (6, 21)]
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


class Recipe:
    name = ''
    quality = 0
    difficulty = 0
    durability = 0

    def set_recipe_stats(self, _, app_data, __):
        self.quality = app_data[0]
        self.difficulty = app_data[1]
        self.durability = app_data[2]
        return self.quality, self.difficulty, self.durability

    def get_recipe_stats(self):
        return self.quality, self.difficulty, self.durability

    def set_recipe_name(self, _, app_data, __):
        self.name = app_data
        self.set_recipe_stats(0, full_config.get_recipes_dict()[self.name], 0)
        dpg.set_value(recipe_stats, self.get_recipe_stats())
        return self.name


with dpg.window(label="Settings", width=500, height=700, no_resize=True, no_title_bar=True, no_move=True):
    user = User()
    recipe = Recipe()

    full_config = loader.Loader()
    characters_names = full_config.get_users_names()
    foods_names = full_config.get_foods_names()
    pots_names = full_config.get_pots_names()
    recipe_names = full_config.get_recipes_names()

    dpg.add_text("Character")
    user_combo = dpg.add_combo(items=characters_names, label="Your character")
    dpg.set_value(user_combo, "NEW")
    # TODO: Load 1rst user in users.yaml by default
    # TODO: Make NEW prompt for a name when saving
    user_stats = dpg.add_input_intx(size=3, label="Stats", tag="stats_tooltip", callback=user.set_initial_stats)
    with dpg.tooltip("stats_tooltip"):
        dpg.add_text("Craftsmanship / Control / CP")
    dpg.add_button(label="Save!", indent=282)
    specialist_checkbox = dpg.add_checkbox(label="Specialist", tag="specialist", callback=user.set_specialist)
    dpg.add_text(" ")
    user_effective_stats = dpg.add_input_intx(size=3, label="Effective stats", tag="effective_stats_tooltip", readonly=True)
    with dpg.tooltip("effective_stats_tooltip"):
        dpg.add_text("Craftsmanship / Control / CP")

    dpg.add_separator()

    dpg.add_text("Consumables")
    food_combo = dpg.add_combo(items=foods_names, label="Food", callback=user.set_food)
    pot_combo = dpg.add_combo(items=pots_names, label="Pot", callback=user.set_pot)

    dpg.add_separator()

    dpg.add_text("Recipe")
    recipe_combo = dpg.add_combo(items=recipe_names, label="Recipe", callback=recipe.set_recipe_name)
    dpg.set_value(recipe_combo, "NEW")
    recipe_stats = dpg.add_input_intx(size=3, label="Stats", tag="recipe_tooltip", callback=recipe.set_recipe_stats)
    with dpg.tooltip("recipe_tooltip"):
        dpg.add_text("Quality / Difficulty / Durability")
    # TODO: add a log if recipe matches a known recipe
    dpg.add_button(label="Save!", indent=282)

    dpg.add_button(label="Solve!")
    dpg.add_collapsing_header(label="Result")

    # TODO: add a log space at the bottom

dpg.setup_dearpygui()
dpg.show_viewport()
while dpg.is_dearpygui_running():
    jobs = dpg.get_callback_queue()  # retrieves and clears queue
    dpg.run_callbacks(jobs)
    dpg.render_dearpygui_frame()
dpg.start_dearpygui()
dpg.destroy_context()
