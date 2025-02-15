#[macro_export]
macro_rules! cfg_test {
    ($($item:item)*) => {
		$(
			#[cfg(test)]
			$item
		)*
    };
}
