from xiv_csolver.modules import loader


class Printer:
    def __init__(self, full_config: loader.Loader):
        self.full_config = full_config
        self.help_text_dict = {
            'users': '''# Name: [Craftsmanship, control, cp]
# You are encouraged to use this either for multiple characters if you have them, or different jobs having
# different gear or melds.
# The top line will always be selected by default         
''',
            'foods': '''Foods:
  # Name: [[Bonus craftsmanship%, max value], [bonus control%, max value], [bonus cp%, max value]]
  # For example, Calamari Ripieni HQ gives 5% craftsmanship up to a maximum of 120, and 26% CP up to a
  # maximum of 82, and is hence noted as:
  # Calamari Ripieni HQ: [[5, 120], [0, 0], [26, 82]]
  #
  # Here is your template to copy/paste and fill in:
  # Name: [[0, 0], [0, 0], [0, 0]]''',
            'pots': '''  # Same as above:
  # Name: [[Bonus craftsmanship%, max value], [bonus control%, max value], [bonus cp%, max value]]
  # For example, Cunning Craftsman's Draught HQ gives 6% CP up to a maximum of 21, and is hence noted as:
  # Cunning Craftsman's Draught HQ: [[0, 0], [0, 0], [6, 21]]
  #
  # Here is your template to copy/paste and fill in:
  # Name: [[0, 0], [0, 0], [0, 0]]''',
            'recipe': '''# Name: [Progress, quality, durability, progress difficulty, quality difficulty, extra progress difficulty, extra quality difficulty]
# as can be found on other solver websites, such as https://yyyy.games/crafter/index.html#/simulator and checking custom recipe.
# The language option is the second menu to the left from the top-right hand corner.
# If this seems super convoluted, and you'd rather it was just the three in-game values, know that me too.
# Unfortunately that is how the craft system works in FFXIV.'''
        }

    def save_users(self, user_name: str = None, user_stats: list = None) -> dict:
        output = self.help_text_dict['users'] + '\n\n'

        if user_name.title() == '':
            for user, stats in self.full_config.get_users_dict().items():
                if user != '   ':
                    output += f'{user}: [{stats[0]}, {stats[1]}, {stats[2]}]\n'

        else:
            self.full_config.get_users_dict()[user_name] = user_stats
            for user, stats in self.full_config.get_users_dict().items():
                if user != '   ':
                    output += f'{user}: [{stats[0]}, {stats[1]}, {stats[2]}]\n'

        self.print_users(output)
        return self.full_config.get_users_dict()

    def print_users(self, output: str) -> True:
        with open(self.full_config.relative_import(self.full_config.config.yaml_user), 'w') as file:
            file.write(output)
        return True

    def save_recipe(self, recipe_name: str = None, recipe_basic_stats: list = None, recipe_advanced_stats: list = None) -> dict:
        output = self.help_text_dict['recipe'] + '\n\n'

        if recipe_name.title() == '':
            for recipe, stats in self.full_config.get_recipes_dict().items():
                if recipe != '   ':
                    output += f'{recipe}: [{stats[0]}, {stats[1]}, {stats[2]}, {stats[3]}, {stats[4]}, {stats[5]}, {stats[6]}]\n'

        else:
            self.full_config.get_recipes_dict()[recipe_name] = recipe_basic_stats[:-1] + recipe_advanced_stats
            for recipe, stats in self.full_config.get_recipes_dict().items():
                if recipe != '   ':
                    output += f'{recipe}: [{stats[0]}, {stats[1]}, {stats[2]}, {stats[3]}, {stats[4]}, {stats[5]}, {stats[6]}]\n'

        self.print_recipe(output)
        return self.full_config.get_recipes_dict()

    def print_recipe(self, output) -> True:
        with open(self.full_config.relative_import(self.full_config.config.yaml_recipes), 'w') as file:
            file.write(output)
        return True
