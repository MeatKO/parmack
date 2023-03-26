#[derive(Debug)]
pub enum WindowEvent
{
	KeyPress(KeyCode),
	KeyRelease(KeyCode),
	MousePress(MouseCode),
	MouseRelease(MouseCode),
	WindowAction(WindowActions)
}

#[derive(Debug)]
pub enum WindowActions
{
	Expose,
	Leave,
	Close,
	Minimize,
	Maximize,
	Resize,
	FocusOut,
	FocusIn,
	Motion(i32, i32),
	Configure(i32, i32),
}

#[derive(Debug)]
#[repr(u32)]
pub enum MouseCode
{
	Left,
	Right,
	Middle,
}

#[derive(Debug)]
#[repr(u32)]
pub enum KeyCode
{
	ESC = 9,
	W = 25,
	A = 38,
	S = 39,
	D = 40,
	Unknown = 255, // placeholder default value
}