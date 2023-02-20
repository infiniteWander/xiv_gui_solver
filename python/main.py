import dearpygui.dearpygui as dpg
import xiv_craft_solver as xcs

dpg.create_context()
dpg.configure_app(manual_callback_management=True)
dpg.create_viewport(title='XIV Solver', width=500, height=900)


class USER:
    craftsmanship = 0
    control = 0


class RECIPE:
    durability = 70


def button_callback(sender, app_data, user_data):
    print(f"sender is: {sender}")
    print(f"app_data is: {app_data}")
    print(f"user_data is: {user_data}")


def specialist(_, app_data, c_s):
    print(c_s)
    print(app_data)
    if dpg.get_value(specialist_checkbox):
        dpg.set_value(stats, [c_s[0]+1, c_s[1]+2, c_s[2]+3, 0])
    else:
        dpg.set_value(stats, [c_s[0], c_s[1], c_s[2], 0])


with dpg.window(label="Settings", width=500, height=900):
    dpg.add_text("Character")
    dpg.add_combo(items=("NEW", "AAAAA", "BBBBB"), label="Your character")
    # Make NEW prompt for a name when saving
    stats = dpg.add_input_intx(size=3, label="Stats", tag="stats_tooltip")
    with dpg.tooltip("stats_tooltip"):
        dpg.add_text("Craftsmanship / Control / CP")

    current_stats = dpg.get_value(stats)
    specialist_checkbox = dpg.add_checkbox(label="Specialist", callback=specialist, tag="specialist")
    dpg.add_button(label="Save!")
    dpg.add_text("Recipe")
    dpg.add_combo(items=("AAAAA", "BBBBB"), label="Food")
    dpg.add_combo(items=("AAAAA", "BBBBB"), label="Pots")
    dpg.add_combo(items=("NEW", "AAAAA", "BBBBB"), label="Recipe")
    test = dpg.add_input_intx(size=3, label="Stats", tag="recipe_tooltip")
    with dpg.tooltip("recipe_tooltip"):
        dpg.add_text("Quality / Progress / Durability")
    dpg.add_button(label="Solve!")
    dpg.add_collapsing_header(label="Result")

dpg.setup_dearpygui()
dpg.show_viewport()
while dpg.is_dearpygui_running():
    jobs = dpg.get_callback_queue()  # retrieves and clears queue
    dpg.run_callbacks(jobs)
    dpg.render_dearpygui_frame()
dpg.start_dearpygui()
dpg.destroy_context()
