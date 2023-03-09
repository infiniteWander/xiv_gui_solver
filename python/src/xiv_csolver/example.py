import xiv_csolver_lib
import rich

rich.inspect(xiv_csolver_lib)
rich.inspect(xiv_csolver_lib.test_result())
rich.inspect(xiv_csolver_lib.test_info())
rich.inspect(xiv_csolver_lib.test_final_results())


class FeedMeDaddy:
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
	

el_daddy = FeedMeDaddy()
rich.inspect(xiv_csolver_lib.solve_from_python(el_daddy))

