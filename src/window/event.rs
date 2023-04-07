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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum KeyCode
{
	ESC,
	W,
	A,
	S,
	D,
	Unknown, // placeholder default value
}