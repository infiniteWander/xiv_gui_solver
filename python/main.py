import dearpygui.dearpygui as dpg
import loader
import rich
import xiv_craft_solver as xcs

dpg.create_context()
dpg.configure_app(manual_callback_management=True)
dpg.create_viewport(title='XIV Solver', width=500, height=700)


class User:
    name = ''
    craftsmanship = 0
    control = 0
    cp = 0
    food = ''
    pot = ''

    def set_user_stats(self, _, app_data, __):
        self.craftsmanship = app_data[0]
        self.control = app_data[1]
        self.cp = app_data[2]
        dpg.set_value(user_stats, user.get_user_stats())
        return self.craftsmanship, self.control, self.cp

    def set_user_name(self, _, app_data, __):
        self.name = app_data
        self.set_user_stats(0, full_config.get_users_dict()[self.name], None)
        dpg.set_value(user_stats, self.get_user_stats())
        return self.name

    def get_user_stats(self):
        return self.craftsmanship, self.control, self.cp

    def set_specialist(self, _, app_data, __):
        if app_data:
            self.craftsmanship += 20
            self.control += 20
            self.cp += 15
            dpg.set_value(user_stats, [self.craftsmanship, self.control, self.cp, 0])
        else:
            self.craftsmanship -= 20
            self.control -= 20
            self.cp -= 15
            dpg.set_value(user_stats, [self.craftsmanship, self.control, self.cp, 0])
        return self.craftsmanship, self.control, self.cp

    def set_food(self, un, app_data, nu):
        # TODO: create pre- and post-enhancements stats to be able to handle multiple stacking buffs
        self.food = app_data
        temp_user_craftsmanship = user.craftsmanship
        temp_user_control = user.control
        temp_user_cp = user.cp

        craftsmanship, control, cp = full_config.get_foods_dict()[app_data]

        if user.craftsmanship * craftsmanship[0] / 100 > craftsmanship[1]:
            temp_user_craftsmanship += craftsmanship[1]
        else:
            temp_user_craftsmanship += round(user.craftsmanship * craftsmanship[0] / 100)

        if user.control * control[0] / 100 > control[1]:
            temp_user_control += control[1]
        else:
            temp_user_control += round(user.control * control[0] / 100)

        if user.cp * cp[0] / 100 > cp[1]:
            temp_user_cp += cp[1]
        else:
            temp_user_cp += round(user.cp * cp[0] / 100)

        dpg.set_value(user_stats, [temp_user_craftsmanship, temp_user_control, temp_user_cp])


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


with dpg.window(label="Settings", width=500, height=700, no_resize=True, no_title_bar=True):
    user = User()
    recipe = Recipe()

    full_config = loader.Loader()
    characters_names = full_config.get_users_names()
    foods_names = full_config.get_foods_names()
    pots_names = full_config.get_pots_names()
    recipe_names = full_config.get_recipes_names()

    dpg.add_text("Character")
    user_combo = dpg.add_combo(items=characters_names, label="Your character", callback=user.set_user_name)
    dpg.set_value(user_combo, "NEW")
    # TODO: Load 1rst user in users.yaml by default
    # Make NEW prompt for a name when saving
    user_stats = dpg.add_input_intx(size=3, label="Stats", tag="stats_tooltip", callback=user.set_user_stats)
    with dpg.tooltip("stats_tooltip"):
        dpg.add_text("Craftsmanship / Control / CP")
    specialist_checkbox = dpg.add_checkbox(label="Specialist", tag="specialist", callback=user.set_specialist)
    dpg.add_button(label="Save!", indent=282)

    dpg.add_separator()

    dpg.add_text("Consumables")
    food_combo = dpg.add_combo(items=foods_names, label="Food", callback=user.set_food)
    pot_combo = dpg.add_combo(items=pots_names, label="Pot")

    dpg.add_separator()

    dpg.add_text("Recipe")
    recipe_combo = dpg.add_combo(items=recipe_names, label="Recipe", callback=recipe.set_recipe_name)
    dpg.set_value(recipe_combo, "NEW")
    recipe_stats = dpg.add_input_intx(size=3, label="Stats", tag="recipe_tooltip", callback=recipe.set_recipe_stats)
    with dpg.tooltip("recipe_tooltip"):
        dpg.add_text("Quality / Progress / Durability")
    # TODO: add a small grey text message if recipe matches a known recipe
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
