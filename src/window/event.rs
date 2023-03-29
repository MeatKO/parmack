#[derive(Debug)]
pub enum WindowEvent
{
	KeyPress(KeyCode),
	KeyRelease(KeyCode),
	MousePress(MouseCode, u32, u32),
	MouseRelease(MouseCode, u32, u32),
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
	Resize{ width: i32, height: i32 },
	FocusOut,
	FocusIn,
	Motion{ x: i32, y: i32 },
	Configure{ width: i32, height: i32 },
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