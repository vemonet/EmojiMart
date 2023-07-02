<script lang="ts">
	import './styles.css';
	import data from '@emoji-mart/data';
	import { Picker } from 'emoji-mart';
	import { clipboard } from '@tauri-apps/api';
	import { appWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/tauri';
	import { getMatches } from '@tauri-apps/api/cli';
	import { onMount, onDestroy } from 'svelte';
	import type { EmojiData } from './+page';

	let theme = 'auto';
	let keep = false;
	let picker: any;
	// const acceptedThemes = ['light', 'dark', 'auto']

	// Add to clipboard and close when clicking an emoji
	const onEmojiSelect = async (emoji: EmojiData) => {
		// clipboard.writeText(emoji.native)
		// appWindow.hide()
		// invoke('trigger_paste', {emoji: emoji.native, keep: false})

		const previous = await clipboard.readText();
		clipboard.writeText(emoji.native);
		appWindow.hide();
		if (previous) await invoke('trigger_paste', { emoji: emoji.native, keep, previous: previous });
		else await invoke('trigger_paste', { emoji: emoji.native, keep: false });
		if (keep && previous) clipboard.writeText(previous);
	};

	// Close when hit <Esc>
	const handleKeypress = (event: any) => {
		if (event.code === 'Escape') {
			appWindow.close();
		}
		// TODO: else if key somewhere from A to Z, bring back focus on the picker search input?
	};

	onMount(async () => {
		let matches = await getMatches();
		if (matches.args['keep'].value?.toString().toLowerCase() === 'true') keep = true;
		if (matches.args['theme'].value) theme = matches.args['theme'].value?.toString().toLowerCase();
		else {
			const sysTheme = await appWindow.theme();
			if (sysTheme) theme = sysTheme;
		}

		picker.append(new Picker({ data, onEmojiSelect, theme, autoFocus: true, dynamicWidth: true }));
		document.addEventListener('keypress', handleKeypress);
	});

	onDestroy(() => {
		document.removeEventListener('keypress', handleKeypress);
	});
</script>

<!-- https://svelte.dev/tutorial/bind-this -->
<section>
	<div bind:this={picker} />
</section>
