import xiv_craft_solver
import rich

rich.inspect(xiv_craft_solver)
rich.inspect(xiv_craft_solver.test_result)
rich.inspect(xiv_craft_solver.test_result())
rich.inspect(xiv_craft_solver.test_result())


class feedMeDaddy:
	durability = 70
	progress = 3900
	quality = 10920
	progress_divider = 130
	quality_divider = 115
	progress_modifier = 80
	quality_modifier = 70
	craftsmanship = 4041
	control = 3959
	max_cp = 602

	# Config
	depth = 10
	byregot_step = 10
	desperate = False
	threads = 8
	verbose = 0
	

el_daddy=feedMeDaddy()
rich.inspect(xiv_craft_solver.solve_from_python(el_daddy))

