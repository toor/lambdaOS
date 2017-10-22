macro_rules! make_handler {
  ($num:expr, $name:ident, $body:expr) => {{
    fn body () {
      $body
    }
    #[naked]
    unsafe extern "C" fn $name () {
      asm!(concat!(
        "push esp",                    "\n\t",
        "mov ebp, esp",                "\n\t",
        "pusha",                       "\n\t",

        "call  $0",                    "\n\t",

        "popa",                        "\n\t",
        "leave",                       "\n\t",
        "iretd",                       "\n\t")
           :: "s" (body as fn()) :: "volatile", "intel");
    }
    //Return the created handler as an IdtEntry.
    IdtEntry::new($name, PrivilegeLevel::Ring0, true)
  }};
  //What do to if there was an exception (at the moment, just panic.)
  ($num:expr, $name:ident, EX, $title:expr) => {
    make_handler!($num, $name, {
      panic!("Exception {:#04x}: {}", $num, $title)
    })
  };
  //An unknown interrupt. Not much we can do about this.
  ($num:expr, $name:ident) => {
    make_handler!($num, $name, {
      panic!("interrupt with no handler: {:#04x}", $num)
    })
  }
}
