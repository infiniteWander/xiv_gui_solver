# xiv_craft_solver
A rust program for solving FFXIV craft recipe. **Only normal lv90 recipes with stars works.**

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
[recipe]
durability = 70
progress = 5060
quality = 12628
progress_divider = 130 # as Progress Difficulty
quality_divider = 115 # as Quality Difficulty
progress_modifier = 80 # as Extra Progress Difficulty
quality_modifier = 70 # as Extra Quality Difficulty

[stats] # including food and potion
craftsmanship = 3745
control = 3636
max_cp = 670
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