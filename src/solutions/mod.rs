macro_rules! import_solutions {
    ($($number:expr),+$(,)?) => {
		paste::paste! {
			$(
				pub mod [<p $number>];
			)+

			pub fn get(number: usize) -> Option<Box<dyn euler::Execute>> {
				match number {
					$(
						$number => Some(Box::new([<p $number>]::Problem)),
					)+
					_ => None,
				}
			}
		}
	};
}

import_solutions! {
    // start-solutions
    1,

    // end-solutions
}
