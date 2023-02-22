import rich
import yaml
import os
# from xiv_craft_solver.modules import log


# loggers = log.Loggers()


class DefaultConfig:
    yaml_user = 'configs/users.yaml'
    yaml_consumable = 'configs/consumables.yaml'
    yaml_recipes = 'configs/recipes.yaml'


class Loader:
    # TODO: create folder/file if missing when clicking "save"
    # TODO: log any missing file(s) or the whole folder
    def __init__(self):
        self.user_list = {'   ': [0, 0, 0]}
        self.foods_list = {'   ': [[0, 0], [0, 0], [0, 0]]}
        self.pots_list = {'   ': [[0, 0], [0, 0], [0, 0]]}
        self.recipes_list = {'   ': [0, 0, 0, 0, 0, 0, 0]}
        self.config = DefaultConfig()

        try:
            with open(self.relative_import(self.config.yaml_user), 'r') as file:
                self.user_list.update(yaml.safe_load(file))
        except FileNotFoundError as e:
            print(e)

        try:
            with open(self.relative_import(self.config.yaml_consumable), 'r') as file:
                loaded_file = yaml.safe_load(file)
                self.foods_list.update(loaded_file['Foods'])
                self.pots_list.update(loaded_file['Pots'])
        except FileNotFoundError as e:
            print(e)

        try:
            with open(self.relative_import(self.config.yaml_recipes), 'r') as file:
                self.recipes_list.update(yaml.safe_load(file))
        except (FileNotFoundError, IsADirectoryError) as e:
            print(e)

    @staticmethod
    def relative_import(path):
        return os.path.normpath(os.path.join(__file__, "../..", path))

    def get_users_dict(self) -> dict:
        # loggers + 'Loaded user dictionary from users.yaml'
        return self.user_list

    def get_users_names(self) -> list:
        user_names = []
        for n in self.user_list:
            user_names.append(n)
        return user_names

    def get_foods_dict(self) -> dict:
        return self.foods_list

    def get_foods_names(self) -> list:
        foods_names = []
        for f in self.foods_list:
            foods_names.append(f)
        return foods_names

    def get_pots_dict(self) -> dict:
        return self.pots_list

    def get_pots_names(self) -> list:
        pots_names = []
        for p in self.pots_list:
            pots_names.append(p)
        return pots_names

    def get_recipes_dict(self) -> dict:
        return self.recipes_list

    def get_recipes_names(self) -> list:
        recipes_names = []
        for r in self.recipes_list:
            recipes_names.append(r)
        return recipes_names


if __name__ == "__main__":
    loader = Loader()
