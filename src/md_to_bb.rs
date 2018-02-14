
use regex::Regex;

lazy_static! {
	static ref LINK:   Regex = Regex::new(r#"\[(.*?)\]\((.*?)\)"#).unwrap();
	static ref CODE3:  Regex = Regex::new(r#"```(.*?)```"#       ).unwrap();
	static ref CODE2:  Regex = Regex::new(r#"``(.*?)``"#         ).unwrap();
	static ref CODE1:  Regex = Regex::new(r#"`(.*?)`"#           ).unwrap();
	static ref H2:     Regex = Regex::new(r#"^##(.*)"#           ).unwrap();
	static ref H1:     Regex = Regex::new(r#"^#(.*)"#            ).unwrap();
	static ref BOLD:   Regex = Regex::new(r#"\*\*(.*?)\*\*"#     ).unwrap();
	static ref ITALIC: Regex = Regex::new(r#"\*(.*?)\*"#         ).unwrap();
	static ref STRIKE: Regex = Regex::new(r#"~~(.*?)~~"#         ).unwrap();
}

pub fn convert(s: &str) -> String {
	let s = LINK   .replace(&s, "[url=$2]$1[/url]");
	let s = CODE3  .replace(&s, "[code]$1[/code]");
	let s = CODE2  .replace(&s, "[code]$1[/code]");
	let s = CODE1  .replace(&s, "[code]$1[/code]");
	let s = H2     .replace(&s, "[b]$1[/b]");
	let s = H1     .replace(&s, "[h1]$1[/h1]");
	let s = BOLD   .replace(&s, "[b]$1[/b]");
	let s = ITALIC .replace(&s, "[i]$1[/i]");
	let s = STRIKE .replace(&s, "[strike]$1[/strike]");
	String::from(s)
}

#[test]
fn test() {
	assert_eq!(convert("\
[my url](google.com)
```a```
``a``
`a`
##hello
#goodbye
**test**
*hi*
~~strike~~
"), "\
[url=google.com]my url[/url]
[code]a[/code]
[code]a[/code]
[code]a[/code]
[b]hello[/b]
[h1]goodbye[/h1]
[b]test[/b]
[i]hi[/i]
[strike]strike[/strike]
");
}