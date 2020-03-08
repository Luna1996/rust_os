//╔═══════════╗
//║ 	Init  	║
//╚═══════════╝
pub unsafe fn init() {
  let cout = (*crate::ST).ConOut;
  let max_mode = (cout.Mode.MaxMode - 1) as usize;
  (cout.ClearScreen)(cout);
  (cout.SetMode)(cout, max_mode);
}
