

pub enum TextureFormat {
	Static(u16, u16),
	FBlock,
	BiteBlock(u16, u16),
	LongBiteBlock(u16),
	ExtraLongBiteBlock(u16),
	Grass,
	GrassyBrick,
	AnimatedSideways(u16, u16),
	AnimatedOnState(u16, u16, u16),
	AnimatedOnStateInline(u16, u16),
	AnimatedBranching(u16, u16),
	
}