export const prerender = true;
export const ssr = false;

export interface EmojiData {
	// Only some properties, waiting for https://github.com/missive/emoji-mart/pull/789
	id: string;
	name: string;
	colons: string;
	native: string;
}

// export const acceptedThemes = ['light', 'dark', 'auto']
