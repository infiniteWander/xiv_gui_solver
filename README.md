# xiv_craft_solver
A rust program for solving FFXIV craft recipe. **Only normal lv90 recipes with stars and 70/80 dur works.**

## Usage
1. Download the latest release.
2. Unzip the file.
3. Edit the `craft.toml` file.
4. Run the program.
5. Wait for about 1 minute.
6. Copy the output.
7. Import to other simulators.

#### Example
Note: The recipe data can be retrieved from [FFXIV Crafting Optimizer](https://yyyy.games/crafter/#/simulator)
with checking of **Custom Recipe** after selecting the recipe.
![img.png](img.png)
```toml
[default_recipe]
durability = 70
progress = 5060
quality = 10920
progress_divider = 130
quality_divider = 115
progress_modifier = 80
quality_modifier = 70

[recipe_35]
durability = 35
progress = 3696
quality = 8200
progress_divider = 130
quality_divider = 115
progress_modifier = 80
quality_modifier = 70

[default_character]
craftsmanship = 4041
control = 3959
max_cp = 602
```

#### Output
```
Solving...

Quality: 11126/12628
        ["muscleMemory", "manipulation", "veneration", "groundwork2", "basicSynth2", "prudentSynthesis", "prudentSynthesis", "prudentSynthesis", "prudentTouch", "prudentTouch", "manipulation", "innovation", "prudentTouch", "basicTouch", "standardTouch", "advancedTouch", "innovation", "prudentTouch", "basicTouch", "standardTouch", "advancedTouch", "innovation", "trainedFinesse", "trainedFinesse", "greatStrides", "byregotsBlessing", "basicSynth2"]

Press enter to exit...
```

You can copy the array part and import to other simulators.

![img_1.png](img_1.png)