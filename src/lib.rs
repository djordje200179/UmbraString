mod string;
mod prefix;
mod remainder;

pub use string::String;

#[cfg(test)]
mod tests {
	const SHORT_LIT: &str = "hi";
	const MID_LIT: &str = "hello";
	const LONG_LIT: &str = "hello my very dear world";

	#[test]
	fn size() {
		assert_eq!(std::mem::size_of::<super::String>(), 16);
	}

    #[test]
    fn conv_short() {
		assert_eq!(super::String::from(SHORT_LIT).to_string(), SHORT_LIT);
	}

	#[test]
	fn conv_mid() {
		assert_eq!(super::String::from(MID_LIT).to_string(), MID_LIT);
	}

	#[test]
	fn conv_long() {
		assert_eq!(super::String::from(LONG_LIT).to_string(), LONG_LIT);
	}

	#[test]
	fn conv_utf8_mid() {
		assert_eq!(super::String::from("héllo").to_string(), "héllo");
	}

	#[test]
	fn conv_utf8_long() {
		assert_eq!(super::String::from("абвгдђежз").to_string(), "абвгдђежз");
	}

	#[test]
	fn clone_short() {
		let s1 = super::String::from(SHORT_LIT);
		let s2 = s1.clone();
		assert_eq!(s1.to_string(), s2.to_string());
	}

	#[test]
	fn clone_mid() {
		let s1 = super::String::from(MID_LIT);
		let s2 = s1.clone();
		assert_eq!(s1.to_string(), s2.to_string());
	}

	#[test]
	fn clone_long() {
		let s1 = super::String::from(LONG_LIT);
		let s2 = s1.clone();
		assert_eq!(s1.to_string(), s2.to_string());
	}

	#[test]
	fn eq_short() {
		assert_eq!(super::String::from(SHORT_LIT), super::String::from(SHORT_LIT));
	}

	#[test]
	fn neq_short_short() {
		assert_ne!(super::String::from(SHORT_LIT), super::String::from("abz"));
	}

	#[test]
	fn eq_mid() {
		assert_eq!(super::String::from(MID_LIT), super::String::from(MID_LIT));
	}

	#[test]
	fn neq_mid_short() {
		assert_ne!(super::String::from(MID_LIT), super::String::from("abz"));
	}

	#[test]
	fn eq_long() {
		assert_eq!(super::String::from(LONG_LIT), super::String::from(LONG_LIT));
	}

	#[test]
	fn neq_long_short() {
		assert_ne!(super::String::from(LONG_LIT), super::String::from("abz"));
	}

	#[test]
	fn cmp_short() {
		assert!(super::String::from("a") < super::String::from("b"));
	}

	#[test]
	fn cmp_mid() {
		assert!(super::String::from("aaaaaa") < super::String::from("aaaaab"));
	}

	#[test]
	fn cmp_long() {
		assert!(super::String::from("aaaaaaaaaaaaaaa") < super::String::from("aaaaaaaaaaaaaab"));
	}

	#[test]
	fn cmp_long_short() {
		assert!(super::String::from("aaaaaaaaab") > super::String::from("a"));
	}
}
