use euler::prelude::Execute;

macro_rules! import_solutions {
    ($($number:expr),+$(,)?) => {
		paste::paste! {
			$(
				pub mod [<p $number>];
			)+

			pub fn get(number: usize) -> Option<Box<dyn Execute>> {
				match number {
					$(
						$number => Some(Box::new([<p $number>]::Problem)),
					)+
					_ => None,
				}
			}

			pub fn loops(number: usize) -> u8 {
				match number {
					$(
						$number => [<p $number>]::LOOPS,
					)+
					_ => 100,
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
